#include <stdlib.h>
#include "stack_int.h"

bool stack_init(stack_int_t *stack, int size) {
	if (!stack) {
		return false;
	}
	struct Stack *pstack = malloc(sizeof(struct Stack) + size * sizeof(Item));
	if (!pstack) {
		return false;
	}
	pstack->size = size;
	pstack->pos = -1;
	*stack = (stack_int_t)pstack;
	return true;
}

bool stack_push(stack_int_t stack, Item *pitem) {
	if (!pitem) {
		return false;
	}
	struct Stack *pstack = (struct Stack *)stack;
	if (pstack->pos == pstack->size - 1) {
		return false;
	}
	pstack->data[++pstack->pos] = *pitem;
	return true;
}

const Item *stack_top(stack_int_t stack) {
	struct Stack *pstack = (struct Stack *)stack;
	if (pstack->pos == -1) {
		return NULL;
	}
	return &pstack->data[pstack->pos];
}

bool stack_pop(stack_int_t stack) {
	struct Stack *pstack = (struct Stack *)stack;
	if (pstack->pos == -1) {
		return false;
	}
	pstack->pos--;
	return true;
}

void stack_destroy(stack_int_t stack) {
	struct Stack *pstack = (struct Stack *)stack;
	free(pstack);
}

bool stack_full(stack_int_t stack) {
	struct Stack *pstack = (struct Stack *)stack;
	if (pstack->pos == pstack->size - 1) {
		return true;
	}
	return false;
}