mod unsafe_ops;
use rand::{thread_rng, Rng};

macro_rules! rand_vec {
    ($type:ty, $len:tt, $min:expr, $max:expr) => {
        {
            let (min, max) = ($min as $type, $max as $type);
            let mut rng = thread_rng();
            let mut vec_rand: Vec<$type> = Vec::new();
            for _ in 0..$len {vec_rand.push(rng.gen_range(min..max))}
            vec_rand
        }
    };
}

fn main() {
    let (short_vec, word_vec, quad_vec) = (
        rand_vec!(i16, 8, -100.0, 100.0),
        rand_vec!(i32, 8, -100.0, 100.0), 
        rand_vec!(i64, 8, -100.0, 100.0)
    );

    println!("(Short) Safe:{:8} Unsafe: {:8}", sum_vec!(short_vec), unsafe_ops::sum_short(short_vec));
    println!("(Word)  Safe:{:8} Unsafe: {:8}", sum_vec!(word_vec), unsafe_ops::sum_word(word_vec));
    println!("(Quad)  Safe:{:8} Unsafe: {:8}", sum_vec!(quad_vec), unsafe_ops::sum_quad(quad_vec));
}
