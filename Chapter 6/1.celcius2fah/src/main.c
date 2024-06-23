#include <stdio.h>
#include <stdlib.h>
#include "c6.h"

#define C2F(c) ((c - 32) * 0.55555556)
#define F2C(c) ((c * 1.8) + 32)

int main(int argc, char **argv)
{
    float celcius    = argc > 1 ? atof(argv[1]) : 114.514;
    float fahrenheit = argc > 2 ? atof(argv[2]) : 1919.810;

    printf("%10.3f %10.3f   C\n", C2F(celcius), F2C(fahrenheit));
    printf("%10.3f %10.3f ASM\n", celcius_2f_asm(celcius), fahrenheit_2c_asm(fahrenheit));

    return 0;
}