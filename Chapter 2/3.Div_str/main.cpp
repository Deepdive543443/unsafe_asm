#include <iostream>

extern "C" void asm_quo_rem(const int *a, const int *b, int *quo, int *rem);

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

    asm_quo_rem(&a, &b, &quo, &rem);
    print_result(a, b, quo, rem);

    a = 100;
    b = 7;
    asm_quo_rem(&a, &b, &quo, &rem);
    print_result(a, b, quo, rem);

    return 0;
}