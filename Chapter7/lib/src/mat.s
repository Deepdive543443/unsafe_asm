                .text 
zero_single:    .single 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
//  float sum_f32(float *src, int length);  // r2=ptr, r3=idx, r4=quo, r5=rem r3Q0(store)(f32, f32, f32, f32) Q1(add)(f32, f32, f32, f32)
                .global sum_f32

sum_f32:        push        {r4,r5}
                cmp         r1,#0
                ble         fin
                
                mov         r2,#4
                sdiv        r4,r1,r2        // quo refer to num of simd ops
                mul         r3,r4,r2
                sub         r5,r1,r3        // rem for num of scalers ops
                
                ldr         r2,=zero_single
                vldm        r2,{q0,q1}      // Init Quad 0 and 1 vector 
                
                mov         r2,r0           // r2 = *src
                mov         r3,#0           // r3 = idx

loop_simd:      cmp         r3,r4           // Check if we need to jump to scaler loop
                movge       r3,#0           // Reset index
                vaddge.f32  s0,s0,s1        // Sum the content of Q0
                vaddge.f32  s0,s0,s2
                vaddge.f32  s0,s0,s3
                bge         loop
                
                vldm        r2,{q1}         // Load 4 float32 from mem, perform SIMD adding...
                vadd.f32    q0,q0,q1
                add         r3,#1
                add         r2,#16          // Moving 4 data forward
                bal         loop_simd

loop:           cmp         r3,r5
                bge         fin
                vldr.f32    s1,[r2]
                vadd.f32    s0,s0,s1
                add         r3,#1           // Update idx
                add         r2,#4           // Update pointer
                bal         loop

fin:            pop         {r4,r5}
                bx          lr