            .equ    ARG_E,0
            .equ    ARG_F,4
            .equ    ARG_G,8     // Assign value to names

            .text
            .global sum_square
sum_square:     // int sum_square(int a, int b, int c, int d, int e, int f, int g);
            mul     r0,r0,r0
            mul     r1,r1,r1
            mul     r2,r2,r2
            mul     r3,r3,r3

            add     r0,r0,r1
            add     r0,r0,r2
            add     r0,r0,r3

            /*  With Volatile register storing the first four argument, the additional
                args will be load into stack. We were accessing these args using Stack 
                pointer(sp).    */
            ldr     r1,[sp,#ARG_E]
            ldr     r2,[sp,#ARG_F]
            ldr     r3,[sp,#ARG_G]

            mul     r1,r1,r1
            mul     r2,r2,r2
            mul     r3,r3,r3

            add     r0,r0,r1
            add     r0,r0,r2
            add     r0,r0,r3

            bx      lr
