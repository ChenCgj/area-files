#include <fcntl.h>
#include "util.h"

bool setNoBlock(int fd) {
    int flag = fcntl(fd, F_GETFL);
    if (fcntl(fd, F_SETFL, flag | O_NONBLOCK) != 0) {
        return false;
    }
    return true;
}
