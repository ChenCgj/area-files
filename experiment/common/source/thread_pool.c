#include <stdlib.h>
#include <string.h>
#include "debug.h"
#include "thread_pool.h"

static void *thread_pool_inner_func(void *args);

bool thread_pool_init(thread_pool_t *a_pool, int thread_size, int task_size) {
	if (!a_pool) {
		return false;
	}
	struct Thread_pool *ppool = malloc(sizeof(struct Thread_pool));
	if (!ppool) {
		return false;
	}
	ppool->pool = malloc(task_size * sizeof(struct Thread_data));
	if (!ppool->pool) {
		free(ppool);
		return false;
	}
	ppool->threads_array = malloc(thread_size * sizeof(pthread_t));
	if (!ppool->threads_array) {
		free(ppool->pool);
		free(ppool);
		return false;
	}
	if (!stack_init(&ppool->avaliable_stack, task_size)) {
		free(ppool->pool);
		free(ppool->threads_array);
		free(ppool);
		return false;
	}
	if (!stack_init(&ppool->occupied_stack, task_size)) {
		free(ppool->pool);
		free(ppool->threads_array);
		stack_destroy(ppool->avaliable_stack);
		free(ppool);
		return false;
	}
	if (pthread_mutex_init(&ppool->pool_mutex, NULL) != 0) {
		free(ppool->pool);
		free(ppool->threads_array);
		stack_destroy(ppool->avaliable_stack);
		stack_destroy(ppool->occupied_stack);
		free(ppool);
		ERR_INFO("pthread_mutex_init()");
		return false;
	}
	if (sem_init(&ppool->count, 0, 0) != 0) {
		free(ppool->pool);
		free(ppool->threads_array);
		stack_destroy(ppool->avaliable_stack);
		stack_destroy(ppool->occupied_stack);
		pthread_mutex_destroy(&ppool->pool_mutex);
		free(ppool);
		return false;
	}
	memset(ppool->pool, 0, task_size * sizeof(struct Thread_data));
	ppool->thread_size = thread_size;
	ppool->task_size = task_size;
	ppool->run_flag = true;
	for (int i = 0; i < task_size; i++) {
		stack_push(ppool->avaliable_stack, &i);
	}
	for (int i = 0; i < thread_size; i++) {
		pthread_create(&ppool->threads_array[i], NULL, thread_pool_inner_func, ppool);
	}
	*a_pool = (thread_pool_t)ppool;
	return true;
}

void thread_pool_destroy(thread_pool_t a_pool) {
	struct Thread_pool *ppool = (struct Thread_pool *)a_pool;
	free(ppool->pool);
	free(ppool->threads_array);
	stack_destroy(ppool->avaliable_stack);
	stack_destroy(ppool->occupied_stack);
	if (pthread_mutex_destroy(&ppool->pool_mutex) != 0) {
		ERR_INFO("pthread_mutex_destroy()");
	}
	if (sem_destroy(&ppool->count) != 0) {
		ERR_INFO("sem_destroy()");
	}
	free(ppool);
}

bool thread_pool_add(thread_pool_t a_pool, thread_func_t func, void *args) {
	struct Thread_pool *ppool = (struct Thread_pool *)a_pool;
	int index = -1;
	pthread_mutex_lock(&ppool->pool_mutex);
	const int *p_index = stack_top(ppool->avaliable_stack);
	if (p_index) {
		index = *p_index;
		stack_pop(ppool->avaliable_stack);
		ppool->pool[index].args = args;
		ppool->pool[index].func = func;
		stack_push(ppool->occupied_stack, &index);
		sem_post(&ppool->count);
	}
	pthread_mutex_unlock(&ppool->pool_mutex);
	if (index == -1) {
		return false;
	}
	return true;
}

void thread_pool_wait(thread_pool_t a_pool) {
	struct Thread_pool *ppool = (struct Thread_pool *)a_pool;
	while (ppool->run_flag) {
		pthread_mutex_lock(&ppool->pool_mutex);
		if (stack_full(ppool->avaliable_stack)) {
			ppool->run_flag = false;
		}
		pthread_mutex_unlock(&ppool->pool_mutex);
	}

	for (int i = 0; i < ppool->thread_size; i++) {
		sem_post(&ppool->count);
	}

	for (int i = 0; i < ppool->thread_size; i++) {
		pthread_join(ppool->threads_array[i], NULL);
	}
}

void *thread_pool_inner_func(void *args) {
	struct Thread_pool *ppool = (struct Thread_pool *)args;
	while (ppool->run_flag) {
		int index = -1;
		sem_wait(&ppool->count);
		pthread_mutex_lock(&ppool->pool_mutex);
		const int *p_index = stack_top(ppool->occupied_stack);
		if (p_index) {
			index = *p_index;
			stack_pop(ppool->occupied_stack);
		}
		pthread_mutex_unlock(&ppool->pool_mutex);
		if (index == -1) {
			continue;
		}
		ppool->pool[index].func(ppool->pool[index].args);
		pthread_mutex_lock(&ppool->pool_mutex);
		stack_push(ppool->avaliable_stack, &index);
		pthread_mutex_unlock(&ppool->pool_mutex);
	}
	return NULL;
}
