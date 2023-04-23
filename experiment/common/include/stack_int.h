#ifndef STACK_INT_H
#define STACK_INT_H
#include <stdbool.h>
#include <stdint.h>

typedef int Item;
struct Stack {
	int size;
	int pos;
	Item data[0];
};

typedef uintptr_t stack_int_t;

bool stack_init(stack_int_t *stack, int size);

bool stack_push(stack_int_t stack, Item *pitem);

const Item *stack_top(stack_int_t stack);

bool stack_pop(stack_int_t stack);

void stack_destroy(stack_int_t stack);

bool stack_full(stack_int_t stack);

#endif