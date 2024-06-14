                .text
                .global local_vars_fp
local_vars_fp:  // int local_vars_fp(uint32_t a, uint32_t b, uint32_t c, uint32_t d, uint32_t e);
                push    {r4,fp,lr}      // lr is used as a scratch register here
                mov     fp,sp
                sub     sp,#16          // [temp1, temp2, temp3, temp4, r4 <- fp, fp, lr, e, *result]

                mov     lr,#2

                add     r4,r0,r1
                sdiv    r4,r4,lr
                str     r4,[fp,#-16]    // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,lr
                str     r4,[fp,#-12]    // Storing temp 2

                add     r4,r2,r3
                sdiv    r4,r4,lr
                str     r4,[fp,#-8]     // Storing temp 3

                ldr     r1,[fp,#12]
                add     r4,r3,r1
                sdiv    r4,r4,lr
                str     r4,[fp,#-4]     // Storing temp 4

                // Convolution 1
                ldr     r0,[fp,#-16]
                ldr     r1,[fp,#-12]
                ldr     r2,[fp,#-8]
                ldr     r3,[fp,#-4]

                add     r4,r0,r1
                sdiv    r4,r4,lr
                str     r4,[fp,#-16]    // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,lr
                str     r4,[fp,#-12]    // Storing temp 2

                add     r4,r2,r3
                sdiv    r4,r4,lr
                str     r4,[fp,#-8]     // Storing temp 3

                @ // Convolution 2
                ldr     r0,[fp,#-16]
                ldr     r1,[fp,#-12]
                ldr     r2,[fp,#-8]

                add     r4,r0,r1
                sdiv    r4,r4,lr
                str     r4,[fp,#-16]    // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,lr
                str     r4,[fp,#-12]    // Storing temp 2

                // Convolution 3
                ldr     r0,[fp,#-16]
                ldr     r1,[fp,#-12]

                add     r4,r0,r1
                sdiv    r4,r4,lr

                ldr     r0,[fp,#16]
                str     r4,[r0]

                // Deallocating
                add     sp,#16
                pop     {r4,fp,pc}

