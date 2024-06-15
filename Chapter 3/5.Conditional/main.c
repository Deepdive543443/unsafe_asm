#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

void ComparedSum(int a, int b, int c, int *results);
void ComparedSumB(int a, int b, int c, int *results);

int main(int argc, char **argv)
{
    int a, b, c, results;
    a = argc > 1 ? atoi(argv[1]) : 1;
    b = argc > 2 ? atoi(argv[2]) : 2;
    c = argc > 3 ? atoi(argv[3]) : 3;
    ComparedSum(a, b, c, &results);
    printf("Input: %d %d %d\nresults: %d\n", a, b, c, results);
    ComparedSumB(a, b, c, &results);
    printf("Input: %d %d %d\nresults: %d\n", a, b, c, results);
    return 0;
}