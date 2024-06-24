use std::env;
use std::f64;

extern "C" {
    fn distance_64_asm(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64;
}

fn distance(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dis_x: f64 = (x2 - x1) * (x2 - x1);
    let dis_y: f64 = (y2 - y1) * (y2 - y1);
    let dis_z: f64 = (z2 - z1) * (z2 - z1);
    return f64::sqrt(dis_x + dis_y + dis_z);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let x1 :f64 = if args.len() > 1 {args[1].parse::<f64>().unwrap()} else {114.0};
    let y1 :f64 = if args.len() > 2 {args[2].parse::<f64>().unwrap()} else {514.0};
    let z1 :f64 = if args.len() > 3 {args[3].parse::<f64>().unwrap()} else {1919.0};
    let x2 :f64 = if args.len() > 4 {args[4].parse::<f64>().unwrap()} else {810.};
    let y2 :f64 = if args.len() > 5 {args[5].parse::<f64>().unwrap()} else {114.514};
    let z2 :f64 = if args.len() > 6 {args[6].parse::<f64>().unwrap()} else {1919.810};

    println!("Safe:   {:>6.4}",distance(x1, y1, z1, x2, y2, z2));
    unsafe {
        println!("Unsafe: {:>6.4}", distance_64_asm(x1, y1, z1, x2, y2, z2));
    };
}
