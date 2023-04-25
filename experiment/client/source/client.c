#include <unistd.h>
#include <sys/epoll.h>
#include <poll.h>
#include <stdbool.h>
#include <pthread.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <errno.h>
#include <assert.h>
#include <ifaddrs.h>
#include <fcntl.h>
#include <sys/uio.h>
#include "strUtil.h"
#include "util.h"
#include "thread_pool.h"
#include "fileList.h"
#include "shareData.h"
#include "downloadInfo.h"
#include "debug.h"

#define PROB_MAX_CLIENT 10

#define MAX_THREAD_COUNT 12
#define MAX_TASK_COUNT 10
#define SERVER_PORT 1111
#define SERVER_IP "127.0.0.1"

#define MAX_IP 5
#define BIND_IP "0.0.0.0"
#define BC_ADDR "255.255.255.255"
#define PORT 11111
#define BACKLOG 10

#define LOCAL_FILE_DIR "shareFiles"
#define LOCAL_FILE_DIR_LEN 11
#define DOWNLOAD_DIR "downloads"

#define UUID_LEN 8
#define NAME_LEN 16

#define MAX_LINE 1024
#define MAX_SHARE_DATA 10

// global -----------------------------------------------------
int epfd = -1;
int listenfd = -1;
int udpfd = -1;

bool quit = false;
thread_pool_t threadPool;

char hostIP[MAX_IP][INET_ADDRSTRLEN];
int ipCount = 0;

char uuid[UUID_LEN + 1] = {"0000001"};
char userName[NAME_LEN + 1] = {"name"};
int quitFd[2];

struct ShareData *datas[MAX_SHARE_DATA] = {};

//--------------------------------------------------------------
static bool initServer(void);
static void quitServer(void);

static void dealRequest(void);
static void dealUDPRequest(int fd);
static void dealTCPRequest(int fd);
static void dealInput(int fd);

static struct FileList *queryFilesLAN(void);

static bool prepareEpoll(void);
static bool prepareTCPServer(void);
static bool prepareUDPServer(void);
static bool prepareIP(void);

static void prepareExit(void);
static bool boardcastAskFilesInfo(void);
static void showPersonalShareFilesInfo(void);
static bool downloadFile(const char *fileName);
static bool registerUser(char *userName, char *password);

static char *generateFileJsonStr(const char *dirName);
static struct ShareData *getShareDataFromJson(const char *dirName);

static void *downloadThread(void *arg);

void *downloadTask(void *arg);

int main() {
    if (!initServer()) {
        ERR_INFO("init server fail.");
        exit(EXIT_FAILURE);
    }
    dealRequest();
    quitServer();
    return 0;
}

bool prepareEpoll() {
    epfd = epoll_create(PROB_MAX_CLIENT);
    if (epfd < 0) {
        ERR_INFO("create epoll description fail: %s", strerror(errno));
        return false;
    }
    return true;
}

