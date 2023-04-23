#ifndef DOWNLOAD_INFO_H
#define DOWNLOAD_INFO_H
#include "fileList.h"
#include "netinet/in.h"

struct DownloadInfo {
    bool init;
    struct FileList m_fileInfo;
    char m_ip[INET_ADDRSTRLEN];
    int m_receiveFd;
    int m_saveFd;
};

#endif
