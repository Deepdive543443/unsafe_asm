#include <stdio.h>
#include <stdlib.h>

int MovRegA(unsigned int a, unsigned int *b);
int MovRegB(unsigned int a, unsigned int *b);

void print_byte_as_bits(char val)
{
    for (int i = 7; 0 <= i; i--) {
        printf("%c", (val & (1 << i)) ? '1' : '0');
    }
}

void print_bits(char *ty, unsigned int val, unsigned char *bytes, size_t num_bytes)
{
    printf("(%15s) %16d = 0x%08x [ ", ty, val, val);
    for (size_t i = 0; i < num_bytes; i++) {
        print_byte_as_bits(bytes[num_bytes - 1 - i]);
        printf(" ");
    }
    printf("]\n");
}

#define SHOW_BITS(T, V)                                                  \
    do {                                                                 \
        T x = V;                                                         \
        print_bits(#T, (unsigned int)V, (unsigned char *)&x, sizeof(x)); \
    }                                                                    \
    while (0);

int main(int argc, char **argv)
{
    const size_t n  = 5;
    unsigned int a1 = argc > 1 ? atoi(argv[1]) : 0x12345678;
    printf("Input Decimal  %d\n", a1);

    unsigned int b1[n], b2[n];
    MovRegA(a1, b1);
    MovRegB(a1, b2);

    SHOW_BITS(unsigned int, a1);
    for (size_t i = 0; i < n; i++) {
        SHOW_BITS(unsigned int, b1[i]);
    }

    printf("\n");

    for (size_t i = 0; i < n; i++) {
        SHOW_BITS(unsigned int, b2[i]);
    }

    return 0;
}