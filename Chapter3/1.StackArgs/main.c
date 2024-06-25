#include <stdio.h>
#include <stdlib.h>

int sum_square(int a, int b, int c, int d, int e, int f, int g);

int main(int argc, char **argv)
{
    int a, b, c, d, e, f, g, result;
    a = argc > 1 ? atoi(argv[1]) : 1;
    b = argc > 2 ? atoi(argv[2]) : 2;
    c = argc > 3 ? atoi(argv[3]) : 3;
    d = argc > 4 ? atoi(argv[4]) : 4;
    e = argc > 5 ? atoi(argv[5]) : 5;
    f = argc > 6 ? atoi(argv[6]) : 6;
    g = argc > 7 ? atoi(argv[7]) : 7;

    result = sum_square(a, b, c, d, e, f, g);

    printf("Stack args %d %d %d %d %d %d %d\n", a, b, c, d, e, f, g);
    printf("Result: %d\n", result);
    return 0;
}