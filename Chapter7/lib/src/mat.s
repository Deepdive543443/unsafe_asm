                .text 
zero_single:    .single 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0

//  float sum_f32(float *src, int length);  // r2=ptr, r3=idx, r4=quo, r5=rem r3Q0(store)(f32, f32, f32, f32) Q1(add)(f32, f32, f32, f32)
                .global sum_f32
sum_f32:        push        {r4,r5}
                cmp         r1,#0
                ble         fin_f32
                
                mov         r2,#4
                sdiv        r4,r1,r2        // quo refer to num of simd ops
                mul         r3,r4,r2
                sub         r5,r1,r3        // rem for num of scalers ops
                
                ldr         r2,=zero_single
                vldm        r2,{q0,q1}      // Init Quad 0 and 1 vector 
                
                mov         r2,r0           // r2 = *src
                mov         r3,#0           // r3 = idx

loop_f32x4:     cmp         r3,r4           // Check if we need to jump to scaler loop
                bge         pre_loop_f32
                
                vldm        r2,{q1}         // Load 4 float32 from mem, perform SIMD adding...
                vadd.f32    q0,q0,q1
                add         r3,#1
                add         r2,#16          // Moving 4 data forward
                bal         loop_f32x4

pre_loop_f32:   mov         r3,#0           // Reset index
                vadd.f32    d0,d0,d1        // Sum the content of Q0
                vadd.f32    s0,s0,s1
loop_f32:       cmp         r3,r5
                bge         fin_f32
                vldr.f32    s1,[r2]
                vadd.f32    s0,s0,s1
                add         r3,#1           // Update idx
                add         r2,#4           // Update pointer
                bal         loop_f32

fin_f32:        pop         {r4,r5}
                bx          lr

//  uint16_t sum_u16(uint16_t *src, int length);
                .global sum_u16
sum_u16:        push        {r4,r5}
                sub         sp,#8
                cmp         r1,#0
                ble         fin_u16
                
                mov         r2,#8           // Most of the part remain identical
                sdiv        r4,r1,r2        // Except we have 8 num in a vector here
                mul         r3,r4,r2
                sub         r5,r1,r3

                ldr         r2,=zero_single
                vldm        r2,{q0,q1}      // Init Quad 0 and 1 vector 

                mov         r2,r0           // r2 = *src
                mov         r3,#0           // r3 = idx

loop_u16x8:     cmp         r3,r4
                bge         pre_loop_u16

                vldm        r2,{q1}         // Load 8 u16 to vec
                vadd.u16    q0,q0,q1
                add         r3,#1
                add         r2,#16
                bal         loop_u16x8

pre_loop_u16:   mov         r3,#0
                vadd.u16    d0,d0,d1
                vstr.u16    d0,[sp]

                ldrh        r0,[sp]
                ldrh        r4,[sp,#2]
                add         r0,r0,r4
                ldrh        r4,[sp,#4]
                add         r0,r0,r4
                ldrh        r4,[sp,#6]
                add         r0,r0,r4

loop_u16:       cmp         r3,r5
                bge         fin_u16

                ldrh        r4,[r2]
                add         r0,r0,r4
                add         r3,#1           // Update idx
                add         r2,#2           // Update pointer
                bal         loop_u16

fin_u16:        add         sp,#8
                pop         {r4,r5}
                bx          lr

