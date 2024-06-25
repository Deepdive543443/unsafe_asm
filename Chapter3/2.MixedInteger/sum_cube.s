            .text
            .global sum_cube

sum_cube:   // int sum_cube(uint32_t a, uint16_t d, uint8_t g, uint8_t h, uint8_t i, uint16_t e, uint16_t f, uint32_t b, uint32_t c);
            push    {r4}
            
            mul     r4,r0,r0        // r0  = a * a;         // mul r0,r0,r0 seems lead to some issue here, why it doesn't 
            mul     r0,r4,r0        // r0  = a * a * a;     // happen before

            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            mul     r4,r2,r2
            mla     r0,r4,r2,r0

            mul     r4,r3,r3
            mla     r0,r4,r3,r0

            ldrb    r1,[sp,#4]      // i (Start from 4 because of pushs)
            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            ldrh    r1,[sp,#8]      // e (All value are aligned with 32 bits so +4)
            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            ldrh    r1,[sp,#12]      // f
            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            ldr     r1,[sp,#16]      // b
            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            ldr     r1,[sp,#20]      // c
            mul     r4,r1,r1
            mla     r0,r4,r1,r0

            pop     {r4}
            bx      lr


