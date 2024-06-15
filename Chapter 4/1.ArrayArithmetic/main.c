#include <stdio.h>
#include <stdlib.h>

int calc_sum_asm(int *nums, int n);

static int calc_sum(int *nums, int n)
{
    int result = 0;
    for (int i = 0; i < n; i++) {
        result += nums[i];
    }
    return result;
}

int main(int argc, char **argv)
{
    int n = argc - 1;
    int num_list[argc - 1];
    for (int i = 1; i < argc; i++) num_list[i - 1] = atoi(argv[i]);

    printf("CalcSum: %d  CalcSum_Asm: %d\n", calc_sum(num_list, n), calc_sum_asm(num_list, n));
    return 0;
}