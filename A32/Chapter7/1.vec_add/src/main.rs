use rand::{thread_rng, Rng};

extern "C" {
    fn sum_f32(src: &f32, length: usize) -> f32;
    fn sum_u16(src: &u16, length: usize) -> u16;
}

fn sum_f32_unsafe(vec: &Vec<f32>) -> f32 {
    let length = vec.len();
    let mut _sum: f32 = 0.0;
    unsafe {
        _sum = sum_f32(&vec[0], length - 1);
    }
    return _sum;
}

fn sum_u16_unsafe(vec: &Vec<u16>) -> u16 {
    let length = vec.len();
    let mut _sum: u16 = 0;
    unsafe {
        _sum = sum_u16(&vec[0], length - 1);
    }
    return _sum;
}


const LENGTH: usize = 40000;

fn main() {
    let mut rng = thread_rng();
    let mut vec_f32: Vec<f32> = Vec::new(); for _ in 0..LENGTH {vec_f32.push(rng.gen())}
    let mut sum: f32 = 0.0; for i in 0..LENGTH {sum += vec_f32[i]} 

    let mut vec_u16: Vec<u16> = Vec::new(); for _ in 0..LENGTH {vec_u16.push(rng.gen_range(0..10))}
    let mut sum_u16: u16 = 0; for i in 0..LENGTH {sum_u16 += vec_u16[i]} 

    println!("{:<3} {:<5} : {:<8}  {:<5} : {:<8}", "(f32)","Safe", sum, "Unsafe", sum_f32_unsafe(&vec_f32));
    println!("{:<3} {:<5} : {:<8}  {:<5} : {:<8}", "(u16)","Safe", sum_u16, "Unsafe", sum_u16_unsafe(&vec_u16));
}
