#include <stdio.h>

int main(int argc, char **argv)
{
    float a, b, c;
    a = 0.01;
    b = 0.001;
    c = 0.0001;

    printf("%.30f\n%.30f\n", (a * b) * c, a * (b * c));
    return 0;
}