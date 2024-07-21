use std::env;
use std::arch::asm;

fn unsafe_ops(num1: i32, num2: i32, num3: i32, num4: i32, num5: i32, ) -> i32 {
    let mut _w0 = num1;
    unsafe {
        asm!(
            "add    w0,w0,w1",
            "sub    w0,w0,w2",
            "mul    w0,w0,w3",
            "sdiv   w0,w0,w4",

            inout("w0") _w0,
            in("w1")    num2,
            in("w2")    num3,
            in("w3")    num4,
            in("w4")    num5,
        );
    }
    return _w0;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let num1 :i32 = if args.len() > 1 {args[1].parse().unwrap()} else {114};
    let num2 :i32 = if args.len() > 2 {args[2].parse().unwrap()} else {514};
    let num3 :i32 = if args.len() > 3 {args[3].parse().unwrap()} else {19};
    let num4 :i32 = if args.len() > 4 {args[4].parse().unwrap()} else {19};
    let num5 :i32 = if args.len() > 5 {args[5].parse().unwrap()} else {810};


    println!("RUST   : {}", (num1 + num2 - num3) * num4 / num5);
    println!("UNSAFE : {}", unsafe_ops(num1, num2, num3, num4, num5));
}
