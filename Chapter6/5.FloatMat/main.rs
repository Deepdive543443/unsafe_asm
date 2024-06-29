use std::env;

extern "C" {
    fn getVal(src_mat: &f32, i: usize, j: usize, w: usize) -> f32;
}


const WIDTH: usize = 416;
const HEIGHT: usize = 320;

fn main() {
    let args: Vec<String> = env::args().collect();
    let h: usize = if args.len() > 1 {args[1].parse().unwrap()} else {HEIGHT / 2};
    let w: usize = if args.len() > 2 {args[2].parse().unwrap()} else {WIDTH / 2};

    let mut mat: [[f32; WIDTH];HEIGHT] = [[0.0; WIDTH];HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            mat[i][j] = ((i as f32) / HEIGHT as f32) * ((j as f32) / WIDTH as f32);
        }
    }
    
    println!("{}", std::mem::size_of::<usize>());
    println!("{} ", mat[h][w]);
    unsafe {
        println!("{:.10} ", getVal(&mat[0][0], h, w, WIDTH));
    }
}