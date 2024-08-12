use std::arch::global_asm;

global_asm!(r#"
            .equ    ARG_I,0             // Offset of arg i
            .equ    ARG_J,8             // Offset of arg j (64 bits)

            .text
            .global sum_cubes_unsafe

sum_cubes_unsafe:
            ldrsw   x8,[SP,ARG_I]       // Load signed word
            ldrsb   x9,[SP,ARG_J]       // Load signed bytes

            and     w0,w0,0xff          // a unsigned extend to 64 bits
            mul     x10,x0,x0
            mul     x10,x10,x0

            sxth    x1,w1               // b Signed half extend to 64
            mul     x11,x1,x1
            madd    x10,x11,x1,x10

            sxtw    x2,w2               // c Signed word extend to 64
            mul     x11,x2,x2
            madd    x10,x11,x2,x10

            mul     x11,x3,x3           // d Untouched
            madd    x10,x11,x3,x10

            sxtb    x4,w4               // e Singed byte extend to 64
            mul     x11,x4,x4
            madd    x10,x11,x4,x10

            uxth    x5,w5               // f Unsigned half extend to 64
            mul     x11,x5,x5
            madd    x10,x11,x5,x10

            uxtw    x6,w6               // g Unsigned word extend to 64
            mul     x11,x6,x6
            madd    x10,x11,x6,x10
            
            mul     x11,x7,x7           // h Untouched
            madd    x10,x11,x7,x10

            mul     x11,x8,x8
            madd    x10,x11,x8,x10

            mul     x11,x9,x9
            madd    x0,x11,x9,x10
            ret
"#);

extern {
    fn sum_cubes_unsafe(a: u8, b: i16, c: i32, d: i64, e: i8, f: u16, g: u32, h: u64, i: i32, j: i16) -> i64;
}

macro_rules! square {
    ($input:tt) => {
        {
            let mut input_i64 = $input as i64;
            input_i64 = input_i64 * input_i64 * input_i64;
            input_i64
        }
    }
}
fn sum_cubes(a: u8, b: i16, c: i32, d: i64, e: i8, f: u16, g: u32, h: u64, i: i32, j: i16) -> i64{
    
    square!(a) + 
    square!(b) + 
    square!(c) + 
    square!(d) + 
    square!(e) + 
    square!(f) + 
    square!(g) + 
    square!(h) + 
    square!(i) + 
    square!(j)
}


fn main() {
    println!("Sum square (Safe)  : {}", sum_cubes(2, 4, 5, 6, 7, 8, 9, 1, 12, 4));
    unsafe {
        println!("Sum square (UnSafe): {}", sum_cubes_unsafe(2, 4, 5, 6, 7, 8, 9, 1, 12, 4));
    }


    println!("Hello, world!");
}
