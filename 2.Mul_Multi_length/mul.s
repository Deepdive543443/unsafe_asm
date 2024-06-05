            .text
            .global asm_Mul
asm_Mul:
/*  It's okay to use r14 here. r14, a.k.a lr, is the link register that 
    response to the function's call and return.
    r14 could also be use as a general purpose register.    */
            mul r0,r0,r1
            bx r14

            .global asm_Smull
asm_Smull:
/*  SMull stands for signed multiply long, return a 64 bit length data
    It takes two 32 bit register to store a 64 bit long data. The third 
    and forth args of smul stand for multiplying r0 and r1, then store 
    the result to r0(lower 32 bit) and r1(higher 32 bit)    */
            smull r0,r1,r0,r1   // smull  lower 32 bit 2store, higher 32 bit 2store, [2Mul, 2Mul]
            bx lr

            .global asm_Umull
asm_Umull:
/*  Unsigned mul long, similar to the Smull above and it takes unsigned args    */
            umull r0,r1,r0,r1
            bx lr