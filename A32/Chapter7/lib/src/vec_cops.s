//  typedef {
@       F32_CVT_I32
@       I32_CVT_F32
@       F32_CVT_U32
@       U32_CVT_F32
@       F32_CMP
@       U32_CMP
@       NUM_OPS
//  } cvtEnum;
                .text               //  Addr
cvtEnum:        .word   F32_CVT_I32 //  (0x0)
                .word   I32_CVT_F32 //  (0x4)
                .word   F32_CVT_U32 //  (0x8)
                .word   U32_CVT_F32 //  (0xc)
                .word   F32_CMP     //  ...
                .word   U32_CMP
                .equ    NUM_OPS,(. - cvtEnum) / 4
                
//  int vec_cvt(void *src_ptr, void *dst_ptr, void *out_ptr, cvtEnum ops);
                .global vec_cvt
vec_cvt:        push            {r4}
                cmp             r3,#NUM_OPS
                bge             err
                
                adr             r4,cvtEnum
                ldr             r4,[r4,r3,lsl #2]
                vldm            r0,{q0}     // load f32 vec
                vldm            r1,{q1}     // load s32 vec
                bx              r4

F32_CVT_I32:    vcvt.s32.f32    q1,q0 
                vstm            r1,{q1}
                b               fin

I32_CVT_F32:    vcvt.f32.s32    q0,q1
                vstm            r0,{q0}
                b               fin

F32_CVT_U32:    vcvt.u32.f32    q1,q0 
                vstm            r1,{q1}
                b               fin

U32_CVT_F32:    vcvt.f32.u32    q0,q1
                vstm            r0,{q0}
                b               fin

F32_CMP:        vcgt.f32        q2,q0,q1
                vstm            r2,{q2}
                b               fin

U32_CMP:        vcgt.u32        q2,q0,q1
                vstm            r2,{q2}
                b               fin
                
err:            mov             r0,#-1
fin:            mov             r0,#0
                pop             {r4}
                bx              lr
