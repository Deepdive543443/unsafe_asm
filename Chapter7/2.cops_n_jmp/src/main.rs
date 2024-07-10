//  Some try out on macro rules, and general types
use rand::{thread_rng, Rng};
use std::ffi::c_void;

extern "C" {
    fn vec_cvt(f32_ptr: *const c_void, s32_ptr: *const c_void, out_ptr: *const c_void, ops: i32) -> i32;
}

//  Ref: https://users.rust-lang.org/t/converting-between-references-and-c-void/39599
fn to_cvoid_ptr<T>(r: &T) -> *const c_void
{
    r as *const T as *const c_void
}

macro_rules! rand_vec {
    ($type:ty, $len:tt, $min:tt, $max:tt) => {
        {
            let mut rng = thread_rng();
            let mut vec_rand: Vec<$type> = Vec::new();
            for _ in 0..$len {vec_rand.push(rng.gen_range($min..$max))}
            vec_rand
        }
    };
}

macro_rules! print_vec {
    ($vec:tt) => { print!("[ "); for i in 0..$vec.len() { print!("{:>7.4} ", $vec[i]) }; println!("]"); }
}

fn main() {
    let (min, max, min_int, max_int): (f32, f32, i32, i32) = (-100.0, 100.0, 0, 10);
    let f32_vec: Vec<f32> = rand_vec!(f32, 4, min, max);
    let i32_vec: Vec<i32> = rand_vec!(i32, 4, min_int, max_int);
    let out_vec: Vec<i32> = rand_vec!(i32, 4, min_int, max_int);

    println!("{:p} {:p} {:p} {:p}", &f32_vec[0], &f32_vec[1], &f32_vec[2], &f32_vec[3]);
    println!("{:p} {:p} {:p} {:p}", &i32_vec[0], &i32_vec[1], &i32_vec[2], &i32_vec[3]);
    println!("{:p} {:p} {:p} {:p}", &out_vec[0], &out_vec[1], &out_vec[2], &out_vec[3]);
    unsafe {
        vec_cvt(
            to_cvoid_ptr::<f32>(&f32_vec[0]),
            to_cvoid_ptr::<i32>(&i32_vec[0]),
            to_cvoid_ptr::<i32>(&out_vec[0]),
            0);
    };
    print_vec!(f32_vec);
    print_vec!(i32_vec);
    print_vec!(out_vec);
}
