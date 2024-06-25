#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

void local_vars_fp(uint32_t a, uint32_t b, uint32_t c, uint32_t d, uint32_t e, uint32_t *results);

int main(int argc, char **argv)
{
    uint32_t a, b, c, d, e, results;
    a = argc > 1 ? atoi(argv[1]) : 1;
    b = argc > 2 ? atoi(argv[2]) : 2;
    c = argc > 3 ? atoi(argv[3]) : 3;
    d = argc > 4 ? atoi(argv[4]) : 4;
    e = argc > 5 ? atoi(argv[5]) : 5;

    local_vars_fp(a, b, c, d, e, &results);


    printf("%d \n", results);
    return 0;
}