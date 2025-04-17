use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
mod bitmap;
mod nv12;

const NV12_WIDTH: usize = 1536;
const NV12_HEIGHT: usize = 1022;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let nv12_src = nv12::new(args[1].clone(), NV12_WIDTH, NV12_HEIGHT)?;

    // Bench
    for i in 0..20 {
        let time_start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        nv12_src.rot(180)?;
        nv12_src.rot(90)?;
        nv12_src.rot(270)?;

        let time_end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("[{:2}/20] {} ms", i + 1, time_end - time_start);
    }

    let nv12_rot180 = nv12_src.rot(180)?;
    let nv12_rot90 = nv12_src.rot(90)?;
    let nv12_rot270 = nv12_src.rot(270)?;

    bitmap::new(&nv12_src.bgr(), nv12_src.width, nv12_src.height).save("src")?;
    bitmap::new(&nv12_rot180.bgr(), nv12_rot180.width, nv12_rot180.height).save("rot180")?;
    bitmap::new(&nv12_rot270.bgr(), nv12_rot270.width, nv12_rot270.height).save("rot270")?;
    bitmap::new(&nv12_rot90.bgr(), nv12_rot90.width, nv12_rot90.height).save("rot90")?;
    Ok(())
}
