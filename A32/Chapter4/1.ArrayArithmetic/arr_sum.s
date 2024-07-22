                .text
                .global calc_sum_asm
/*  Slightly improved loop compare to C306 */
//  int calc_sum_asm(int *nums, int n); [r0: ptr, r1 num, r2: cur_vals, r3: sum]
calc_sum_asm:       mov     r3,#0 
                    cmp     r1,#0
                    ble     Finish          // Early stop

                    mov     r2,r0
loop:               ldr     r2,[r0],#4      // load and add
                    add     r3,r3,r2
                    subs    r1,r1,#1        // check n's val with 0 and update the APSR flags
                    bgt     loop

Finish:             mov     r0,r3
                    bx      lr

                .global calc_sum_64_asm
// uint64_t calc_sum_64_asm(uint32_t *nums, uint32_t n); [r0: ptr, r1 num, r2: cur_vals, r3:sum(lower),r4:sum(higher),r5:idx]
calc_sum_64_asm:    push    {r4,r5}
                    mov     r3,#0
                    mov     r4,#0

                    cmp     r1,#0
                    ble     Finish_64

                    mov     r5,#0
loop_64:            ldr     r2,[r0,r5,lsl #2]   // r2 = *(r0 + (r5 << 2))

                    adds    r3,r2,r3
                    adc     r4,r4,#0            // Adding 64bits

                    add     r5,#1
                    cmp     r5,r1
                    blt     loop_64

Finish_64:          mov     r0,r3
                    mov     r1,r4
                    pop     {r4,r5}             // Forget to add this line will makes CalcSum_64() to output different values
                    bx      lr                  // Wonder what is the reason