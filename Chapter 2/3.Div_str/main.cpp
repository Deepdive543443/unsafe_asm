#include <iostream>
#include "asm_impl.h"

void print_result(int a, int b, int quo, int rem)
{
    std::cout << " a: " << a;
    std::cout << " b: " << b;
    std::cout << " quo: " << quo;
    std::cout << " rem: " << rem << std::endl;
}

int main(int argc, char **argv)
{
    int a, b, quo, rem;
    a = (argc > 1) ? std::stoi(argv[1]) : 1;
    b = (argc > 2) ? std::stoi(argv[2]) : 1;

    quo_rem_asm(&a, &b, &quo, &rem);
    print_result(a, b, quo, rem);
    return 0;
}