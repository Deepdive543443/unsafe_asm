use std::env;

extern "C" {
    fn int_add_asm(input: i32, add: i32, sub:i32) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input :i32 = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {114};
    let add   :i32 = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {514};
    let sub   :i32 = if args.len() > 3 {args[3].parse::<i32>().unwrap()} else {1919};

    unsafe {
        println!("Safe:   {:>5}\nUnsafe: {:>5}", input + add - sub, int_add_asm(input, add, sub));
    }
    
}