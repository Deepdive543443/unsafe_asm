use bytemuck::cast_slice_mut;
use std::fs::File;
use std::io::prelude::*;

#[cfg(feature = "neon")]
use std::arch::asm;

pub struct NV12 {
    pub yy: Vec<u8>,
    pub uv: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl NV12 {
    pub fn bgr(&self) -> Vec<u8> {
        let (height, width) = (self.height, self.width);

        let mut bgr = vec![0u8; width * height * 3];
        for h in (0..height).step_by(2) {
            for w in (0..width).step_by(2) {
                let offset = h * width + w;
                let offset_uv = h * width / 2 + w;
                let offset_pixel00 = offset * 3;
                let offset_pixel10 = (offset + width) * 3;

                let (y00, y01, y10, y11, u, v) = (
                    (self.yy[offset] as i32) << 6,
                    (self.yy[offset + 1] as i32) << 6,
                    (self.yy[offset + width] as i32) << 6,
                    (self.yy[offset + width + 1] as i32) << 6,
                    (self.uv[offset_uv] as i32) - 128,
                    (self.uv[offset_uv + 1] as i32) - 128,
                );
                let (ruv, guv, buv) = (90 * v, (-46 * v) + (-22 * u), 113 * u);

                bgr[offset_pixel00] = ((y00 + buv) >> 6) as u8;
                bgr[offset_pixel00 + 1] = ((y00 + guv) >> 6) as u8;
                bgr[offset_pixel00 + 2] = ((y00 + ruv) >> 6) as u8;

                bgr[offset_pixel00 + 3] = ((y01 + buv) >> 6) as u8;
                bgr[offset_pixel00 + 4] = ((y01 + guv) >> 6) as u8;
                bgr[offset_pixel00 + 5] = ((y01 + ruv) >> 6) as u8;

                bgr[offset_pixel10] = ((y10 + buv) >> 6) as u8;
                bgr[offset_pixel10 + 1] = ((y10 + guv) >> 6) as u8;
                bgr[offset_pixel10 + 2] = ((y10 + ruv) >> 6) as u8;

                bgr[offset_pixel10 + 3] = ((y11 + buv) >> 6) as u8;
                bgr[offset_pixel10 + 4] = ((y11 + guv) >> 6) as u8;
                bgr[offset_pixel10 + 5] = ((y11 + ruv) >> 6) as u8;
            }
        }
        return bgr;
    }

    #[cfg(not(feature = "neon"))]
    pub fn rot(&self) -> NV12 {
        let mut rotated = NV12 {
            yy: vec![0u8; self.width * self.height],
            uv: vec![0u8; self.width * self.height / 2],
            width: self.width,
            height: self.height,
        };

        rotated.yy.copy_from_slice(&self.yy[..]);
        rotated.uv.copy_from_slice(&self.uv[..]);

        let u16_buffer: &mut [u16] = cast_slice_mut(&mut rotated.uv[..]);
        rotated.yy.reverse();
        u16_buffer.reverse();

        return rotated;
    }

    #[cfg(feature = "neon")]
    pub fn rot(&self) -> NV12 {
        let mut rotated = NV12 {
            yy: vec![0u8; self.width * self.height],
            uv: vec![0u8; self.width * self.height / 2],
            width: self.width,
            height: self.height,
        };

        let mut num_vec = (self.width * self.height) >> 4;
        let mut remain = self.width * self.height - num_vec * 16;

        if num_vec > 0 {
            let rev_tbl: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            unsafe {
                asm!(
                    "ld1        {{v0.16b}}, [x2]",
                    "mov        x5, #-16",

                    "0:",
                    "ld1        {{v1.16b}}, [x3], #16",
                    "tbl        v2.16b, {{v1.16b}}, v0.16b",
                    "st1        {{v2.16b}}, [x4], x5",
                    "subs       x0, x0, #1",
                    "bne        0b",

                    "sub        x4, x4, x5",
                    "cmp        x1, #0",
                    "ble        2f",

                    "1:",
                    "ldr        x6, [x3], #1",
                    "str        x6, [x4]",
                    "sub        x4, x4, #1",

                    "subs       x1, x1, #1",
                    "bne        1b",

                    "2:",
                    inout("x0") num_vec,
                    inout("x1") remain,
                    in("x2") &rev_tbl[0],
                    in("x3") &self.yy[0],
                    in("x4") &rotated.yy[self.width * self.height - 16],
                );
            }
        }

        let mut num_vec = (self.width * self.height / 2) >> 4;
        let mut remain = (self.width * self.height / 2) - num_vec * 16;

        if num_vec > 0 {
            let rev_tbl: [u8; 16] = [14, 15, 12, 13, 10, 11, 8, 9, 6, 7, 4, 5, 2, 3, 0, 1];
            unsafe {
                asm!(
                    "ld1        {{v0.16b}}, [x2]",
                    "mov        x5, #-16",

                    "0:",
                    "ld1        {{v1.16b}}, [x3], #16",
                    "tbl        v2.16b, {{v1.16b}}, v0.16b",
                    "st1        {{v2.16b}}, [x4], x5",
                    "subs       x0, x0, #1",
                    "bne        0b",

                    "sub        x4, x4, x5",
                    "cmp        x1, #0",
                    "ble        2f",

                    "1:",
                    "ldrsw      x6, [x3], #2",
                    "str        x6, [x4]",
                    "sub        x4, x4, #2",

                    "subs       x1, x1, #1",
                    "bne        1b",

                    "2:",
                    inout("x0") num_vec,
                    inout("x1") remain,
                    in("x2") &rev_tbl[0],
                    in("x3") &self.uv[0],
                    in("x4") &rotated.uv[(self.width * self.height / 2) - 16],
                );
            }
        }
        return rotated;
    }
}

pub fn new(filename: String, width: usize, height: usize) -> std::io::Result<NV12> {
    let mut nv12_data = File::open(filename)?;
    let mut nv12 = NV12 {
        yy: vec![0u8; width * height],
        uv: vec![0u8; width * height / 2],
        width: width,
        height: height,
    };

    nv12_data.read_exact(&mut nv12.yy)?;
    nv12_data.read_exact(&mut nv12.uv)?;

    Ok(nv12)
}
