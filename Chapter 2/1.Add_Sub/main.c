#include <stdio.h>
#include <stdlib.h>
#include "asm_impl.h"

int main(int argc, char **argv)
{
    int x, add, sub;
    x   = argc > 1 ? atoi(argv[1]) : 114;
    add = argc > 2 ? atoi(argv[2]) : 514;
    sub = argc > 3 ? atoi(argv[3]) : 1919;

    printf("Result C: %d ASM: %d\n", x + add - sub, int_add_asm(x, add, sub));
    return 0;
}