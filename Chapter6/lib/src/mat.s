                .text 

//  float getVal(float *src_mat, int i, int j, int w);
                .global getVal
getVal:         mul         r1,r1,r3        // i *= w;
                add         r1,r1,r2        // i += j;
                add         r0,r0,r1,lsl #2 // *src_mat += (i << 4);

                vldr.f32    s0,[r0]
                bx          lr