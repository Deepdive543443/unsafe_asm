#include <stdio.h>

int asm_int_Add_(int a, int b, int c);

int main(int argc, char **argv)
{
    printf("Result: %d\n", asm_int_Add_(3, 12, 10));
    printf("Result: %d\n", asm_int_Add_(4, 1, 6));
    printf("Result: %d\n", asm_int_Add_(114, 514, 1919));
    return 0;
}