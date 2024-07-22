                .text
                .global arr_sum_asm
//  int arr_sum_asm(int *list, int n);[r2=g_max, r3=cur_val, r4=sum, r5=idx]
arr_sum_asm:    push    {r4,r5}
                mov     r4,#0                
                
                cmp     r1,#0
                ble     fin

                ldr     r2,=g_max
                ldr     r2,[r2]             // Load global var from C/C++
                mov     r5,#0 
loop:           ldr     r3,[r0,r5,lsl #2]
                
                cmp     r3,r2
                movgt   r3,r2               // if(r3 > r2) r3 = r2  
                
                add     r4,r4,r3
                add     r5,#1

                cmp     r5,r1
                blt     loop

fin:            mov     r0,r4
                pop     {r4,r5}
                bx      lr