bool prepareTCPServer() {
    if ((listenfd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        ERR_INFO("create listen socket fail");
        return false;
    }
    int option = 1;
    if (setsockopt(listenfd, SOL_SOCKET, SO_REUSEADDR, &option, sizeof(option)) < 0) {
        ERR_INFO("set socket option fail: %s", strerror(errno));
        close(listenfd);
        listenfd = -1;
        return false;
    }

    struct sockaddr_in addr;
    bzero(&addr, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(PORT);
    if (inet_pton(AF_INET, BIND_IP, &addr.sin_addr) != 1) {
        ERR_INFO("the ip %s may be incorrect", BIND_IP);
        close(listenfd);
        listenfd = -1;
        return false;
    }
    if (bind(listenfd, (struct sockaddr *) &addr, sizeof(addr)) < 0) {
        ERR_INFO("bind address fail: %s", strerror(errno));
        close(listenfd);
        listenfd = -1;
        return false;
    }

    if (listen(listenfd, BACKLOG) < 0) {
        ERR_INFO("listen on %s:%hd fail", BIND_IP, PORT);
        close(listenfd);
        listenfd = -1;
        return false;
    }

    if (!setNoBlock(listenfd)) {
        ERR_INFO("set listenfd no block fail: %s", strerror(errno));
        close(listenfd);
        listenfd = -1;
        return false;
    }
    return true;
}

bool prepareUDPServer() {
    if ((udpfd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
        ERR_INFO("create udp socket fail");
        return false;
    }
    int option = 1;
    if (setsockopt(udpfd, SOL_SOCKET, SO_BROADCAST, &option, sizeof(option)) < 0) {
        ERR_INFO("set socket option fail: %s", strerror(errno));
        close(udpfd);
        udpfd = -1;
        return false;
    }

    struct sockaddr_in addr;
    bzero(&addr, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(PORT);
    if (inet_pton(AF_INET, BIND_IP, &addr.sin_addr) != 1) {
        ERR_INFO("the ip %s may be incorrect", BIND_IP);
        close(udpfd);
        udpfd = -1;
        return false;
    }
    if (bind(udpfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        ERR_INFO("bind address fail: %s", strerror(errno));
        close(udpfd);
        udpfd = -1;
        return false;
    }

    if (!setNoBlock(udpfd)) {
        ERR_INFO("set udpfd no block fail: %s", strerror(errno));
        close(udpfd);
        udpfd = -1;
        return false;
    }
    return true;
}

bool prepareIP() {
    struct ifaddrs *addrs;
    if (getifaddrs(&addrs) != 0) {
        ERR_INFO("get IP fail.");
        return false;
    }
    int count = 0;
    char iptmp[INET_ADDRSTRLEN];
    for (struct ifaddrs *a = addrs; a; a = a->ifa_next) {
        struct sockaddr *addr = a->ifa_addr;
        if (addr->sa_family != AF_INET) {
            continue;
        }
        if (inet_ntop(AF_INET, &((struct sockaddr_in *)addr)->sin_addr, iptmp, sizeof(iptmp)) == NULL) {
            ERR_INFO("get a ip address fail");
            continue;
        }
        if (strcmp(iptmp, "127.0.0.1") == 0) {
            continue;
        }
        strcpy(hostIP[count++], iptmp);
        // printf("name: %s, ip: %s\n", a->ifa_name, iptmp);
    }
    freeifaddrs(addrs);
    ipCount = count;
    return true;
}

bool initServer() {

    if (socketpair(AF_LOCAL, SOCK_DGRAM, 0, quitFd) < 0) {
        ERR_INFO("create socketpair fail: %s", strerror(errno));
        return false;
    }
    // init threadPool
    if (!thread_pool_init(&threadPool, MAX_THREAD_COUNT, MAX_TASK_COUNT)) {
        ERR_INFO("init thread pool fail");
        return false;
    }

    if (!prepareTCPServer() || !prepareUDPServer()) {
        return false;
    }

    if (!prepareIP()) {
        return false;
    }

    if (!setNoBlock(STDIN_FILENO)) {
        ERR_INFO("set stdin no block fail: %s", strerror(errno));
        return false;
    }

    if (!prepareEpoll()) {
        ERR_INFO("create epoll fail");
        return false;
    }

    struct epoll_event e = {
        .data.ptr = (void *)((long long)quitFd[0]),
        .events = EPOLLIN
    };
    if (epoll_ctl(epfd, EPOLL_CTL_ADD, quitFd[0], &e) < 0) {
        ERR_INFO("add quit fd to epoll fail: %s", strerror(errno));
        return false;
    }
    if (!thread_pool_add(threadPool, downloadThread, NULL)) {
        ERR_INFO("start download thread fail");
        return false;
    }

    return true;
}

void quitServer() {
    thread_pool_wait(threadPool);
    thread_pool_destroy(threadPool);

    if (epfd >= 0) {
        close(epfd);
        epfd = -1;
    }
    if (listenfd >= 0) {
        close(listenfd);
        listenfd = -1;
    }
    if (udpfd >= 0) {
        close(udpfd);
        udpfd = -1;
    }
}

void dealRequest() {
    struct pollfd fds[3] = {
        {.fd = STDIN_FILENO, .events = POLLIN, .revents = 0},
        {.fd = udpfd, .events = POLLIN, .revents = 0},
        {.fd = listenfd, .events = POLLIN, .revents = 0},
    };
    int n = 0;
    printf(">>> ");
    fflush(stdout);
    while (!quit && (n = poll(fds, 3, -1)) > 0) {
        int count = 0;
        for (int i = 0; i < sizeof(fds) / sizeof(struct pollfd); i++) {
            if (fds[i].revents & POLLIN) {
                count++;
            }
            if (fds[i].fd == STDIN_FILENO) {
                dealInput(fds[i].fd);
            } else if (fds[i].fd == udpfd) {
                dealUDPRequest(fds[i].fd);
            } else if (fds[i].fd == listenfd) {
                dealTCPRequest(listenfd);
            }
            fds[i].revents = 0;
            if (count == n) {
                break;
            }
        }
    }
    if (quit) {
        return;
    }
    if (n == 0) {
        if (errno == EINTR) {
            printf("quit with signal\n");
        }
    } else {
        ERR_INFO("poll fail.");
    }
}

void dealTCPRequest(int fd) {
    struct sockaddr_in cliAddr;
    socklen_t len = sizeof(cliAddr);
    bzero(&cliAddr, len);

    int clientfd = accept(fd, (struct sockaddr *)&cliAddr, &len);
    if (clientfd < 0) {
        if (errno == EWOULDBLOCK) {
            return;
        }
        ERR_INFO("get client fail");
        return;
    }

    char buf[MAX_LINE];
    int n = read(clientfd, buf, MAX_LINE - 1);
    if (n == 0) {
        close(clientfd);
        return;
    }
    buf[n < MAX_LINE ? n : MAX_LINE - 1] = '\0';
    // LOG_INFO(">> tcp receive [%s]\n", buf);

    struct DownloadInfo *info = (struct DownloadInfo *)calloc(1, sizeof(*info));
    info->m_fileInfo.m_fileName = copyStr(buf);
    info->m_saveFd = clientfd;
    info->m_receiveFd = open(info->m_fileInfo.m_fileName, O_RDONLY);
    if (info->m_receiveFd < 0) {
        ERR_INFO("open file %s fail", info->m_fileInfo.m_fileName);
        close(clientfd);
        free(info->m_fileInfo.m_fileName);
        free(info);
        return;
    }
    
    struct epoll_event e = {
        .data.ptr = info,
        .events = EPOLLOUT
    };
    if (epoll_ctl(epfd, EPOLL_CTL_ADD, info->m_saveFd, &e) < 0) {
        ERR_INFO("add to epoll fail: %s", strerror(errno));
        close(info->m_receiveFd);
        close(info->m_saveFd);
        free(info->m_fileInfo.m_fileName);
        free(info);
    }
    return;
}

void dealUDPRequest(int fd) {
    char buf[MAX_LINE];
    struct sockaddr_in addr;
    socklen_t len = sizeof(addr);
    bzero(&addr, len);
    int n = recvfrom(fd, buf, sizeof(buf) - 1, 0, (struct sockaddr *)&addr, &len);
    if (n <= 0) {
        if (n == 0 || errno == EWOULDBLOCK) {
            return;
        }
        ERR_INFO("read for udp fail: %s", strerror(errno));
        return;
    }

    char ipStr[INET_ADDRSTRLEN] = {};
    if (inet_ntop(AF_INET, &addr.sin_addr, ipStr, INET_ADDRSTRLEN) == NULL) {
        ERR_INFO("get the ip fail");
    }
    // for (int i = 0; i < ipCount; i++) {
    //     if (strcmp(ipStr, hostIP[i]) == 0) {
    //         return;
    //     }
    // }

    buf[n < MAX_LINE ? n : MAX_LINE - 1] = '\0';
    // LOG_INFO(">> udp receive [%s] from [%s]", buf, ipStr);

    if (strncmp(buf, "get", 4) == 0) {
        char *json = generateFileJsonStr(LOCAL_FILE_DIR);
        int jsonlen = strlen(json);
        if (sendto(fd, json, jsonlen, 0, (struct sockaddr *)&addr, len) != jsonlen) {
            ERR_INFO("send json str fail: %s", strerror(errno));
        }
    } else {
        if (buf[0] == '{') {
            struct ShareData *data = getShareDataFromJson(buf);
            for (int i = 0; i < MAX_SHARE_DATA; i++) {
                if (datas[i] == NULL) {
                    datas[i] = data;
                    break;
                }
            }
        }
    }
}

void dealInput(int fd) {
    char buf[MAX_LINE];
    int n = read(fd, buf, sizeof(buf) - 1);
    if (n < 0) {
        if (errno == EWOULDBLOCK || errno == EINTR) {
            return;
        }
        ERR_INFO("read from stdin fail");
        return;
    }
    if (n == 0) {
        prepareExit();
    }
    buf[n] = '\0';
    if (buf[n - 1] == '\n') {
        buf[n - 1] = '\0';
    }
    if (buf[0] == '\0') {
        printf(">>> ");
        fflush(stdout);
        return;
    }

    if (strcmp(buf, "exit") == 0) {
        prepareExit();
        return;
    } else if (strcmp(buf, "fresh") == 0) {
        for (int i = 0; datas[i]; i++) {
            free(datas[i]->m_userName);
            free(datas[i]->m_uuid);
            destroyFilesInfo(datas[i]->m_files);
            datas[i] = NULL;
        }
        if (!boardcastAskFilesInfo()) {
            printf("send boardcast msg ask for sharing files fail");
        }
    } else if (strcmp(buf, "list-my") == 0) {
        showPersonalShareFilesInfo();
    } else if (strncmp(buf, "download ", 9) == 0 && strlen(buf) > 9) {
        const char *filename = buf + 9;
        if (!downloadFile(filename)) {
            printf("download %s fail\n", filename);
        } else {
            printf("downloading %s...\n", filename);
        }
    } else if (strcmp(buf, "list-all") == 0) {
        queryFilesLAN();
    } else if (strncmp(buf, "register ", 9) == 0) {
        char *userName = buf + 9;
        char *password = strchr(userName, ' ');
        if (password == NULL) {
            ERR_INFO("lost the password");
        } else {
            *password = '\0';
            password += 1;
            registerUser(userName, password);
        }
    } else {
        printf("unknow command\n");
    }
    printf(">>> ");
    fflush(stdout);
}

void prepareExit() {
    quit = true;
    write(quitFd[1], "1", 1);
}

bool boardcastAskFilesInfo() {
    struct sockaddr_in bcAddr;
    bzero(&bcAddr, sizeof(bcAddr));
    bcAddr.sin_family = AF_INET;
    bcAddr.sin_port = htons(PORT);
    if (inet_pton(AF_INET, BC_ADDR, &bcAddr.sin_addr) != 1) {
        ERR_INFO("the ip %s may be incorrect", BC_ADDR);
        return false;
    }
    // why call sendto will make the stdin can be read?
    if (sendto(udpfd, "get", 3, 0, (struct sockaddr *)&bcAddr, sizeof(bcAddr)) != 3) {
        ERR_INFO("send to %s fail: %s", BC_ADDR, strerror(errno));
        return false;
    }
    return true;
}

void showPersonalShareFilesInfo() {
    struct FileList *list = getFilesInfo(LOCAL_FILE_DIR);
    if (list == NULL) {
        printf("not files...\n");
    }
    struct FileList *l = list;
    while (l) {
        const char *t = ctime(&l->m_time);  // end up with '\n'
        printf("name: %s, size: %zd, time: %s", l->m_fileName + LOCAL_FILE_DIR_LEN, l->m_fileSize, t);
        l = l->m_pnext;
    }
    destroyFilesInfo(list);
}

bool downloadFile(const char *fileName) {
    struct DownloadInfo *info = (struct DownloadInfo *)calloc(1, sizeof(*info));
    for (int i = 0; datas[i] && !info->m_fileInfo.m_fileName; i++) {
        struct FileList *l = datas[i]->m_files;
        while (l) {
            if (strcmp(l->m_fileName, fileName) == 0) {
                info->m_fileInfo.m_fileName = copyStr(fileName);
                info->m_fileInfo.m_fileSize = l->m_fileSize;
                info->m_fileInfo.m_certificate = NULL;
                info->m_fileInfo.m_time = l->m_time;
                strcpy(info->m_ip, datas[i]->m_IP[0]);
                info->m_fileInfo.m_pnext = NULL;
                break;
            }
            l = l->m_pnext;
        }
    }
    if (!info->m_fileInfo.m_fileName) {
        return false;
    }
    if (!thread_pool_add(threadPool, downloadTask, info)) {
        return false;
    }
    return true;
}

char *generateFileJsonStr(const char *dirName) {
    struct FileList *list = getFilesInfo(dirName);
    if (list == NULL) {
        return NULL;
    }

    char *jsonStr = malloc(MAX_LINE);
    if (!jsonStr) {
        return false;
    }

    // json
    int currLen = 0;
    currLen += snprintf(jsonStr + currLen, MAX_LINE - currLen,
        "{"
            "\"name\":\"%s\","
            "\"uuid\":\"%s\","
            "\"ip\":[",
        userName, uuid
    );

    char sep[] = "\0\0";
    for (int i = 0; i < ipCount; i++) {
        currLen += snprintf(jsonStr + currLen, MAX_LINE - currLen, "%s\"%s\"", sep, hostIP[i]);
        sep[0] = ',';
    }
    currLen += snprintf(jsonStr + currLen, MAX_LINE - currLen, "],\"files\":[");

    sep[0] = '\0';
    for (struct FileList *l = list; l; l = l->m_pnext) {
        currLen += snprintf(jsonStr + currLen, MAX_LINE - currLen,
            "%s{"
                "\"name\":\"%s\","
                "\"size\":\"%zd\","
                "\"time\":\"%ld\","
                "\"certificate\":\"%s\""
            "}", sep, l->m_fileName, l->m_fileSize, l->m_time, l->m_certificate ? l->m_certificate : ""
        );
        sep[0] = ',';
    }
    currLen += snprintf(jsonStr + currLen, MAX_LINE - currLen, "]}");
    //json

    destroyFilesInfo(list);
    return jsonStr;
}

struct ShareData *getShareDataFromJson(const char *str) {
    struct ShareData *tmp = (struct ShareData *)malloc(sizeof(*tmp));
    if (tmp == NULL) {
        return NULL;
    }
    *tmp = (struct ShareData) {NULL, NULL, {}, NULL};
    char value[MAX_LINE];
    
    char *start = strchr(strstr(str, "\"name\"") + 6, '"') + 1;
    char *end = strchr(start, '"');
    tmp->m_userName = (char *)malloc(NAME_LEN); //bug
    strncpy(tmp->m_userName, start, end - start);
    tmp->m_userName[end - start] = '\0';

    start = strchr(strstr(str, "\"uuid\"") + 6, '"') + 1;
    end = strchr(start, '\"');
    tmp->m_uuid = (char *)malloc(UUID_LEN);
    strncpy(tmp->m_uuid, start, end - start);
    tmp->m_uuid[end - start] = '\0';

    start = strchr(strstr(str, "\"ip\"") + 4, '"') + 1;
    end = strchr(start, '"');
    strncpy(tmp->m_IP[0], start, end - start);

    struct FileList *node = NULL;
    start = strchr(strstr(str, "\"files\"") + 7, '{');
    while (start) {
        end = strchr(start, '}') + 1;
        strncpy(value, start, end - start);
        value[end - start] = '\0';
        node = strToNode(value);
        tmp->m_files = addNode(tmp->m_files, node);
        start = strchr(end, '{');
    }

    return tmp;
}

struct FileList *queryFilesLAN() {
    for (int i = 0; datas[i]; i++) {
        printf("userName: %s, user uuid: %s, ip: %s\n", datas[i]->m_userName, datas[i]->m_uuid, datas[i]->m_IP[0]);
        printf("\t%-30s\t\t%-20s\t\t%s\n", "filename", "size", "time");
        struct FileList *l = datas[i]->m_files;
        while (l) {
            printf("\t%-30s\t\t%-20zd\t\t%s", l->m_fileName, l->m_fileSize, ctime(&l->m_time));
            l = l->m_pnext;
        }
        printf("-------------------------------------------\n");
    }
    return NULL;
}

void *downloadTask(void *arg) {
    struct DownloadInfo *info = (struct DownloadInfo *)arg;
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(PORT),
    };
    if (inet_pton(AF_INET, info->m_ip, &addr.sin_addr) != 1) {
        ERR_INFO("the ip %s may be incorrect", info->m_ip);
        goto downloadFail;
    }
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) {
        ERR_INFO("create socket fail: %s", strerror(errno));
        goto downloadFail;
    }
    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        ERR_INFO("connet fail: %s", strerror(errno));
        close(fd);
        goto downloadFail;
    }
    if (!setNoBlock(fd)) {
        close(fd);
        ERR_INFO("set socket no block fail: %s", strerror(errno));
        goto downloadFail;
    }
    info->m_receiveFd = fd;
    char *saveFileName = (char *)malloc(MAX_LINE);
    if (!saveFileName) {
        ERR_INFO("malloc memory fail");
        goto downloadFail;
    }
    strcpy(saveFileName, DOWNLOAD_DIR);
    strcat(saveFileName, "/");
    strcat(saveFileName, strrchr(info->m_fileInfo.m_fileName, '/') + 1);
    if ((info->m_saveFd = open(saveFileName, O_CREAT | O_EXCL | O_WRONLY)) < 0) {
        ERR_INFO("create file fail: %s", strerror(errno));
        free(saveFileName);
        close(fd);
        goto downloadFail;
    }
    free(saveFileName);
    struct epoll_event e = {
        .data.ptr = info,
        .events = EPOLLIN
    };
    write(info->m_receiveFd, info->m_fileInfo.m_fileName, strlen(info->m_fileInfo.m_fileName));
    printf("%s\n", info->m_fileInfo.m_fileName);
    if (epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &e) < 0) {
        ERR_INFO("add to epoll fail: %s", strerror(errno));
        close(info->m_receiveFd);
        close(info->m_saveFd);
        goto downloadFail;
    }
    return NULL;

downloadFail:
    ERR_INFO("download file %s fail", info->m_fileInfo.m_fileName);
    free(info->m_fileInfo.m_fileName);
    free(arg);
    return NULL;
}

void *downloadThread(void *arg) {
    struct epoll_event event;
    char buf[BUFSIZ];
    int n = 0;
    while (!quit && (n = epoll_wait(epfd, &event, 1, -1)) > 0) {
        if ((unsigned long long)(event.data.ptr) == quitFd[0]) {
            if (read(quitFd[0], buf, 1) == 1) {
                break;
            }
        }
        struct DownloadInfo *info = (struct DownloadInfo *)event.data.ptr;
        int readn = read(info->m_receiveFd, buf, BUFSIZ);
        if (readn == 0) {
            epoll_ctl(epfd, EPOLL_CTL_DEL, info->m_receiveFd, NULL);
            close(info->m_receiveFd);
            close(info->m_saveFd);
            printf("download %s finish.\n", info->m_fileInfo.m_fileName);
            free(info->m_fileInfo.m_fileName);
            free(info);
        } else if (readn < 0) {
            if (errno != EWOULDBLOCK) {
                ERR_INFO("download file fail: %s", strerror(errno));
                epoll_ctl(epfd, EPOLL_CTL_DEL, info->m_receiveFd, NULL);
                close(info->m_receiveFd);
                close(info->m_saveFd);
                free(info->m_fileInfo.m_fileName);
                free(info);
            }
        } else {
            if (write(info->m_saveFd, buf, readn) != readn) {
                if (errno != EWOULDBLOCK) {
                    ERR_INFO("download file %s fail: %s", info->m_fileInfo.m_fileName, strerror(errno));
                    epoll_ctl(epfd, EPOLL_CTL_DEL, info->m_receiveFd, NULL);
                    close(info->m_receiveFd);
                    close(info->m_saveFd);
                    free(info->m_fileInfo.m_fileName);
                    free(info);
                }
            }
        }
    }
    if (!quit && n < 0) {
        ERR_INFO("epoll wait fail: %s", strerror(errno));
        return NULL;
    }
    return NULL;
}

bool registerUser(char *name, char *password) {
    struct sockaddr_in addr;
    bzero(&addr, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(SERVER_PORT);
    if (inet_pton(AF_INET, SERVER_IP, &addr.sin_addr) != 1) {
        ERR_INFO("the ip %s may be incorrect.", SERVER_IP);
        return false;
    }
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) {
        ERR_INFO("create socket fail");
        return false;
    }
    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        ERR_INFO("connect to the server fail.");
        return false;
    }
    struct iovec data[2] = {
        {.iov_base = name, .iov_len = strlen(name)},
        {.iov_base = password, .iov_len = strlen(password)}
    };
    writev(fd, data, 2);
    char buf[MAX_LINE];
    int n = read(fd, buf, sizeof(buf));
    buf[n] = '\0';
    printf("receive: %s\n", buf);
    close(fd);
    return true;
}
