// int asm_int_Add_(int r0, int r1, int r2);

    .text
    .global asm_int_Add_
asm_int_Add_:

    add r0,r0,r1    // r0 = r0 + r1;
    sub r0,r0,r2    // r0 = r0 + r2;
    bx lr           // return