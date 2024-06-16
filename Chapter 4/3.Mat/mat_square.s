                .text
                .global mat_square_asm
//  void mat_square_asm(int *dst_mat, int *src_mat, int width, int height);[r4=i, r5=j,r6=position,r7=cur_val]
mat_square_asm: push    {r4-r7}
                mov     r6,#0

                mov     r4,#0               // int i = 0
loop_w:         cmp     r4,r2               // i < width
                bge     fin   

                mov     r5,#0               // int j = 0
loop_h:         cmp     r5,r3
                addge   r4,#1               // j >= height
                bge     loop_w

                mul     r6,r2,r4            // r6  = i * width
                add     r6,r6,r5            // r6 += j
                ldr     r7,[r1,r6,lsl #2]   // r7  = src_mat[r6]
                mul     r7,r7,r7            // r7  = r7 ^ 2
                str     r7,[r0,r6,lsl #2]   // dst_mat[r6] = r7

                add     r5,#1               // j++
                blt     loop_h

fin:            pop     {r4-r7}
                bx      lr