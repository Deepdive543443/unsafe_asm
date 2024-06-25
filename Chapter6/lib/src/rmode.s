                .text
                .global getRMode
//  int getRMode();
getRMode:       vmrs    r1,fpscr
                lsr     r2,r1,#22
                and     r0,r2,#3
                bx      lr

                .global setRMode
//  void setRMode(int mode);
setRMode:       vmrs    r1,fpscr
                bfi     r1,r0,#22,#3
                vmsr    fpscr,r1
                bx      lr

                .global f64_2i32
//  int f64_2i32(double x);
f64_2i32:       vcvtr.s32.f64   s0,d0
                vmov    r0,s0
                bx      lr
