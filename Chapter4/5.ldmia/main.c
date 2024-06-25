#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REVERSE(dsr, src, n) for (int i = 0; i < n; i++) dst[i] = src[n - 1 - i];

void reverse_asm(int *dsr, int *src, int n);

int main(int argc, char **argv)
{
    // Init
    int n = argc > 1 ? atoi(argv[1]) : 5;
    int src[n], dst[n];
    for (int i = 0; i < n; i++) src[i] = i + 1;

    REVERSE(dst, src, n);
    for (int i = 0; i < n; i++) printf("%3d ", src[i]);
    for (int i = 0; i < n; i++) printf("%3d ", dst[i]);
    printf("C implemention\n");

    memset(dst, 0, n * sizeof(int));
    reverse_asm(dst, src, n);
    for (int i = 0; i < n; i++) printf("%3d ", src[i]);
    for (int i = 0; i < n; i++) printf("%3d ", dst[i]);
    printf("ASM implemention\n");
    return 0;
}