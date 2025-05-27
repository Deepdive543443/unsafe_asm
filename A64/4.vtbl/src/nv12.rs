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

#[macro_export]
macro_rules! NV12Err {
    ($ErrMsg: expr) => {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("NV12Err: {}", $ErrMsg),
        ))
    };
}

fn rotated_coordinate(
    src_w: usize,
    src_h: usize,
    src_x: usize,
    src_y: usize,
    rot: i32,
) -> std::io::Result<(usize, usize)> {
    match rot {
        90 => Ok((src_h - 1 - src_y, src_x)),
        180 => Ok((src_w - 1 - src_x, src_h - 1 - src_y)),
        270 => Ok((src_y, src_w - 1 - src_x)),
        _ => NV12Err!("Unsupported rotations angle"),
    }
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

    pub fn rot(&self, rot: i32) -> std::io::Result<NV12> {
        match rot {
            180 => {
                let mut rotated = NV12 {
                    yy: vec![0u8; self.width * self.height],
                    uv: vec![0u8; self.width * self.height / 2],
                    width: self.width,
                    height: self.height,
                };

                let mut _remain = self.width * self.height;

                #[cfg(feature = "neon")]
                {
                    let num_vec = (self.width * self.height) >> 4;
                    let rev_tbl: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

                    _remain = self.width * self.height - num_vec * 16;
                    if num_vec > 0 {
                        unsafe {
                            asm!(
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        x2, #-16",

                                "0:",
                                "prfm       pldl1keep, [x3, #128]",
                                "ld1        {{v1.16b}}, [x3], #16",
                                "tbl        v2.16b, {{v1.16b}}, v0.16b",
                                "st1        {{v2.16b}}, [x4], x2",
                                "subs       x0, x0, #1",
                                "bne        0b",

                                inout("x0") num_vec => _,
                                inout("x1") _remain  => _,
                                inout("x2") &rev_tbl[0] => _,   // offset
                                inout("x3") &self.yy[0] => _,
                                inout("x4") &rotated.yy[self.width * self.height - 16] => _,
                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                            );
                        }
                    }
                }

                rotated.yy[.._remain].copy_from_slice(&self.yy[(self.width * self.height - _remain)..]);
                rotated.yy[.._remain].reverse();

                _remain = self.width * self.height / 2;
                #[cfg(feature = "neon")]
                {
                    let num_vec = (self.width * self.height / 2) >> 4;
                    let rev_tbl: [u8; 16] = [14, 15, 12, 13, 10, 11, 8, 9, 6, 7, 4, 5, 2, 3, 0, 1];

                    _remain = (self.width * self.height / 4) - num_vec * 8;
                    if num_vec > 0 {
                        unsafe {
                            asm!(
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        x2, #-16",

                                "0:",
                                "prfm       pldl1keep, [x3, #128]",
                                "ld1        {{v1.16b}}, [x3], #16",
                                "tbl        v2.16b, {{v1.16b}}, v0.16b",
                                "st1        {{v2.16b}}, [x4], x2",
                                "subs       x0, x0, #1",
                                "bne        0b",
                                inout("x0") num_vec => _,
                                inout("x1") _remain => _,
                                inout("x2") &rev_tbl[0] => _,   // offset
                                inout("x3") &self.uv[0] => _,
                                inout("x4") &rotated.uv[(self.width * self.height / 2) - 16] => _,
                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                            );
                        }
                    }
                }

                rotated.uv[.._remain].copy_from_slice(&self.uv[(self.width * self.height / 2 - _remain)..]);
                let u16_buffer: &mut [u16] = bytemuck::cast_slice_mut(&mut rotated.uv[..]);
                u16_buffer[..(_remain / 2)].reverse();
                Ok(rotated)
            }
            90 => {
                let mut rotated = NV12 {
                    yy: vec![0u8; self.width * self.height],
                    uv: vec![0u8; self.width * self.height / 2],
                    width: self.height,
                    height: self.width,
                };

                let mut _y: usize = 0;

                #[cfg(feature = "neon")]
                {
                    let num_lane = self.height >> 3;
                    if num_lane > 0 {
                        let (tbl_v0, tbl_v4, tbl_v5): ([u8; 16], [u8; 16], [u8; 16]) = (
                            [57, 41, 25, 9, 56, 40, 24, 8, 49, 33, 17, 1, 48, 32, 16, 0],
                            [28, 29, 30, 31, 12, 13, 14, 15, 24, 25, 26, 27, 8, 9, 10, 11],
                            [20, 21, 22, 23, 4, 5, 6, 7, 16, 17, 18, 19, 0, 1, 2, 3],
                        );
                        let (dst_x, dst_y) =
                            rotated_coordinate(self.width, self.height, 0, 0, rot)?;
                        let num_mat = self.width >> 4;
                        let num_vec = self.width - (16 * num_mat);

                        unsafe {
                            asm!(
                                // Lookup table and others setup   [V0-V5]
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        w2, #2",
                                "dup        v4.16b, w2",
                                "add        v1.16b, v0.16b, v4.16b",
                                "add        v2.16b, v1.16b, v4.16b",
                                "add        v3.16b, v2.16b, v4.16b",
                                "ld1        {{v4.16b}}, [x3]",
                                "ld1        {{v5.16b}}, [x4]",

                                "mov        x2, x5",                   // nl

                                "mov        x13, #7",
                                "mul        x13, x13, x8",              // 7 * src_w

                                "0:",
                                "mov        x12, x1",                   // dst0

                                "cmp        x6, #0",
                                "ble        3f",                        // Jmp to vec

                                // Mat Rotation preload
                                "1:",
                                "mov        x3, x6",                   // nm

                                "2:",
                                "mov        x11, x0",                   // src0 = src_ptr
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v6.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v7.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v8.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v9.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v10.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v11.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v12.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v13.16b}}, [x11], x8",     // src0 += 8 * src_W

                                "tbl        v14.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v0.16b",
                                "tbl        v15.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v1.16b",
                                "tbl        v16.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v2.16b",
                                "tbl        v17.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v3.16b",
                                "tbl        v18.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v0.16b",
                                "tbl        v19.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v1.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v2.16b",
                                "tbl        v21.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v3.16b",
                                "orr        v6.16b, v14.16b, v14.16b",
                                "orr        v7.16b, v18.16b, v18.16b",
                                "orr        v8.16b, v15.16b, v15.16b",
                                "orr        v9.16b, v19.16b, v19.16b",
                                "orr        v10.16b, v16.16b, v16.16b",
                                "orr        v11.16b, v20.16b, v20.16b",
                                "orr        v12.16b, v17.16b, v17.16b",
                                "orr        v13.16b, v21.16b, v21.16b",
                                "tbl        v14.16b, {{v6.16b, v7.16b}}, v4.16b",
                                "tbl        v18.16b, {{v6.16b, v7.16b}}, v5.16b",
                                "tbl        v15.16b, {{v8.16b, v9.16b}}, v4.16b",
                                "tbl        v19.16b, {{v8.16b, v9.16b}}, v5.16b",
                                "tbl        v16.16b, {{v10.16b, v11.16b}}, v4.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b}}, v5.16b",
                                "tbl        v17.16b, {{v12.16b, v13.16b}}, v4.16b",
                                "tbl        v21.16b, {{v12.16b, v13.16b}}, v5.16b",

                                "st1        {{v14.d}}[0], [x12], x9",
                                "st1        {{v14.d}}[1], [x12], x9",
                                "st1        {{v15.d}}[0], [x12], x9",
                                "st1        {{v15.d}}[1], [x12], x9",
                                "st1        {{v16.d}}[0], [x12], x9",
                                "st1        {{v16.d}}[1], [x12], x9",
                                "st1        {{v17.d}}[0], [x12], x9",
                                "st1        {{v17.d}}[1], [x12], x9",
                                "st1        {{v18.d}}[0], [x12], x9",
                                "st1        {{v18.d}}[1], [x12], x9",
                                "st1        {{v19.d}}[0], [x12], x9",
                                "st1        {{v19.d}}[1], [x12], x9",
                                "st1        {{v20.d}}[0], [x12], x9",
                                "st1        {{v20.d}}[1], [x12], x9",
                                "st1        {{v21.d}}[0], [x12], x9",
                                "st1        {{v21.d}}[1], [x12], x9",   // += 16 * dst_w

                                "add        x0, x0, #16",               // src_ptr += 16
                                "subs       x3, x3, #1",                // nm--
                                "bne        2b",                        // Mat Rotation end

                                "cmp        x7, #0",
                                "ble        5f",                        // Jmp to next lane

                                // vec
                                "3:",
                                "mov        x4, x7",                    // nv

                                "4:",
                                "mov        x11, x0",                   // src0 = src_ptr
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[7], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[6], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[5], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[4], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[3], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[2], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[1], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[0], [x11], x8",
                                "st1        {{v6.d}}[0], [x12], x9",

                                "add        x0, x0, #1",                // src_ptr++
                                "subs       x4, x4, #1",                // nv--
                                "bne        4b",

                                // Next lane?
                                "5:",
                                "add        x10, x10, #8",              // y += 8
                                "add        x0, x0, x13",               // src_ptr += 7 * src_w
                                "sub        x1, x1, #8",                // dst_ptr -= 8
                                "subs       x2, x2, #1",                // nl--
                                "bne        0b",
                                inout("x0") &self.yy[0] => _,
                                inout("x1") &rotated.yy[dst_y * rotated.width + dst_x - 7] => _,
                                inout("x2") &tbl_v0[0] => _,    //  nl
                                inout("x3") &tbl_v4[0] => _,    //  nm
                                inout("x4") &tbl_v5[0] => _,    //  nv
                                inout("x5") num_lane => _,
                                inout("x6") num_mat => _,
                                inout("x7") num_vec => _,
                                inout("x8") self.width => _,    // dst_h
                                inout("x9") self.height => _,   // dst_w
                                inout("x10") _y,
                                out("x11") _,   //  src0
                                out("x12") _,   //  dst0
                                out("x13") _,   //  7 * src_ptr

                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                                out("v3") _,
                                out("v4") _,
                                out("v5") _,
                                out("v6") _,
                                out("v7") _,
                                out("v8") _,
                                out("v9") _,
                                out("v10") _,
                                out("v11") _,
                                out("v12") _,
                                out("v13") _,
                                out("v14") _,
                                out("v15") _,
                                out("v16") _,
                                out("v17") _,
                                out("v18") _,
                                out("v19") _,
                                out("v20") _,
                                out("v21") _,   // This is required to inform assembler which Register was used.
                                                // Otherwise Rust mess up the origin value in Register, led to undefined behaviour
                            );
                        }
                    }
                }

                for i in _y..self.height {
                    for j in 0..self.width {
                        let (dst_x, dst_y) =
                            rotated_coordinate(self.width, self.height, j, i, rot)?;
                        rotated.yy[rotated.width * dst_y + dst_x] = self.yy[self.width * i + j];
                    }
                }

                // uv: 16bits
                let (srcuv_width, srcuv_height) = (self.width / 2, self.height / 2);
                let dstuv_width = srcuv_height;
                let mut _y: usize = 0;

                #[cfg(feature = "neon")]
                {
                    let num_lane = srcuv_height >> 3;
                    if num_lane > 0 {
                        let (tbl_v0, tbl_v4, tbl_v5): ([u8; 16], [u8; 16], [u8; 16]) = (
                            [56, 57, 40, 41, 24, 25, 8, 9, 48, 49, 32, 33, 16, 17, 0, 1],
                            [24, 25, 26, 27, 28, 29, 30, 31, 8, 9, 10, 11, 12, 13, 14, 15],
                            [16, 17, 18, 19, 20, 21, 22, 23, 0, 1, 2, 3, 4, 5, 6, 7],
                        );
                        let (dstuv_x, dstuv_y) =
                            rotated_coordinate(srcuv_width, srcuv_height, 0, 0, rot)?;
                        let num_mat = self.width >> 4;
                        let num_vec = self.width / 2 - (8 * num_mat);

                        unsafe {
                            asm!(
                                // Lookup table and others setup   [V0-V5]
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        w2, #2",
                                "dup        v4.16b, w2",
                                "add        v1.16b, v0.16b, v4.16b",
                                "add        v2.16b, v1.16b, v4.16b",
                                "add        v3.16b, v2.16b, v4.16b",
                                "ld1        {{v4.16b}}, [x3]",
                                "ld1        {{v5.16b}}, [x4]",

                                "mov        x2, x5",                   // nl

                                "mov        x13, #7",
                                "mul        x13, x13, x8",              // 7 * src_w

                                "0:",
                                "mov        x12, x1",                   // dst0

                                "cmp        x6, #0",
                                "ble        3f",                        // Jmp to vec

                                // Mat Rotation preload
                                "1:",
                                "mov        x3, x6",                   // nm

                                "2:",
                                "mov        x11, x0",                   // src0 = src_ptr
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v6.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v7.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v8.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v9.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v10.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v11.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v12.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v13.16b}}, [x11], x8",     // src0 += 8 * src_W

                                "tbl        v14.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v0.16b",
                                "tbl        v15.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v1.16b",
                                "tbl        v16.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v2.16b",
                                "tbl        v17.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v3.16b",
                                "tbl        v18.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v0.16b",
                                "tbl        v19.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v1.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v2.16b",
                                "tbl        v21.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v3.16b",
                                "orr        v6.16b, v14.16b, v14.16b",
                                "orr        v7.16b, v18.16b, v18.16b",
                                "orr        v8.16b, v15.16b, v15.16b",
                                "orr        v9.16b, v19.16b, v19.16b",
                                "orr        v10.16b, v16.16b, v16.16b",
                                "orr        v11.16b, v20.16b, v20.16b",
                                "orr        v12.16b, v17.16b, v17.16b",
                                "orr        v13.16b, v21.16b, v21.16b",
                                "tbl        v14.16b, {{v6.16b, v7.16b}}, v4.16b",
                                "tbl        v18.16b, {{v6.16b, v7.16b}}, v5.16b",
                                "tbl        v15.16b, {{v8.16b, v9.16b}}, v4.16b",
                                "tbl        v19.16b, {{v8.16b, v9.16b}}, v5.16b",
                                "tbl        v16.16b, {{v10.16b, v11.16b}}, v4.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b}}, v5.16b",
                                "tbl        v17.16b, {{v12.16b, v13.16b}}, v4.16b",
                                "tbl        v21.16b, {{v12.16b, v13.16b}}, v5.16b",

                                "st1        {{v14.16b}}, [x12], x9",
                                "st1        {{v15.16b}}, [x12], x9",
                                "st1        {{v16.16b}}, [x12], x9",
                                "st1        {{v17.16b}}, [x12], x9",
                                "st1        {{v18.16b}}, [x12], x9",
                                "st1        {{v19.16b}}, [x12], x9",
                                "st1        {{v20.16b}}, [x12], x9",
                                "st1        {{v21.16b}}, [x12], x9", // += 8 * dst_w

                                "add        x0, x0, #16",               // src_ptr += 16
                                "subs       x3, x3, #1",                // nm--
                                "bne        2b",                        // Mat Rotation end

                                "cmp        x7, #0",
                                "ble        5f",                        // Jmp to next lane

                                // vec
                                "3:",
                                "mov        x4, x7",                    // nv

                                "4:",
                                "mov        x11, x0",                   // src0 = src_ptr
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[7], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[6], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[5], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[4], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[3], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[2], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[1], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[0], [x11], x8",
                                "st1        {{v6.8h}}, [x12], x9",

                                "add        x0, x0, #2",                // src_ptr++
                                "subs       x4, x4, #1",                // nv--
                                "bne        4b",

                                // Next lane?
                                "5:",
                                "add        x10, x10, #8",              // y += 8
                                "add        x0, x0, x13",               // src_ptr += 7 * src_w
                                "sub        x1, x1, #16",                // dst_ptr -= 8
                                "subs       x2, x2, #1",                // nl--
                                "bne        0b",
                                inout("x0") &self.uv[0] => _,
                                inout("x1") &rotated.uv[(dstuv_y * dstuv_width + dstuv_x - 7) * 2] => _,
                                inout("x2") &tbl_v0[0] => _,    //  nl
                                inout("x3") &tbl_v4[0] => _,    //  nm
                                inout("x4") &tbl_v5[0] => _,    //  nv
                                inout("x5") num_lane => _,
                                inout("x6") num_mat => _,
                                inout("x7") num_vec => _,
                                inout("x8") self.width => _,        // dst_h: self.width * 2
                                inout("x9") self.height => _,   // dst_w: self.height
                                inout("x10") _y,
                                out("x11") _,   //  src0
                                out("x12") _,   //  dst0
                                out("x13") _,   //  7 * src_ptr

                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                                out("v3") _,
                                out("v4") _,
                                out("v5") _,
                                out("v6") _,
                                out("v7") _,
                                out("v8") _,
                                out("v9") _,
                                out("v10") _,
                                out("v11") _,
                                out("v12") _,
                                out("v13") _,
                                out("v14") _,
                                out("v15") _,
                                out("v16") _,
                                out("v17") _,
                                out("v18") _,
                                out("v19") _,
                                out("v20") _,
                                out("v21") _,   // This is required to inform assembler which Register was used.
                                                // Otherwise Rust mess up the origin value in Register, led to undefined behaviour
                            );
                        }
                    }
                }

                let (dst_uv16, src_uv16): (&mut [u16], &[u16]) = (
                    bytemuck::cast_slice_mut(&mut rotated.uv[..]),
                    bytemuck::cast_slice(&self.uv[..]),
                );
                for i in _y..srcuv_height {
                    for j in 0..srcuv_width {
                        let (dst_x, dst_y) =
                            rotated_coordinate(srcuv_width, srcuv_height, j, i, rot)?;
                        dst_uv16[dstuv_width * dst_y + dst_x] = src_uv16[srcuv_width * i + j];
                    }
                }
                Ok(rotated)
            }
            270 => {
                let mut rotated = NV12 {
                    yy: vec![0u8; self.width * self.height],
                    uv: vec![0u8; self.width * self.height / 2],
                    width: self.height,
                    height: self.width,
                };

                let mut _y: usize = 0;

                #[cfg(feature = "neon")]
                {
                    let num_lane = self.height >> 3;
                    if num_lane > 0 {
                        let (tbl_v0, tbl_v4, tbl_v5): ([u8; 16], [u8; 16], [u8; 16]) = (
                            [15, 31, 47, 63, 14, 30, 46, 62, 7, 23, 39, 55, 6, 22, 38, 54],
                            [0, 1, 2, 3, 16, 17, 18, 19, 4, 5, 6, 7, 20, 21, 22, 23],
                            [8, 9, 10, 11, 24, 25, 26, 27, 12, 13, 14, 15, 28, 29, 30, 31],
                        );
                        let (dst_x, dst_y) =
                            rotated_coordinate(self.width, self.height, 0, 0, rot)?;
                        let num_mat = self.width >> 4;
                        let num_vec = self.width - (16 * num_mat);

                        unsafe {
                            asm!(
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        w2, #2",
                                "dup        v4.16b, w2",
                                "sub        v1.16b, v0.16b, v4.16b",
                                "sub        v2.16b, v1.16b, v4.16b",
                                "sub        v3.16b, v2.16b, v4.16b",
                                "ld1        {{v4.16b}}, [x3]",
                                "ld1        {{v5.16b}}, [x4]",
                                "mov        x2, #-1",
                                "mul        x9, x9, x2",
                                "mov        x2, x5",
                                "mov        x13, #7",
                                "mul        x13, x13, x8",
                                "0:",
                                "mov        x12, x1",
                                "cmp        x6, #0",
                                "ble        3f",
                                "1:",
                                "mov        x3, x6",
                                "2:",
                                "mov        x11, x0",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v6.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v7.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v8.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v9.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v10.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v11.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v12.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v13.16b}}, [x11], x8",
                                "tbl        v14.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v0.16b",
                                "tbl        v15.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v1.16b",
                                "tbl        v16.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v2.16b",
                                "tbl        v17.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v3.16b",
                                "tbl        v18.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v0.16b",
                                "tbl        v19.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v1.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v2.16b",
                                "tbl        v21.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v3.16b",
                                "orr        v6.16b, v14.16b, v14.16b",
                                "orr        v7.16b, v18.16b, v18.16b",
                                "orr        v8.16b, v15.16b, v15.16b",
                                "orr        v9.16b, v19.16b, v19.16b",
                                "orr        v10.16b, v16.16b, v16.16b",
                                "orr        v11.16b, v20.16b, v20.16b",
                                "orr        v12.16b, v17.16b, v17.16b",
                                "orr        v13.16b, v21.16b, v21.16b",
                                "tbl        v14.16b, {{v6.16b, v7.16b}}, v4.16b",
                                "tbl        v18.16b, {{v6.16b, v7.16b}}, v5.16b",
                                "tbl        v15.16b, {{v8.16b, v9.16b}}, v4.16b",
                                "tbl        v19.16b, {{v8.16b, v9.16b}}, v5.16b",
                                "tbl        v16.16b, {{v10.16b, v11.16b}}, v4.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b}}, v5.16b",
                                "tbl        v17.16b, {{v12.16b, v13.16b}}, v4.16b",
                                "tbl        v21.16b, {{v12.16b, v13.16b}}, v5.16b",
                                "st1        {{v21.d}}[1], [x12], x9",
                                "st1        {{v21.d}}[0], [x12], x9",
                                "st1        {{v20.d}}[1], [x12], x9",
                                "st1        {{v20.d}}[0], [x12], x9",
                                "st1        {{v19.d}}[1], [x12], x9",
                                "st1        {{v19.d}}[0], [x12], x9",
                                "st1        {{v18.d}}[1], [x12], x9",
                                "st1        {{v18.d}}[0], [x12], x9",
                                "st1        {{v17.d}}[1], [x12], x9",
                                "st1        {{v17.d}}[0], [x12], x9",
                                "st1        {{v16.d}}[1], [x12], x9",
                                "st1        {{v16.d}}[0], [x12], x9",
                                "st1        {{v15.d}}[1], [x12], x9",
                                "st1        {{v15.d}}[0], [x12], x9",
                                "st1        {{v14.d}}[1], [x12], x9",
                                "st1        {{v14.d}}[0], [x12], x9",
                                "add        x0, x0, #16",
                                "subs       x3, x3, #1",
                                "bne        2b",
                                "cmp        x7, #0",
                                "ble        5f",
                                "3:",
                                "mov        x4, x7",
                                "4:",
                                "mov        x11, x0",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[0], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[1], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[2], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[3], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[4], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[5], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[6], [x11], x8",
                                "prfm       pldl1keep, [x11, #8]",
                                "ld1        {{v6.b}}[7], [x11], x8",
                                "st1        {{v6.d}}[0], [x12], x9",
                                "add        x0, x0, #1",
                                "subs       x4, x4, #1",
                                "bne        4b",
                                "5:",
                                "add        x10, x10, #8",
                                "add        x0, x0, x13",
                                "add        x1, x1, #8",
                                "subs       x2, x2, #1",
                                "bne        0b",
                                inout("x0") &self.yy[0] => _,
                                inout("x1") &rotated.yy[dst_y * rotated.width + dst_x] => _,
                                inout("x2") &tbl_v0[0] => _,
                                inout("x3") &tbl_v4[0] => _,
                                inout("x4") &tbl_v5[0] => _,
                                inout("x5") num_lane => _,
                                inout("x6") num_mat => _,
                                inout("x7") num_vec => _,
                                inout("x8") self.width => _,
                                inout("x9") self.height => _,
                                inout("x10") _y,
                                out("x11") _,
                                out("x12") _,
                                out("x13") _,

                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                                out("v3") _,
                                out("v4") _,
                                out("v5") _,
                                out("v6") _,
                                out("v7") _,
                                out("v8") _,
                                out("v9") _,
                                out("v10") _,
                                out("v11") _,
                                out("v12") _,
                                out("v13") _,
                                out("v14") _,
                                out("v15") _,
                                out("v16") _,
                                out("v17") _,
                                out("v18") _,
                                out("v19") _,
                                out("v20") _,
                                out("v21") _,
                            );
                        }
                    }
                }

                for i in _y..self.height {
                    for j in 0..self.width {
                        let (dst_x, dst_y) =
                            rotated_coordinate(self.width, self.height, j, i, rot)?;
                        rotated.yy[rotated.width * dst_y + dst_x] = self.yy[self.width * i + j];
                    }
                }

                // uv: 16bits
                let (srcuv_width, srcuv_height) = (self.width / 2, self.height / 2);
                let dstuv_width = srcuv_height;

                let mut _y: usize = 0;

                #[cfg(feature = "neon")]
                {
                    let num_lane = srcuv_height >> 3;
                    if num_lane > 0 {
                        let (tbl_v0, tbl_v4, tbl_v5): ([u8; 16], [u8; 16], [u8; 16]) = (
                            [14, 15, 30, 31, 46, 47, 62, 63, 6, 7, 22, 23, 38, 39, 54, 55], // 62,63,46,47,30,31,14,15
                            [0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23],
                            [8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31],
                        );
                        let (dstuv_x, dstuv_y) =
                            rotated_coordinate(srcuv_width, srcuv_height, 0, 0, rot)?;
                        let num_mat = self.width >> 4;
                        let num_vec = self.width / 2 - (8 * num_mat);

                        unsafe {
                            asm!(
                                "ld1        {{v0.16b}}, [x2]",
                                "mov        w2, #2",
                                "dup        v4.16b, w2",
                                "sub        v1.16b, v0.16b, v4.16b",
                                "sub        v2.16b, v1.16b, v4.16b",
                                "sub        v3.16b, v2.16b, v4.16b",
                                "ld1        {{v4.16b}}, [x3]",
                                "ld1        {{v5.16b}}, [x4]",
                                "mov        x2, #-1",
                                "mul        x9, x9, x2",
                                "mov        x2, x5",
                                "mov        x13, #7",
                                "mul        x13, x13, x8",
                                "0:",
                                "mov        x12, x1",
                                "cmp        x6, #0",
                                "ble        3f",
                                "1:",
                                "mov        x3, x6",
                                "2:",
                                "mov        x11, x0",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v6.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v7.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v8.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v9.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v10.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v11.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v12.16b}}, [x11], x8",
                                "prfm       pldl1keep, [x11, #128]",
                                "ld1        {{v13.16b}}, [x11], x8",
                                "tbl        v14.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v0.16b",
                                "tbl        v15.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v1.16b",
                                "tbl        v16.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v2.16b",
                                "tbl        v17.16b, {{v6.16b, v7.16b, v8.16b, v9.16b}}, v3.16b",
                                "tbl        v18.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v0.16b",
                                "tbl        v19.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v1.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v2.16b",
                                "tbl        v21.16b, {{v10.16b, v11.16b, v12.16b, v13.16b}}, v3.16b",
                                "orr        v6.16b, v14.16b, v14.16b",
                                "orr        v7.16b, v18.16b, v18.16b",
                                "orr        v8.16b, v15.16b, v15.16b",
                                "orr        v9.16b, v19.16b, v19.16b",
                                "orr        v10.16b, v16.16b, v16.16b",
                                "orr        v11.16b, v20.16b, v20.16b",
                                "orr        v12.16b, v17.16b, v17.16b",
                                "orr        v13.16b, v21.16b, v21.16b",
                                "tbl        v14.16b, {{v6.16b, v7.16b}}, v4.16b",
                                "tbl        v18.16b, {{v6.16b, v7.16b}}, v5.16b",
                                "tbl        v15.16b, {{v8.16b, v9.16b}}, v4.16b",
                                "tbl        v19.16b, {{v8.16b, v9.16b}}, v5.16b",
                                "tbl        v16.16b, {{v10.16b, v11.16b}}, v4.16b",
                                "tbl        v20.16b, {{v10.16b, v11.16b}}, v5.16b",
                                "tbl        v17.16b, {{v12.16b, v13.16b}}, v4.16b",
                                "tbl        v21.16b, {{v12.16b, v13.16b}}, v5.16b",
                                "st1        {{v21.16b}}, [x12], x9",
                                "st1        {{v20.16b}}, [x12], x9",
                                "st1        {{v19.16b}}, [x12], x9",
                                "st1        {{v18.16b}}, [x12], x9",
                                "st1        {{v17.16b}}, [x12], x9",
                                "st1        {{v16.16b}}, [x12], x9",
                                "st1        {{v15.16b}}, [x12], x9",
                                "st1        {{v14.16b}}, [x12], x9",
                                "add        x0, x0, #16",
                                "subs       x3, x3, #1",
                                "bne        2b",
                                "cmp        x7, #0",
                                "ble        5f",
                                "3:",
                                "mov        x4, x7",
                                "4:",
                                "mov        x11, x0",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[0], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[1], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[2], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[3], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[4], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[5], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[6], [x11], x8",
                                "prfm       pldl1keep, [x11, #16]",
                                "ld1        {{v6.h}}[7], [x11], x8",
                                "st1        {{v6.8h}}, [x12], x9",
                                "add        x0, x0, #2",
                                "subs       x4, x4, #1",
                                "bne        4b",
                                "5:",
                                "add        x10, x10, #8",
                                "add        x0, x0, x13",
                                "add        x1, x1, #16",
                                "subs       x2, x2, #1",
                                "bne        0b",
                                inout("x0") &self.uv[0] => _,
                                inout("x1") &rotated.uv[(dstuv_y * dstuv_width + dstuv_x) * 2] => _,
                                inout("x2") &tbl_v0[0] => _,
                                inout("x3") &tbl_v4[0] => _,
                                inout("x4") &tbl_v5[0] => _,
                                inout("x5") num_lane => _,
                                inout("x6") num_mat => _,
                                inout("x7") num_vec => _,
                                inout("x8") self.width => _,
                                inout("x9") self.height => _,
                                inout("x10") _y,
                                out("x11") _,
                                out("x12") _,
                                out("x13") _,
                                out("v0") _,
                                out("v1") _,
                                out("v2") _,
                                out("v3") _,
                                out("v4") _,
                                out("v5") _,
                                out("v6") _,
                                out("v7") _,
                                out("v8") _,
                                out("v9") _,
                                out("v10") _,
                                out("v11") _,
                                out("v12") _,
                                out("v13") _,
                                out("v14") _,
                                out("v15") _,
                                out("v16") _,
                                out("v17") _,
                                out("v18") _,
                                out("v19") _,
                                out("v20") _,
                                out("v21") _,
                            );
                        }
                    }
                }

                let (dst_uv16, src_uv16): (&mut [u16], &[u16]) = (
                    bytemuck::cast_slice_mut(&mut rotated.uv[..]),
                    bytemuck::cast_slice(&self.uv[..]),
                );
                for i in _y..srcuv_height {
                    for j in 0..srcuv_width {
                        let (dst_x, dst_y) =
                            rotated_coordinate(srcuv_width, srcuv_height, j, i, rot)?;
                        dst_uv16[dstuv_width * dst_y + dst_x] = src_uv16[srcuv_width * i + j];
                    }
                }
                Ok(rotated)
            }
            _ => NV12Err!("Non supported rotation"),
        }
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
