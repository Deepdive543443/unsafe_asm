#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

void bit_ops_16(uint16_t a, uint16_t b, uint16_t *result);

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
    uint16_t a = argc > 1 ? atoi(argv[1]) : 114;
    uint16_t b = argc > 2 ? atoi(argv[2]) : 514;
    uint16_t result[3];

    bit_ops_16(a, b, result);

    SHOW_BITS(uint16_t, a);
    SHOW_BITS(uint16_t, b);
    SHOW_BITS(uint16_t, result[0]);
    SHOW_BITS(uint16_t, result[1]);
    SHOW_BITS(uint16_t, result[2]);
    return 0;
}