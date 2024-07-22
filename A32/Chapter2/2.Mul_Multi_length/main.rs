use std::env;

extern "C" {
    fn asm_mul(input: i32, mul: i32) -> i32;
    fn smull_asm(input: i32, mul: i32) -> i64;
    fn umull_asm(input: u32, mul: u32) -> u64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input :i32 = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {114};
    let mut mul   :i32 = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {514};

    let mut input_u32 = input as u32;
    let mut mul_u32   = mul as u32;

    println!("Safe: {}", input * mul);
    unsafe {
        println!("UnSafe: {}", asm_mul(input, mul));
        println!("SMull: {}", smull_asm(input, mul));
        println!("UMull: {}", umull_asm(input_u32, mul_u32));
    }

    input     = 4000;
    mul       = 10000;  // Overflow
    input_u32 = 0x80000000;
    mul_u32   = 0x80000000;

    unsafe {
        println!("(Overflow case)");
        println!("UnSafe: {}", asm_mul(input, mul));
        println!("SMull: {}", smull_asm(input, mul));
        println!("UMull: {}", umull_asm(input_u32, mul_u32));
    }

}