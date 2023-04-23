#include <string.h>
#include <stdlib.h>

#include "strUtil.h"
char *copyStr(const char *str)
{
    int len = strlen(str);
    char *ret = (char *)malloc(sizeof(*ret) * (len + 1));
    if (ret == NULL) {
        return NULL;
    }
    memcpy(ret, str, len + 1);
    return ret;
}
