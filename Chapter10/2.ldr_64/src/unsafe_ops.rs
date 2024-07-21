use std::arch::asm;

#[macro_export]
macro_rules! rand_vec {
    ($type:ty, $len:tt, $min:tt, $max:tt) => {
        {
            let mut rng = thread_rng();
            let mut vec_rand: Vec<$type> = Vec::new();
            for _ in 0..$len {vec_rand.push(rng.gen_range($min..$max))}
            vec_rand
        }
    };
}

#[macro_export]
macro_rules! print_vec {
    ($vec:tt) => { print!("[ "); for i in 0..$vec.len() { print!("{:>7.4} ", $vec[i]) }; println!("]"); }
}


pub fn sum_short(src: Vec<i16>) -> i32 { 
    let mut _w0: i32 = 0;
    unsafe {
        asm!(
            "mov        x3,#0", //idx
            "cmp        x3,x1",
            "bge        98f",

            "2:",
            "ldrsh      w4,[x2,x3,lsl #1]",
            "add        w0,w0,w4",
            "add        x3,x3,#1",

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
            "mov        x3,#0", //idx
            "cmp        x3,x1",
            "bge        98f",

            "2:",
            "ldr        w4,[x2,x3,lsl #2]",
            "add        w0,w0,w4",
            "add        x3,x3,#1",

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
pub fn sum_quad(src: Vec<i64>) { println!("sum_quad()"); }

