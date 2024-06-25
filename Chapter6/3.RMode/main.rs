use std::env;
extern "C" {
    fn getRMode() -> i32;
    fn setRMode(mode: i32);
    fn f64_2i32(x: f64) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rmode = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {0}; 
    let input = if args.len() > 2 {args[2].parse::<f64>().unwrap()} else {0.0}; 
    unsafe {
        setRMode(rmode);
        println!("RMode: {}", getRMode());
        println!("Conver {} to {}", input, f64_2i32(input));
    }
}