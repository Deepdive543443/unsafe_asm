                .text
                .global calc_sum_asm
// int calc_sum_asm(int *nums, int n); (r2: current idx, r3: current addr, r4: current val, r5: current sum, r6: sizeof(int))
calc_sum_asm:   push    {r4-r6}
                mov     r2,#0
                mov     r3,#0
                mov     r4,#0       
                mov     r5,#0       
                mov     r6,#4           // init

loop:           mul     r3,r2,r6
                ldr     r4,[r0,r3]
                add     r5,r4,r5
                
                add     r2,r2,#1
                cmp     r2,r1
                blt     loop            // Update index, check if index <= (n - 1)
                                        // Jump back to loop if 
                mov     r0,r5
                pop     {r4-r6}
                bx      lr
