// use std::arch::asm;     //  Inline Assembly
use std::env;
// TODO: https://www.quora.com/Can-a-C-library-be-used-from-rust-If-so-how-can-it-be-done-and-why-would-it-be-necessary
extern crate libc;
use libc::c_float;
extern "C" { 
    fn celcius_2f_asm(celcius: c_float) -> c_float;
    fn fahrenheit_2c_asm(fahrenheit: c_float) -> c_float;
} 



fn c2f(celcius: f32) -> f32 {
    return (celcius - 32.0) * 5.0 / 9.0;
}

fn f2c(fahrenheit: f32) -> f32 {
    return fahrenheit * 9.0 / 5.0 + 32.0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let celcius    :f32 = if args.len() > 1 {args[1].parse::<f32>().unwrap()} else {114.514};
    let fahrenheit :f32 = if args.len() > 2 {args[2].parse::<f32>().unwrap()} else {1919.810};
    println!("Rust: {} {}",c2f(celcius), f2c(fahrenheit));
}
