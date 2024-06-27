                .text
zero_double:    .double 0.0
                .global double_stddev
//  void double_stddev(double *arr, int size, double *mean, double *std);
double_stddev:  push            {r4,r5}
                cmp             r1,#0
                ble             fin     // size > 0

                mov             r4,#0               // idx init
                mov             r5,r0               // ptr
                vldr.f64        d0,zero_double      // mean init
                vldr.f64        d1,zero_double      // std init

mean_loop:      vldmia          r5!,{d2}
                vadd.f64        d0,d0,d2            // mean += arr[idx]

                add             r4,#1
                cmp             r4,r1
                blt             mean_loop

                vmov            s5,r1
                vcvt.f64.s32    d3,s5
                vdiv.f64        d0,d0,d3
                vstr.f64        d0,[r2]             // mean /= size

                mov             r4,#0               // idx init
                mov             r5,r0
stddev_loop:    vldmia          r5!,{d2}
                vsub.f64        d3,d0,d2            // d3 = mean - arr[idx]
                vfma.f64        d1,d3,d3            // d1 = d3 * d3

                add             r4,#1
                cmp             r4,r1
                blt             stddev_loop

                vmov            s5,r1
                vcvt.f64.s32    d3,s5
                vdiv.f64        d1,d1,d3
                vsqrt.f64       d1,d1
                vstr.f64        d1,[r3]             // mean /= size


fin:            pop             {r4,r5}
                bx              lr