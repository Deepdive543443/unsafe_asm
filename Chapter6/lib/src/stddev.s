                .text
zero_double:    .single
                .global double_stddev
//  void double_stddev(double *arr, int size, double *mean, double *std);
double_stddev:  push            {r4}
                cmp             r1,#0
                ble             fin     // size > 0

                mov             r4,#0               // idx init
                vldr.f64        d0,zero_double      // mean init
                vldr.f64        d1,zero_double      // std init

mean_loop:      vldr.f64        d2,[r0,r4,lsl #4]
                vadd.f64        d0,d0,d2            // mean += arr[idx]

                add             r4,#1
                cmp             r1,#0
                blt             mean_loop

                vmov            s5,r1
                vcvt.f64.s32    d3,s5
                vdiv.f64        d0,d0,d3
                vstr.f64        d0,[r2]             // mean /= size


fin:            pop             {r4}
                bx              lr