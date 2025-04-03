// Ref: https://www.fileformat.info/format/bmp/egff.htm

// typedef struct _WinBMPFileHeader
// {
// 	WORD   FileType;     /* File type, always 4D42h ("BM") */
// 	DWORD  FileSize;     /* Size of the file in bytes */
// 	WORD   reserved1;    /* Always 0 */
// 	WORD   reserved2;    /* Always 0 */
// 	DWORD  bitmap_offset; /* Starting position of image data in bytes */
// } WINBMPFILEHEADER;

// typedef struct _WinNtBitmapHeader
// {
// 	DWORD Size;            /* Size of this header in bytes */
// 	LONG  Width;           /* Image width in pixels */
// 	LONG  Height;          /* Image height in pixels */
// 	WORD  Planes;          /* Number of color planes */
// 	WORD  BitsPerPixel;    /* Number of bits per pixel */
// 	DWORD Compression;     /* Compression methods used */
// 	DWORD SizeOfBitmap;    /* Size of bitmap in bytes */
// 	LONG  HorzResolution;  /* Horizontal resolution in pixels per meter */
// 	LONG  VertResolution;  /* Vertical resolution in pixels per meter */
// 	DWORD ColorsUsed;      /* Number of colors in the image */
// 	DWORD ColorsImportant; /* Minimum number of important colors */
// } WINNTBITMAPHEADER;
use std::fs::File;
use std::io::prelude::*;
use std::mem;

#[repr(C, packed(1))]
#[derive(Debug)]
struct WINBMPFILEHEADER {
    file_type: u16,
    file_size: u32,
    reserved: u32,
    bitmap_offset: u32,
}

#[repr(C, packed(1))]
#[derive(Debug)]
struct WINNTBITMAPHEADER {
    size: u32,
    width: u32,
    height: u32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    size_of_bitmap: u32,
    horz_resolution: u32,
    vert_resolution: u32,
    colors_used: u32,
    colors_important: u32,
}

pub struct BMP {
    file_header: WINBMPFILEHEADER,
    bitmap_header: WINNTBITMAPHEADER,
    data: Vec<u8>,
}

impl Default for WINBMPFILEHEADER {
    fn default() -> Self {
        WINBMPFILEHEADER {
            file_type: 0x4D42,
            file_size: 0,
            reserved: 0,
            bitmap_offset: (mem::size_of::<WINBMPFILEHEADER>()
                + mem::size_of::<WINNTBITMAPHEADER>()) as u32,
        }
    }
}

impl Default for WINNTBITMAPHEADER {
    fn default() -> Self {
        WINNTBITMAPHEADER {
            size: mem::size_of::<WINNTBITMAPHEADER>() as u32,
            width: 0,
            height: 0,
            planes: 1,
            bits_per_pixel: 24,
            compression: 0,
            size_of_bitmap: 0,
            horz_resolution: 0,
            vert_resolution: 0,
            colors_used: 0,
            colors_important: 0,
        }
    }
}

impl Default for BMP {
    fn default() -> Self {
        BMP {
            file_header: WINBMPFILEHEADER::default(),
            bitmap_header: WINNTBITMAPHEADER::default(),
            data: Vec::new(),
        }
    }
}

pub fn new(pixels: &[u8], width: usize, height: usize) -> BMP {
    let aligned_byte_per_row = (width * 3 * 8 + 31) / 32 * 4;
    let pixels_bytes_per_row = width * 3;

    let mut bmp = BMP::default();
    bmp.bitmap_header.width = width as u32;
    bmp.bitmap_header.height = height as u32;
    bmp.bitmap_header.size_of_bitmap = (width * height * 3) as u32;
    bmp.file_header.file_size = bmp.file_header.bitmap_offset + bmp.bitmap_header.size_of_bitmap;

    for h in (0..height).rev() {
        for byte_idx in 0..pixels_bytes_per_row {
            bmp.data.push(pixels[h * pixels_bytes_per_row + byte_idx]);
        }

        for _ in 0..(aligned_byte_per_row - pixels_bytes_per_row) {
            bmp.data.push(0);
        }
    }
    return bmp;
}

pub fn save(bmp: BMP, filename: &str) -> std::io::Result<()> {
    let mut bitmap_file = File::create(format!("{}.bmp", filename))?;

    let file_header_bytes = unsafe {
        std::mem::transmute::<WINBMPFILEHEADER, [u8; mem::size_of::<WINBMPFILEHEADER>()]>(
            bmp.file_header,
        )
    };
    bitmap_file.write_all(&file_header_bytes)?;

    let bitmap_header_bytes = unsafe {
        std::mem::transmute::<WINNTBITMAPHEADER, [u8; mem::size_of::<WINNTBITMAPHEADER>()]>(
            bmp.bitmap_header,
        )
    };
    bitmap_file.write_all(&bitmap_header_bytes)?;
    bitmap_file.write_all(bmp.data.as_slice())?;

    Ok(())
}
