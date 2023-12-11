#include "utils.c"
#include <stdio.h>

DeclVec(int);

int fib(int n) {
    if (n < 2) return n;
    return fib(n-1) + fib(n-2);
}

int main() {
    Vec(int) vec = vec_init(int);
    for (int i = 0; i < 7; i++) vec_push(int, &vec, i);
    for (size_t i = 0; i < 7; i++) 
        printf("%li: %i\n", i, fib(deref(int, vec_index(int, &vec, i).data)));
    return 0;
}
