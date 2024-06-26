use std::env;

fn f64_stddev(arr: Vec<f64>) -> (f64, f64) {
    let mut mean: f64 = 0.0;
    let mut std : f64 = 0.0;

    for f in &arr {
        mean += f;
    }
    mean /= arr.len() as f64;
    
    for f in &arr {
        let temp = mean - f;
        std += temp * temp;
    }

    std = f64::sqrt(std / arr.len() as f64); 
    return (mean, std);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut float_vec: Vec<f64> = Vec::new();

    for i in 1..args.len() {
        float_vec.push(args[i].parse::<f64>().unwrap());
    }
    
    let result = f64_stddev(float_vec);
    println!("{} {}",result.0, result.1);
}