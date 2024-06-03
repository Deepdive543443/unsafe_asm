            .text
            .global asm_Mul
asm_Mul:
//=======================================================================
// It's okay to use r14 here. r14, a.k.a lr, is the link register that 
// response to the function's call and return.
// r14 could also be use as a general purpose register.
//=======================================================================
            mul r0,r0,r1
            bx r14

            .global asm_Smull
asm_Smull:
//=======================================================================
// To process 64 bit data, we have lower 32 bit stored in r0 
// and higher 32 bit stored in r1. 
// This also apply to other assembler that handle 64 bit length data.
//=======================================================================
            smull r0,r1,r0,r1
            bx lr

            .global asm_Umull
asm_Umull:
            umull r0,r1,r0,r1
            bx lr