#include "utils.c"
#include <stdio.h>

DeclVec(int);

int fib(int n) {
    if (n < 2) return n;
    return fib(n-1) + fib(n-2);
}

int main() {
    Vec(int) vec = vec_init(int);
    for (int i = 0; i < 7; i++) vec_push(int, &vec, fib(i));
    Slice(int) slice = vec_move_to_slice(int, &vec);
    Option(int) head = slice_next(int, &slice);
	while (head.is_ok) {
        printf("%i\n", head.data);
		head = slice_next(int, &slice);
	}
    return 0;
}
