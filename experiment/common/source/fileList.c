#include <stddef.h>
#include <string.h>
#include <dirent.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <sys/stat.h>
#include <stdbool.h>
#include "debug.h"
#include "strUtil.h"
#include "fileList.h"

struct FileList *addNode(struct FileList *list, struct FileList *node) {
    if (node == NULL) {
        return NULL;
    }
    node->m_pnext = list;
    return node;
}

struct FileList *removeNode(struct FileList *list, const char *fileName, struct FileList **node) {
    if (list == NULL) {
        return NULL;
    }
    struct FileList dummy;
    dummy.m_fileName = NULL;
    dummy.m_fileSize = dummy.m_time = 0;
    dummy.m_pnext = list;

    struct FileList *iter = list;
    struct FileList *pre = &dummy;
    while (!iter && strcmp(iter->m_fileName, fileName)) {
        pre = iter;
        iter = iter->m_pnext;
    }

    if (iter) {
        pre->m_pnext = iter->m_pnext;
        *node = iter;
    } else {
        *node = NULL;
    }

    return dummy.m_pnext;
}

struct FileList *findNode(struct FileList *list, const char *fileName) {
    struct FileList *iter = list;
    while (!iter && strcmp(iter->m_fileName, fileName)) {
        iter = iter->m_pnext;
    }
    return iter;
}

struct FileList *getFilesInfoHelper(struct FileList *list, char *dirName, int preLen) {
    struct FileList *ret = list;
    DIR *shareDir = opendir(dirName);
    if (!shareDir) {
        ERR_INFO("get files in %s fail: %s", dirName, strerror(errno));
        return ret;
    }
    rewinddir(shareDir);
    struct dirent *d = NULL;
    while ((d = readdir(shareDir))) {
        if (d->d_type == DT_REG) {
            // printf("file: %s/%s\n", dirName, d->d_name);
            char fileName[PATH_MAX];
            sprintf(fileName, "%s/%s", dirName, d->d_name);
            struct stat fileStat;
            if (stat(fileName, &fileStat) < 0) {
                ERR_INFO("can't get the info of %s: %s", fileName, strerror(errno));
                continue;
            }
            struct FileList *newNode = malloc(sizeof(*newNode));
            if (!newNode) {
                ERR_INFO("create new node fail.");
                continue;
            }
            newNode->m_certificate = NULL;
            newNode->m_fileName = copyStr(fileName);
            if (!newNode->m_fileName) {
                ERR_INFO("create new node fail.");
                free(newNode);
            }
            newNode->m_time = fileStat.st_mtime;
            newNode->m_fileSize = fileStat.st_size;
            ret = addNode(ret, newNode);
        } else if (d->d_type == DT_DIR) {
            int nameLen = strlen(d->d_name);
            if (nameLen == 1 || nameLen == 2) {
                if (strcmp(d->d_name, ".") == 0 || strcmp(d->d_name, "..") == 0) {
                    continue;
                }
            }
            dirName[preLen] = '/';
            strcpy(dirName + preLen + 1, d->d_name);
            ret = getFilesInfoHelper(ret, dirName, preLen + nameLen + 1);
            dirName[preLen] = '\0';
        }
    }
    closedir(shareDir);
    return ret;
}

struct FileList *getFilesInfo(const char *dirName) {
    char nameBuf[PATH_MAX];
    strcpy(nameBuf, dirName);
    struct FileList *ret = NULL;
    return getFilesInfoHelper(ret, nameBuf, strlen(dirName));
}


void destroyFilesInfo(struct FileList *list) {
    while (list) {
        struct FileList *n = list->m_pnext;
        free(list->m_fileName);
        free(list->m_certificate);
        free(list);
        list = n;
    }
}

struct FileList *strToNode(const char *str) {
    struct FileList *tmp = (struct FileList *)malloc(sizeof(*tmp));
    if (tmp == NULL) {
        return NULL;
    }
    *tmp = (struct FileList) {0, NULL, NULL, 0, NULL};

    char value[1024];
    char *start = NULL;
    char *end = NULL;

    start = strchr(strstr(str, "\"name\"") + 6, '"') + 1;
    end = strchr(start, '"');
    tmp->m_fileName = (char *)malloc(end - start + 1);
    strncpy(tmp->m_fileName, start, end - start);
    tmp->m_fileName[end - start] = '\0';

    start = strchr(strstr(str, "\"size\"") + 6, '"') + 1;
    end = strchr(start, '"');
    strncpy(value, start, end - start);
    value[end - start] = '\0';
    tmp->m_fileSize = atoll(value);

    start = strchr(strstr(str, "\"time\"") + 6, '"') + 1;
    end = strchr(start, '"');
    strncpy(value, start, end - start);
    value[end - start] = '\0';
    tmp->m_time = atol(value);
    
    start = strchr(strstr(str, "\"certificate\"") + 13, '"') + 1;
    end = strchr(start, '"');
    if (end - start != 0) {
        tmp->m_certificate = (char *)malloc(end - start + 1);
        strncpy(tmp->m_certificate, start, end - start);
        tmp->m_certificate[end - start] = '\0';
    }
    return tmp;
}
