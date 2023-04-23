#ifndef FILE_LIST_H
#define FILE_LIST_H

#include <stddef.h>
#include <time.h>

struct FileList {
    size_t m_fileSize;
    char *m_fileName;
    char *m_certificate;
    time_t m_time;
    struct FileList *m_pnext;
};

struct FileList *addNode(struct FileList *list, struct FileList *node);

struct FileList *removeNode(struct FileList *list, const char *fileName, struct FileList **node);

struct FileList *findNode(struct FileList *list, const char *fileName);

struct FileList *getFilesInfo(const char *dirName);

struct FileList *strToNode(const char *str);

void destroyFilesInfo(struct FileList *list);

#endif
