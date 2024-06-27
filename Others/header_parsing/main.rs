use std::fs::File;
use std::io::prelude::*;

#[repr(C)]
struct WavHeader {
    riff: String,
    size: u32
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("file_example_WAV_2MG.wav")?;
    let mut contents: Vec<u8> = Vec::new();
    
    let _ = file.read_to_end(&mut contents);
    let wav_header = WavHeader {
        riff: [contents[0] as char, contents[1] as char, contents[2] as char, contents[3] as char].iter().collect(),
        size: u32::from_ne_bytes([contents[4], contents[5], contents[6], contents[7]])
    };

    println!("{:<10} : {}", "RIFF"      , wav_header.riff);
    println!("{:<10} : {}", "File size" , wav_header.size);
    Ok(())
}