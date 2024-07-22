use std::env;
extern "C" {
    fn getRMode() -> i32;
    fn setRMode(mode: i32);
    fn f64_2i32(x: f64) -> i32;
}

fn print_result(x: f64) {
    unsafe {
        setRMode(0);
        println!("({})NEAREST : {}",getRMode(), f64_2i32(x));
        setRMode(1);
        println!("({})PLUSINF : {}",getRMode(), f64_2i32(x));
        setRMode(2);
        println!("({})MINUSINF: {}",getRMode(), f64_2i32(x));
        setRMode(3);
        println!("({})ZERO    : {}",getRMode(), f64_2i32(x));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: f64 = if args.len() > 1 {args[1].parse().unwrap()} else {0.5}; 
    print_result(input);
}