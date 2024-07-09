use rand::{thread_rng, Rng};

macro_rules! rand_vec {
    ($type:ty, $len:tt) => {
        {
            let mut rng = thread_rng();
            let mut vec_rand: Vec<$type> = Vec::new();
            for _ in 0..$len {vec_rand.push(rng.gen())}
            vec_rand
        }
    };
}

fn main() {
    let f32_rand = rand_vec!(f32, 23);
    let i32_rand = rand_vec!(i32, 23);
    let u8_rand = rand_vec!(u8, 23);
    println!("{}", f32_rand[3]);
    println!("{}", i32_rand[3]);
    println!("{}", u8_rand[3]);
}
