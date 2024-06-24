            .text
            .global bit_ops_16
bit_ops_16:     //void bit_ops_16(uint16_t a, uint16_t b, uint16_t *result);
            and     r3,r0,r1
            strh    r3,[r2]

            orr     r3,r0,r1
            strh    r3,[r2,#2]

            eor     r3,r0,r1
            strh    r3,[r2,#4]
