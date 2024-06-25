#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int sum_cube(uint32_t a, uint16_t d, uint8_t g, uint8_t h, uint8_t i, uint16_t e, uint16_t f, uint32_t b, uint32_t c);

int main(int argc, char **argv)
{
    uint32_t a, b, c;
    uint16_t d, e, f;
    uint8_t  g, h, i;
    a = argc > 1 ? atoi(argv[1]) : 1;
    b = argc > 2 ? atoi(argv[2]) : 2;
    c = argc > 3 ? atoi(argv[3]) : 3;
    d = argc > 4 ? atoi(argv[4]) : 4;
    e = argc > 5 ? atoi(argv[5]) : 5;
    f = argc > 6 ? atoi(argv[6]) : 6;
    g = argc > 7 ? atoi(argv[7]) : 7;
    h = argc > 8 ? atoi(argv[8]) : 8;
    i = argc > 9 ? atoi(argv[9]) : 9;
    printf("Stack args %d %d %d %d %d %d %d %d %d\nResult: %d\n", a, b, c, d, e, f, g, h, i, sum_cube(a, b, c, d, e, f, g, h, i));
    return 0;
}