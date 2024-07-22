use std::arch::asm;

#[macro_export]
macro_rules! print_vec {
    ($vec:tt) => { print!("[ "); for i in 0..$vec.len() { print!("{:>7.4} ", $vec[i]) }; println!("]"); };
}

#[macro_export]
macro_rules! sum_vec {
    ($vec:tt) => {
        {
            let mut sum = $vec[0]; 
            for i in 1..$vec.len() { sum += $vec[i] }; 
            sum
        }
    };
}


pub fn sum_short(src: Vec<i16>) -> i32 { 
    let mut _w0: i32 = 0;
    unsafe {
        asm!(
            "mov        x3,0", //idx
            "cmp        x3,x1",
            "bge        98f",

            "2:",
            "ldrsh      w4,[x2,x3,lsl 1]",
            "add        w0,w0,w4",
            "add        x3,x3,1",

            "cmp        x3,x1",
            "blt        2b",

            "98:",
            inout("w0") _w0,
            in("x1")    src.len(),
            in("x2")    &src[0],
        );
    }
    return _w0;
}

pub fn sum_word(src: Vec<i32>) -> i32 { 
    let mut _w0: i32 = 0;
    unsafe {
        asm!(
            "mov        w3,0", //idx
            "cmp        w3,w1",
            "bge        98f",

            "2:",
            "ldr        w4,[x2,w3,uxtw 2]",
            "add        w0,w0,w4",
            "add        x3,x3,1",

            "cmp        x3,x1",
            "blt        2b",

            "98:",
            inout("w0") _w0,
            in("w1")    src.len(),
            in("x2")    &src[0],
        );
    }
    return _w0;
}

pub fn sum_quad(src: Vec<i64>) -> i64 { 
    let mut _x0: i64 = 0;
    unsafe {
        asm!(
            "mov        x3,0", //idx
            "cmp        x3,x1",
            "bge        98f",

            "2:",
            "ldr        x4,[x2,x3,lsl 3]",
            "add        x0,x0,x4",
            "add        x3,x3,1",

            "cmp        x3,x1",
            "blt        2b",

            "98:",
            inout("x0") _x0,
            in("x1")    src.len(),
            in("x2")    &src[0],
        );
    }
    return _x0;
}
