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

        nv12_src.rot();

        let time_end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("[{:2}/20] {} ms", i + 1, time_end - time_start);
    }

    let nv12_rot180 = nv12_src.rot();
    bitmap::new(&nv12_src.bgr(), nv12_src.width, nv12_src.height).save("src")?;
    bitmap::new(&nv12_rot180.bgr(), nv12_rot180.width, nv12_rot180.height).save("rot180")?;
    Ok(())
}
