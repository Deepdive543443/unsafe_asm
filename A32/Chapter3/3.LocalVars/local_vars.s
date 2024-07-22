                .text
                .global local_vars
local_vars:     // int local_vars(uint32_t a, uint32_t b, uint32_t c, uint32_t d, uint32_t e);
                push    {r4,r5}
                sub     sp,#16          // Allocate 5 32bits integers //[tmp1, tmp2, tmp3, tmp4, r4, r5, e] 
                                        // Thought this is call "压栈" in Chinese?
                mov     r5,#2

                add     r4,r0,r1
                sdiv    r4,r4,r5
                str     r4,[sp,#0]      // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,r5
                str     r4,[sp,#4]      // Storing temp 2

                add     r4,r2,r3
                sdiv    r4,r4,r5
                str     r4,[sp,#8]      // Storing temp 3

                ldr     r1,[sp,#24]
                add     r4,r3,r1
                sdiv    r4,r4,r5
                str     r4,[sp,#12]     // Storing temp 4

                // Start convolution
                ldr     r0,[sp,#0]
                ldr     r1,[sp,#4]
                ldr     r2,[sp,#8]
                ldr     r3,[sp,#12]

                add     r4,r0,r1
                sdiv    r4,r4,r5
                str     r4,[sp,#0]      // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,r5
                str     r4,[sp,#4]      // Storing temp 2

                add     r4,r2,r3
                sdiv    r4,r4,r5
                str     r4,[sp,#8]      // Storing temp 3

                // convolution 2
                ldr     r0,[sp,#0]
                ldr     r1,[sp,#4]
                ldr     r2,[sp,#8]

                add     r4,r0,r1
                sdiv    r4,r4,r5
                str     r4,[sp,#0]      // Storing temp 1

                add     r4,r1,r2
                sdiv    r4,r4,r5
                str     r4,[sp,#4]      // Storing temp 2

                // convolution 3
                ldr     r0,[sp,#0]
                ldr     r1,[sp,#4]

                add     r0,r0,r1
                sdiv    r0,r0,r5

                // Deallocate temp vars and register
                add     sp,#16
                pop     {r4,r5}

                bx      lr

