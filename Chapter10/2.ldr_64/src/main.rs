mod unsafe_ops;

fn main() {
    let word_vec:  Vec<i32> = vec!(1,2,3,4,5,6,7,8);
    let quad_vec:  Vec<i64> = vec!(10, -20, 30, 40, 50, -60, 70, -80);
    let short_vec: Vec<i16> = vec!(100, -200, 300, 400, 500, -600, 700, -800);

    let mut sum: i16 = 0;
    for i in 0..short_vec.len() {sum += short_vec[i]}
    println!("Safe:{} Unsafe: {}", sum, unsafe_ops::sum_short(short_vec));

    let mut sum: i32 = 0;
    for i in 0..word_vec.len() {sum += word_vec[i]}
    println!("Safe:{} Unsafe: {}", sum, unsafe_ops::sum_word(word_vec));
    print_vec!(quad_vec);
}
