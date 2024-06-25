#include <stdio.h>

int mov_instruct_a();
int mov_instruct_b();
int mov_instruct_c();

int main(int argc, char **argv)
{
    printf("%x %x\n", 56, -57);
    printf("mov_instruct_a: %d\n", mov_instruct_a());
    printf("mov_instruct_b: %d\n", mov_instruct_b());
    printf("mov_instruct_c: %d\n", mov_instruct_c());
    return 0;
}