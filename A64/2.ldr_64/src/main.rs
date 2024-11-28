mod unsafe_ops;
use rand::{thread_rng, Rng};

fn main() {
    let (short_vec, word_vec, quad_vec) = (
        rand_vec!(i16, 8, -100.0, 100.0),
        rand_vec!(i32, 8, -100.0, 100.0), 
        rand_vec!(i64, 8, -100.0, 100.0)
    );

    print!("(Short)");
    print_vec!(short_vec);
    println!("Safe:{:5} Unsafe: {:5}\n", sum_vec!(short_vec), unsafe_ops::sum_short(short_vec));

    print!("(Word) ");
    print_vec!(word_vec);
    println!("Safe:{:5} Unsafe: {:5}\n", sum_vec!(word_vec), unsafe_ops::sum_word(word_vec));

    print!("(Quad) ");
    print_vec!(quad_vec);
    println!("Safe:{:5} Unsafe: {:5}\n", sum_vec!(quad_vec), unsafe_ops::sum_quad(quad_vec));
}
