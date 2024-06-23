                    .text
shift_const:        .single 32.0
f_scaler:           .single 0.55555556
c_scaler:           .single 1.8
                    .global celcius_2f_asm
//  float celcius_2f_asm(float celcius);
celcius_2f_asm:     vldr.f32    s1,shift_const
                    vldr.f32    s2,f_scaler
                    vsub.f32    s0,s0,s1
                    vmul.f32    s0,s0,s2
                    bx          lr

//  float fahrenheit_2c_asm(float fahrenheit);
                    .global fahrenheit_2c_asm
fahrenheit_2c_asm:  vldr.f32    s1,shift_const
                    vldr.f32    s2,c_scaler
                    vmul.f32    s0,s2
                    vadd.f32    s0,s1
                    bx          lr

