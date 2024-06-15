                .text
                .global ComparedSum
// void ComparedSum(uint32_t a, uint32_t b, uint32_t c, uint32_t *results);
ComparedSum:    add     r0,r0,r1
                add     r0,r0,r2

                cmp     r0,#100
                bge     clamp_max_100   // If r0 >= 100, transfer to "clamp_max_100" 

                str     r0,[r3]
                bx      lr

clamp_max_100:  mov     r0,#100
                str     r0,[r3]

                bx      lr

                .global ComparedSumB
ComparedSumB:   add     r0,r0,r1
                adds    r0,r0,r2        // This instruction also update the NZCV cond flag

                bgt     clamp_min_0     // If r0 > 0,...
                mov     r0,#0
                str     r0,[r3]
                bx      lr

clamp_min_0:    str     r0,[r3]
                bx      lr

