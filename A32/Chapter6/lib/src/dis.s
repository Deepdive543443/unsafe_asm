                    .text
                    .global distance_64_asm
//  double distance_64_asm(double x1, double y1, double z1, double x2, double y2, double z2);
distance_64_asm:    vsub.f64    d0,d3,d0    // d0 = x2 - x1
                    vmul.f64    d0,d3,d0    // d0 = d0 ^ 2

                    vsub.f64    d1,d4,d1    // d1  = y2 - y1
                    vfma.f64    d0,d1,d1    // d0 += (y2 - y1) ^ 2
                    
                    vsub.f64    d2,d5,d2    // d2  = z2 - z1
                    vfma.f64    d0,d2,d2    // d0 += (z2 - z1) ^ 2
                    
                    vsqrt.f64   d0,d0
                    bx          lr