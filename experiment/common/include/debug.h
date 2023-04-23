#ifndef DEBUG_H
#define DEBUG_H

#define DEBUG

#if defined(__cplusplus)
#include <cstdio>
#else
#include <stdio.h>
#endif

#define DBG_STRINGIZE_X(x) #x
#define DBG_STRINGIZE(x) DBG_STRINGIZE_X(x)

#define INFO_STR(pre) (__FILE__ ":" DBG_STRINGIZE(__LINE__) ":1:\n\t" pre)

/*
#define INFO(FILE, msg, ...) do {\
    fprintf(FILE, "%s" msg "\n", INFO_STR(""), ##__VA_ARGS__);\
} while (0)
*/

#define INFO(FILE, pre, msg, ...) fprintf(FILE, "%s" msg "\n", INFO_STR(pre), ##__VA_ARGS__)

#define LOG_INFO(msg, ...) INFO(stdout, "", msg, ##__VA_ARGS__)
#ifdef DEBUG
#define DBG_INFO(msg, ...) INFO(stdout, "", msg, ##__VA_ARGS__)
#else
#define DBG_INFO(msg, ...)
#endif
#define ERR_INFO(msg, ...) INFO(stderr, "", msg, ##__VA_ARGS__)

#ifdef __cplusplus
#define THROW_ERR_INFO(EXCEPTION, what, msg, ...) do {\
    ERR_INFO("%s: " msg, what, ##__VA_ARGS__);\
    throw EXCEPTION(what);\
} while(0)
#endif

#endif
