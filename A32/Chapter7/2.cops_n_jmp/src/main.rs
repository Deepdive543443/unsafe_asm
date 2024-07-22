//  Some try out on macro rules, and general types
use rand::{thread_rng, Rng};
use std::ffi::c_void;
use std::env;

extern "C" {
    fn vec_cvt(f32_ptr: *const c_void, i32_ptr: *const c_void, out_ptr: *const c_void, ops: i32) -> i32;
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

fn ops_handler(ops: i32) {
    match ops {
        0 => println!("F32_CVT_I32"),
        1 => println!("I32_CVT_F32"),
        2 => println!("F32_CVT_U32"),
        3 => println!("U32_CVT_F32"),
        4 => println!("F32_CMP (vcgt.f32)"),
        5 => println!("U32_CMP (vcgt.u32)"),
        _ => {println!("Invalid Ops"); panic!()}
    }

    let (min, max, min_int, max_int): (f32, f32, i32, i32) = (-100.0, 100.0, 0, 10);

    match ops {
        0 | 1 => {
            let (vec1, vec2, vec3) = (rand_vec!(f32, 4, min, max), rand_vec!(i32, 4, min_int, max_int), rand_vec!(i32, 4, min_int, max_int));
            unsafe {
                vec_cvt(
                    to_cvoid_ptr::<f32>(&vec1[0]),
                    to_cvoid_ptr::<i32>(&vec2[0]),
                    to_cvoid_ptr::<i32>(&vec3[0]),
                    ops);
            };
            print_vec!(vec1);
            print_vec!(vec2);
            print_vec!(vec3);
        },
        2 | 3 => {
            let (min_int_u32, max_int_u32): (u32, u32) = (min_int as u32, max_int as u32);
            let (vec1, vec2, vec3) = (rand_vec!(f32, 4, min, max), rand_vec!(u32, 4, min_int_u32, max_int_u32), rand_vec!(i32, 4, min_int, max_int));
            unsafe {
                vec_cvt(
                    to_cvoid_ptr::<f32>(&vec1[0]),
                    to_cvoid_ptr::<u32>(&vec2[0]),
                    to_cvoid_ptr::<i32>(&vec3[0]),
                    ops);
            };
            print_vec!(vec1);
            print_vec!(vec2);
            print_vec!(vec3);
        },
        4 => {
            let (min_int_u32, max_int_u32): (u32, u32) = (min_int as u32, max_int as u32);
            let (vec1, vec2, vec3) = (rand_vec!(f32, 4, min, max), rand_vec!(f32, 4, min, max), rand_vec!(u32, 4, min_int_u32, max_int_u32));
            unsafe {
                vec_cvt(
                    to_cvoid_ptr::<f32>(&vec1[0]),
                    to_cvoid_ptr::<f32>(&vec2[0]),
                    to_cvoid_ptr::<u32>(&vec3[0]),
                    ops);
            };            
            print_vec!(vec1);
            print_vec!(vec2);
            println!("[0x{:08x} 0x{:08x} 0x{:08x} 0x{:08x}]",{vec3[0]}, {vec3[1]}, {vec3[02]}, {vec3[3]});
        },
        5 => {
            let (min_int_u32, max_int_u32): (u32, u32) = (min_int as u32, max_int as u32);
            let (vec1, vec2, vec3) = (rand_vec!(u32, 4, min_int_u32, max_int_u32), rand_vec!(u32, 4, min_int_u32, max_int_u32), rand_vec!(u32, 4, min_int_u32, max_int_u32));
            unsafe {
                vec_cvt(
                    to_cvoid_ptr::<u32>(&vec1[0]),
                    to_cvoid_ptr::<u32>(&vec2[0]),
                    to_cvoid_ptr::<u32>(&vec3[0]),
                    ops);
            };
            print_vec!(vec1);
            print_vec!(vec2);
            println!("[0x{:08x} 0x{:08x} 0x{:08x} 0x{:08x}]",{vec3[0]}, {vec3[1]}, {vec3[02]}, {vec3[3]});
        },
        _ => {println!("Invalid Ops"); panic!()}
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ops: i32 = if args.len() > 1 {args[1].parse().unwrap()} else {0};
    
    ops_handler(ops);
}
