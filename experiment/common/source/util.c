#include <fcntl.h>
#include <sys/stat.h>
#include "util.h"

bool setNoBlock(int fd) {
    int flag = fcntl(fd, F_GETFL);
    if (fcntl(fd, F_SETFL, flag | O_NONBLOCK) != 0) {
        return false;
    }
    return true;
}

bool mkdirIfNotExist(const char *path) {
    struct stat s;
    if (stat(path, &s) == -1 || !S_ISDIR(s.st_mode)) {
        if (mkdir(path, S_IRWXU) == -1) {
            return false;
        }
    }
    return true;
}
