// This is a tryout on parsing header of .wav file, based on Nom library.
// It has been a hard time on discovering how to parse a header only with stdlib.
// Mostly because of Rust doesn't recommand us to touching unsafe feature, like 
// we does in C/C++, which is likely casting a struct pointer.
// 
// Thanks for the works Nom library (https://github.com/rust-bakery/nom). This is 
// by far the most elegant implementation I found in the communities. But I believe
// my approach could still be improved. Please let me know if you have any suggestion 
// that helps to make the implementation simpler and elegant

use std::env;
use std::fs::File;
use std::io::prelude::*;
use nom::{number::complete::{le_u16, le_u32}, IResult};
use nom::character::complete::anychar;

fn parse_wav_header(input: &[u8]) -> IResult<&[u8], (String, u32, String, u32, u16, u16, u32, u32, u16, u16, String, u32)> {
    let mut ptr = input;
    let mut temp: char;

    let mut riff       = String::new();
    let mut wavefmt    = String::new();
    let mut chunk_mark = String::new();
    
    for _ in 0..4 { (ptr, temp) = anychar(ptr)?;riff.push(temp); }
    let size: u32;  (ptr, size) = le_u32(ptr)?;
    for _ in 0..8 { (ptr, temp) = anychar(ptr)?;wavefmt.push(temp); }
    
    let format_size     : u32; (ptr, format_size)     = le_u32(ptr)?;
    let format_type     : u16; (ptr, format_type)     = le_u16(ptr)?;
    let num_channels    : u16; (ptr, num_channels)    = le_u16(ptr)?;
    let sample_rate     : u32; (ptr, sample_rate)     = le_u32(ptr)?;
    let bytes_per_sec   : u32; (ptr, bytes_per_sec)   = le_u32(ptr)?;
    let mono            : u16; (ptr, mono)            = le_u16(ptr)?;
    let bits_per_sample : u16; (ptr, bits_per_sample) = le_u16(ptr)?;
    
    for _ in 0..4     { (ptr, temp)      = anychar(ptr)?;chunk_mark.push(temp); }
    let data_size: u32; (ptr, data_size) = le_u32(ptr)?;
    
    Ok((ptr, (riff, size, wavefmt, format_size, format_type, num_channels, sample_rate, bytes_per_sec, mono, bits_per_sample, chunk_mark, data_size)))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone())?;
    
    let mut contents: Vec<u8> = Vec::new();
    let ptr = &mut contents;
    let _ = file.read_to_end(ptr);

    let (_, wav_header) = parse_wav_header(ptr).unwrap();

    println!("{:<15} : {}", "RIFF"           , wav_header.0);
    println!("{:<15} : {}", "File size"      , wav_header.1);
    println!("{:<15} : {}", "wavefmt"        , wav_header.2);
    println!("{:<15} : {}", "format_size"    , wav_header.3);
    println!("{:<15} : {}", "format_type"    , wav_header.4);
    println!("{:<15} : {}", "num_channels"   , wav_header.5);
    println!("{:<15} : {}", "sample_rate"    , wav_header.6);
    println!("{:<15} : {}", "bytes_per_sec"  , wav_header.7);
    println!("{:<15} : {}", "mono"           , wav_header.8);
    println!("{:<15} : {}", "bits_per_sample", wav_header.9);
    println!("{:<15} : {}", "chunk_mark"     , wav_header.10);
    println!("{:<15} : {}", "Data_size"      , wav_header.11);
    Ok(())
}