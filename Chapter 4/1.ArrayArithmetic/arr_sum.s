                .text
                .global calc_sum_asm
/*  Slightly improved loop compare to C306 */
//  int calc_sum_asm(int *nums, int n); [r0: ptr, r1 num, r2: cur_vals, r3: sum]
calc_sum_asm:   mov     r3,#0 
                cmp     r1,#0
                ble     Finish          // Early stop

                mov     r2,r0
loop:           ldr     r2,[r0],#4      // load and add
                add     r3,r3,r2
                subs    r1,r1,#1        // check n's val with 0 and update the APSR flags
                bgt     loop

Finish:         mov     r0,r3
                bx      lr