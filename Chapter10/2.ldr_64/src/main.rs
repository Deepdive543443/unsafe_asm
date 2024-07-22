mod unsafe_ops;
use rand::{thread_rng, Rng};

fn main() {
    let word_vec:  Vec<i32> = rand_vec!(i32, 8, 0, 255);
    let quad_vec:  Vec<i64> = rand_vec!(i64, 8, 0, 255);
    let short_vec: Vec<i16> = rand_vec!(i16, 8, 0, 100);

    let sum: i16 = sum_vec!(short_vec);
    println!("Safe:{} Unsafe: {}", sum, unsafe_ops::sum_short(short_vec));

    let sum: i32 = sum_vec!(word_vec);
    println!("Safe:{} Unsafe: {}", sum, unsafe_ops::sum_word(word_vec));

    let sum: i64 = sum_vec!(quad_vec);
    println!("Safe:{} Unsafe: {}", sum, unsafe_ops::sum_quad(quad_vec));
}
