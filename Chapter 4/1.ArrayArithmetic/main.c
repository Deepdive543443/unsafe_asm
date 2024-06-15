#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int      calc_sum_asm(int *nums, int n);
uint64_t calc_sum_64_asm(uint32_t *nums, uint32_t n);

static int calc_sum(int *nums, int n)
{
    int result = 0;
    if (n <= 0) return result;
    for (int i = 0; i < n; i++) {
        result += nums[i];
    }
    return result;
}

static uint64_t calc_sum_64(uint32_t *nums, uint32_t n)
{
    uint64_t result = 0;
    if (n <= 0) return result;
    for (int i = 0; i < n; i++) {
        result += (uint64_t)nums[i];
    }
    return result;
}

int main(int argc, char **argv)
{
    int n = argc - 1;
    int num_list[argc - 1];
    for (int i = 1; i < argc; i++) num_list[i - 1] = atoi(argv[i]);

    printf("CalcSum: %d  CalcSum_Asm: %d\n", calc_sum(num_list, n), calc_sum_asm(num_list, n));
    printf("CalcSum_64: %lld  CalcSum_Asm_64: %lld\n", calc_sum_64((uint32_t *)num_list, n), calc_sum_64_asm((uint32_t *)num_list, n));
    return 0;
}