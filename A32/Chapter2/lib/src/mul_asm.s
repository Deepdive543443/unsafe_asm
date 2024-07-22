            .text
            .global asm_mul
//  int asm_mul(int a, int b);
asm_mul:
/*  It's okay to use r14 here. r14, a.k.a lr, is the link register that 
    response to the function's call and return.
    r14 could also be use as a general purpose register.    */
            mul r0,r0,r1
            bx r14

            .global smull_asm
//  long long smull_asm(int a, int b);
smull_asm:
/*  SMull stands for signed multiply long, return a 64 bit length data
    It takes two 32 bit register to store a 64 bit long data. The third 
    and forth args of smul stand for multiplying r0 and r1, then store 
    the result to r0(lower 32 bit) and r1(higher 32 bit)    */
            smull r0,r1,r0,r1   // smull  lower 32 bit 2store, higher 32 bit 2store, [2Mul, 2Mul]
            bx lr

            .global umull_asm
//  unsigned long long umull_asm(unsigned int a, unsigned int b);
umull_asm:
/*  Unsigned mul long, similar to the Smull above and it takes unsigned args    */
            umull r0,r1,r0,r1
            bx lr
