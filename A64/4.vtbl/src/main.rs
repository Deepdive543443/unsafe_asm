use std::env;
mod bench;
mod bitmap;
mod nv12;

const NV12_WIDTH: usize = 1536;
const NV12_HEIGHT: usize = 1022;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let nv12_src = nv12::new(args[1].clone(), NV12_WIDTH, NV12_HEIGHT)?;
    let rots: [i32; 3] = [90, 180, 270];

    // Bench
    for rot in rots {
        let mut timer = bench::init(100, format!("Bench {} ROT", rot));
        for _ in timer.iter() {
            nv12_src.rot(rot)?;
        }
    }

    let nv12_rot180 = nv12_src.rot(180)?;
    let nv12_rot90 = nv12_src.rot(90)?;
    let nv12_rot270 = nv12_src.rot(270)?;

    bitmap::new(&nv12_src.bgr(), nv12_src.width, nv12_src.height).save("src")?;
    println!("Saving BMP src");

    bitmap::new(&nv12_rot180.bgr(), nv12_rot180.width, nv12_rot180.height).save("rot180")?;
    println!("Saving BMP rot180");

    bitmap::new(&nv12_rot270.bgr(), nv12_rot270.width, nv12_rot270.height).save("rot270")?;
    println!("Saving BMP rot270");

    bitmap::new(&nv12_rot90.bgr(), nv12_rot90.width, nv12_rot90.height).save("rot90")?;
    println!("Saving BMP rot90");
    Ok(())
}
