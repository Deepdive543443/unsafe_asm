#include <iostream>
#include "asm_impl.h"

template <typename T1, typename T2>
void print_result(const char *msg, T1 a, T1 b, T2 result)
{
    std::cout << msg << std::endl;
    std::cout << "C++: " << a * b << std::endl;
    std::cout << "ASM: " << result << std::endl;
    std::cout << std::endl;
}

int main(int argc, char **argv)
{
    int a, b, ua, ub;
    a = 114;
    b = 514;
    print_result("Mul", a, b, asm_mul(a, b));

    a = 1919;
    b = -810;
    print_result("Mul", a, b, asm_mul(a, b));

    a = 4000;
    b = 10000000;
    print_result("Mul(Overflow)", a, b, asm_mul(a, b));
    print_result("SMull", a, b, smull_asm(a, b));

    ua = ub = 0x80000000;
    print_result("SMull", ua, ub, umull_asm(ua, ub));
    print_result("Umull", ua, ub, umull_asm(ua, ub));
}