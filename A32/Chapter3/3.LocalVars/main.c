#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int local_vars(uint32_t a, uint32_t b, uint32_t c, uint32_t d, uint32_t e);

int main(int argc, char **argv)
{
    uint32_t a, b, c, d, e;
    a = argc > 1 ? atoi(argv[1]) : 1;
    b = argc > 2 ? atoi(argv[2]) : 2;
    c = argc > 3 ? atoi(argv[3]) : 3;
    d = argc > 4 ? atoi(argv[4]) : 4;
    e = argc > 5 ? atoi(argv[5]) : 5;
    printf("%d \n", local_vars(a, b, c, d, e));
    return 0;
}