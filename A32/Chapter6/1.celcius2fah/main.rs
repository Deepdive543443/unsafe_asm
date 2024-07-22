use std::env;
extern "C" { 
    fn celcius_2f_asm(celcius: f32) -> f32;
    fn fahrenheit_2c_asm(fahrenheit: f32) -> f32;
}

fn c2f(celcius: f32) -> f32 {
    return (celcius - 32.0) * 5.0 / 9.0;
}

fn f2c(fahrenheit: f32) -> f32 {
    return fahrenheit * 9.0 / 5.0 + 32.0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let celcius    :f32 = if args.len() > 1 {args[1].parse().unwrap()} else {114.514};
    let fahrenheit :f32 = if args.len() > 2 {args[2].parse().unwrap()} else {1919.810};
    println!("Rust: {:>6.4} {:>6.4}",c2f(celcius), f2c(fahrenheit));
    unsafe {
        println!("ASM:  {:>6.4} {:>6.4}",celcius_2f_asm(celcius), fahrenheit_2c_asm(fahrenheit));
    }
}
