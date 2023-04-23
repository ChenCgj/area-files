#include <stdio.h>
// #include <mysql/mysql.h>
#include <stdbool.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <errno.h>
#include <poll.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <arpa/inet.h>
#include "util.h"
#include "thread_pool.h"
#include "debug.h"

#define BIND_IP "0.0.0.0"
#define PORT1 1111
#define PORT2 1112

#define BACKLOG 10

#define MAX_THREAD_COUNT 12
#define MAX_TASK_COUNT 100
#define MAX_LINE 1024

// global ---------------------------------
int udpfd = -1;
int listenfd = -1;
thread_pool_t threadPool;

static bool initServer(void);
static void dealRequest(void);
static void quitServer(void);
static void *dealThread(void *arg);
static void dealTCPRequest(int fd);
static void dealUDPRequest(int fd);
static bool prepareTCPServer(void);
static bool prepareUDPServer(void);

int main() {
    if (!initServer()) {
        ERR_INFO("init server fail");
    }
    dealRequest();
    quitServer();
    return 0;
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
    addr.sin_port = htons(PORT1);
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
        ERR_INFO("listen on %s:%hd fail", BIND_IP, PORT1);
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
    addr.sin_port = htons(PORT1);
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


bool initServer() {
    if (!prepareTCPServer() || !prepareUDPServer()) {
        ERR_INFO("prepare tcp/udp server fail");
        return false;
    }

    // init threadPool
    if (!thread_pool_init(&threadPool, MAX_THREAD_COUNT, MAX_TASK_COUNT)) {
        ERR_INFO("init thread poll fail");
    }
    return true;
}

void dealRequest(void) {
    struct pollfd fds[2] = {
        // {.fd = STDIN_FILENO, .events = POLLIN, .revents = 0},
        {.fd = udpfd, .events = POLLIN, .revents = 0},
        {.fd = listenfd, .events = POLLIN, .revents = 0},
    };
    int n = 0;
    while ((n = poll(fds, 2, -1)) > 0) {
        int count = 0;
        for (int i = 0; i < sizeof(fds) / sizeof(struct pollfd); i++) {
            if (fds[i].revents & POLLIN) {
                count++;
            }
            if (fds[i].fd == STDIN_FILENO) {

            } else if (fds[i].fd == listenfd) {
                dealTCPRequest(listenfd);
            } else if (fds[i].fd == udpfd) {
                dealUDPRequest(fds[i].fd);
            }
            fds[i].revents = 0;
            if (count == n) {
                break;
            }
        }
    }
}

void quitServer() {
    thread_pool_wait(threadPool);
    thread_pool_destroy(threadPool);

    if (listenfd >= 0) {
        close(listenfd);
        listenfd = -1;
    }
    if (udpfd >= 0) {
        close(udpfd);
        udpfd = -1;
    }
}

void *dealThread(void *arg) {
    int fd = (int)(unsigned long long)(arg);
    char buf[MAX_LINE];
    int n = read(fd, buf, sizeof(buf));
    if (n < 0) {
        ERR_INFO("read fail: %s", strerror(errno));
        return NULL;
    }
    buf[n] = '\0';
    printf("receive: %s\n", buf);
    send(fd, buf, n, 0);
    close(fd);
    return NULL;
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

    if (!thread_pool_add(threadPool, dealThread, (void *)(unsigned long long)(clientfd))) {
        ERR_INFO("add to thread pool fail");
    }
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

}
