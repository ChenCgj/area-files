#ifndef THREAD_POOL_H
#define THREAD_POOL_H

#include <stdbool.h>
#include <stdint.h>
#include <pthread.h>
#include <semaphore.h>
#include "stack_int.h"

typedef uintptr_t thread_pool_t;
typedef void *(*thread_func_t)(void *);

struct Thread_data {
//	pthread_t thread_no;
	thread_func_t func;
	void *args;
//	bool occupid;
//	int index;
};

struct Thread_pool {
	bool run_flag;
	pthread_mutex_t pool_mutex;
	sem_t count;
	stack_int_t avaliable_stack;
	stack_int_t occupied_stack;
	int thread_size;
	int task_size;
	pthread_t *threads_array;
	struct Thread_data *pool;
};

bool thread_pool_init(thread_pool_t *a_pool, int thread_size, int task_size);

void thread_pool_destroy(thread_pool_t a_pool);

bool thread_pool_add(thread_pool_t a_pool, thread_func_t func, void *args);

void thread_pool_wait(thread_pool_t a_pool);

#endif