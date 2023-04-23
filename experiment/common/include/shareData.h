#ifndef SHARE_DATA_H
#define SHARE_DATA_H

#include <netinet/in.h>
#include "fileList.h"

struct ShareData {
    char *m_userName;
    char *m_uuid;
    char m_IP[2][INET_ADDRSTRLEN];
    struct FileList *m_files;
};

#endif
