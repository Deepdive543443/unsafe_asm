use bytemuck::cast_slice_mut;
use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::time::{SystemTime, UNIX_EPOCH};
mod bitmap;

const NV12_WIDTH: usize = 1536;
const NV12_HEIGHT: usize = 1022;

fn nv12_2_bgr(yy: &[u8], uv: &[u8], width: usize, height: usize, rgb: &mut [u8]) {
    for h in (0..height).step_by(2) {
        for w in (0..width).step_by(2) {
            let offset = h * width + w;
            let offset_uv = h * width / 2 + w;
            let offset_pixel00 = offset * 3;
            let offset_pixel10 = (offset + width) * 3;

            let (y00, y01, y10, y11, u, v) = (
                (yy[offset] as i32) << 6,
                (yy[offset + 1] as i32) << 6,
                (yy[offset + width] as i32) << 6,
                (yy[offset + width + 1] as i32) << 6,
                (uv[offset_uv] as i32) - 128,
                (uv[offset_uv + 1] as i32) - 128,
            );
            let (ruv, guv, buv) = (90 * v, (-46 * v) + (-22 * u), 113 * u);

            rgb[offset_pixel00] = ((y00 + buv) >> 6) as u8;
            rgb[offset_pixel00 + 1] = ((y00 + guv) >> 6) as u8;
            rgb[offset_pixel00 + 2] = ((y00 + ruv) >> 6) as u8;

            rgb[offset_pixel00 + 3] = ((y01 + buv) >> 6) as u8;
            rgb[offset_pixel00 + 4] = ((y01 + guv) >> 6) as u8;
            rgb[offset_pixel00 + 5] = ((y01 + ruv) >> 6) as u8;

            rgb[offset_pixel10] = ((y10 + buv) >> 6) as u8;
            rgb[offset_pixel10 + 1] = ((y10 + guv) >> 6) as u8;
            rgb[offset_pixel10 + 2] = ((y10 + ruv) >> 6) as u8;

            rgb[offset_pixel10 + 3] = ((y11 + buv) >> 6) as u8;
            rgb[offset_pixel10 + 4] = ((y11 + guv) >> 6) as u8;
            rgb[offset_pixel10 + 5] = ((y11 + ruv) >> 6) as u8;
        }
    }
}

fn nv12_rot180(yy: &[u8], uv: &[u8], dst_yy: &mut [u8], dst_uv: &mut [u8]) {
    dst_yy.copy_from_slice(&yy[..]);
    dst_uv.copy_from_slice(&uv[..]);

    let u16_buffer: &mut [u16] = cast_slice_mut(dst_uv);
    dst_yy.reverse();
    u16_buffer.reverse();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut nv12_data = File::open(args[1].clone())?;

    let mut src_yy = [0u8; NV12_WIDTH * NV12_HEIGHT];
    let mut src_uv = [0u8; NV12_WIDTH * NV12_HEIGHT / 2];
    nv12_data.read(&mut src_yy)?;
    nv12_data.read(&mut src_uv)?;

    {
        let mut rgb = [0u8; NV12_WIDTH * NV12_HEIGHT * 3];
        nv12_2_bgr(&src_yy, &src_uv, NV12_WIDTH, NV12_HEIGHT, &mut rgb);
        let bmp = bitmap::new(&rgb, NV12_WIDTH, NV12_HEIGHT);
        bitmap::save(bmp, "src")?;
    }

    // {
    //     let mut rgb = [0u8; NV12_WIDTH * NV12_HEIGHT * 3];
    //     let mut dst_yy = [0u8; NV12_WIDTH * NV12_HEIGHT];
    //     let mut dst_uv = [0u8; NV12_WIDTH * NV12_HEIGHT / 2];
    //     nv12_rot180(&src_yy, &src_uv, &mut dst_yy, &mut dst_uv);
    //     nv12_2_bgr(&dst_yy, &dst_uv, NV12_WIDTH, NV12_HEIGHT, &mut rgb);
    //     let bmp = bitmap::new(&rgb, NV12_WIDTH, NV12_HEIGHT);
    //     bitmap::save(bmp, "rot_180");
    // }
    Ok(())
}
