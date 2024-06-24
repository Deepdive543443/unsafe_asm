use std::env;
extern "C" {
    fn quo_rem_asm(a: &i32, b: &i32, quo: &mut i32, rem: &mut i32);
}

fn quo_rem(a: &i32, b: &i32, quo: &mut i32, rem: &mut i32) {
    *quo = *a / *b;
    *rem = *a % *b;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let a :i32 = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {114514};
    let b :i32 = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {1919};
    let mut quo: i32 = 0;
    let mut rem: i32 = 0;

    quo_rem(&a, &b, &mut quo, &mut rem);
    println!("{quo:>5} {rem:>5} Rust");
    quo = 0;
    rem = 0;
    unsafe { quo_rem_asm(&a, &b, &mut quo, &mut rem); }
    println!("{quo:>5} {rem:>5} ASM");
}