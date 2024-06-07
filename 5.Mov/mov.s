            .text
            .global mov_instruct_a
mov_instruct_a:
            // Add valid mov
            mov     r0,#25
            bx      lr
/*  In AArch32, an instruction is 32 bit long. With command and register 
    taking 32 bit, there were only 16 bit left for us to load an integer, 
    which is a short integer. This is why we cannot load integer with 24 
    and 32 bit(like the 508219 and -1000 we show below) */
            .global mov_instruct_b
mov_instruct_b:
            // 508219 : 0x07c13b
            movw    r0,#49467   // r0 = 0x00c13b
            movt    r0,#7       // r0 = 0x07c13b
            bx      lr

            .global mov_instruct_c
mov_instruct_c:
            // -1000 : 0xfffffc18
            movw    r0,#64536   // r0 = 0xfc18
            movt    r0,#65535   // r0 = 0xffff
            bx      lr