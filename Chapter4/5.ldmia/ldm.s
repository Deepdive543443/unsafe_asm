                .text
                .global reverse_asm
//  void reverse_asm(int *dsr, int *src, int n);[r3=idx, r4-r7:load, r8-r11: store ip]
reverse_asm:    push    {r4-r11}
                cmp     r2,#0               //
                ble     fin 

                add     r1,r2,lsl #2
                sub     r1,#4               // Moving *src pointer to the end of arr    
                mov     r3,#0               // idx init

ldm_loop:       sub     r2,r3
                cmp     r2,#4
                add     r2,r3               // Restore
                ble     res_loop

                ldmda   r1!,{r4-r7}         // Load 4 from src and move forward by 4
                mov     r11,r4
                mov     r10,r5
                mov     r9,r6
                mov     r8,r7
                stmia   r0!,{r8-r11}        // Reverse and store to dst and move backward by 4

                add     r3,#4               // Update idx
                bal     ldm_loop

res_loop:       cmp     r3,r2
                bge     fin

                ldr     r4,[r1]
                str     r4,[r0]

                add     r0,#4               // dst++
                sub     r1,#4               // src--
                
                add     r3,#1               // Update idx
                bal     res_loop

fin:            pop     {r4-r11}
                bx      lr


