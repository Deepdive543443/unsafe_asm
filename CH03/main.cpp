#include <climits>
#include <iostream>

extern "C" {
int                asm_Mul(int a, int b);
long long          asm_Smull(int a, int b);
unsigned long long asm_Umull(unsigned int a, unsigned int b);
}

template <typename T1, typename T2>
void print_result(const char *msg, T1 a, T1 b, T2 result)
{
    std::cout << msg << std::endl;
    std::cout << "a: " << a << std::endl;
    std::cout << "b: " << b << std::endl;
    std::cout << "Result: " << result << std::endl;
    std::cout << std::endl;
}

int main(int argc, char **argv)
{
    int          a, b;
    unsigned int ua, ub;

    std::cout << "INT size: " << sizeof(int) << std::endl;
    std::cout << "LONG size: " << sizeof(long) << std::endl;
    std::cout << "LLONG size: " << sizeof(long long) << std::endl;

    std::cout << "INT_MAX: " << INT_MAX << std::endl;
    std::cout << "INT_MIN: " << INT_MIN << std::endl;
    std::cout << "UINT_MAX: " << UINT_MAX << std::endl;
    std::cout << "LLONG_MAX: " << LLONG_MAX << std::endl;
    std::cout << "LLONG_MIN: " << LLONG_MIN << std::endl;
    std::cout << "ULLONG_MAX: " << ULLONG_MAX << std::endl;

    std::cout << std::endl;

    a = 114;
    b = 514;
    print_result("Mul", a, b, asm_Mul(a, b));

    a = 1919;
    b = -810;
    print_result("Mul", a, b, asm_Mul(a, b));

    a = 4000;
    b = 10000000;
    print_result("Mul(Overflow)", a, b, asm_Mul(a, b));
    print_result("SMull", a, b, asm_Smull(a, b));

    ua = ub = 0x80000000;
    print_result("SMull", ua, ub, asm_Smull(ua, ub));
    print_result("Umull", ua, ub, asm_Umull(ua, ub));
}