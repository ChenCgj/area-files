#ifndef UTIL_H
#define UTIL_H

#include <stdbool.h>

bool setNoBlock(int fd);
bool mkdirIfNotExist(const char *path);
#endif
