fn main () {
    let a: f32 = 0.01;
    let b: f32 = 0.001;
    let c: f32 = 0.0001;

    println!("{:.30} {:.30}", (a * b) * c, a * (b * c));
}