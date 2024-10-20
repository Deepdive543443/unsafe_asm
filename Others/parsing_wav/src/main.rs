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

struct WavHeader {
    riff            : String,
    size            : u32,
    wavefmt         : String,
    format_size     : u32,
    format_type     : u16,
    num_channels    : u16,
    sample_rate     : u32,
    bytes_per_sec   : u32,
    mono            : u16,
    bits_per_sample : u16,
    chunk_mark      : String,
    data_size       : u32
}

fn wav_header_init() -> WavHeader {
    WavHeader {
        riff            : String::new(),
        size            : 0,
        wavefmt         : String::new(),
        format_size     : 0,
        format_type     : 0,
        num_channels    : 0,
        sample_rate     : 0,
        bytes_per_sec   : 0,
        mono            : 0,
        bits_per_sample : 0,
        chunk_mark      : String::new(),
        data_size       : 0
    }
}

fn parse_wav_header(input: &[u8]) -> IResult<&[u8], WavHeader> {
    let mut wav_header = wav_header_init();
    let mut ptr = input;
    let mut temp: char;

    wav_header.riff       = String::new();
    wav_header.wavefmt    = String::new();
    wav_header.chunk_mark = String::new();
    
    for _ in 0..4 { (ptr, temp) = anychar(ptr)?;wav_header.riff.push(temp); }
    (ptr, wav_header.size) = le_u32(ptr)?;
    for _ in 0..8 { (ptr, temp) = anychar(ptr)?;wav_header.wavefmt.push(temp); }
    
    (ptr, wav_header.format_size)     = le_u32(ptr)?;
    (ptr, wav_header.format_type)     = le_u16(ptr)?;
    (ptr, wav_header.num_channels)    = le_u16(ptr)?;
    (ptr, wav_header.sample_rate)     = le_u32(ptr)?;
    (ptr, wav_header.bytes_per_sec)   = le_u32(ptr)?;
    (ptr, wav_header.mono)            = le_u16(ptr)?;
    (ptr, wav_header.bits_per_sample) = le_u16(ptr)?;
    
    for _ in 0..4     { (ptr, temp)      = anychar(ptr)?;wav_header.chunk_mark.push(temp); }
    (ptr, wav_header.data_size) = le_u32(ptr)?;
    
    Ok((ptr, wav_header))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone())?;
    
    let mut contents: Vec<u8> = Vec::new();
    let ptr = &mut contents;
    let _ = file.read_to_end(ptr);

    let (_, wav_header) = parse_wav_header(ptr).unwrap();

    println!("{:<15} : {}", "RIFF"           , wav_header.riff);
    println!("{:<15} : {}", "File size"      , wav_header.size);
    println!("{:<15} : {}", "wavefmt"        , wav_header.wavefmt);
    println!("{:<15} : {}", "format_size"    , wav_header.format_size);
    println!("{:<15} : {}", "format_type"    , wav_header.format_type);
    println!("{:<15} : {}", "num_channels"   , wav_header.num_channels);
    println!("{:<15} : {}", "sample_rate"    , wav_header.sample_rate);
    println!("{:<15} : {}", "bytes_per_sec"  , wav_header.bytes_per_sec);
    println!("{:<15} : {}", "mono"           , wav_header.mono);
    println!("{:<15} : {}", "bits_per_sample", wav_header.bits_per_sample);
    println!("{:<15} : {}", "chunk_mark"     , wav_header.chunk_mark);
    println!("{:<15} : {}", "Data_size"      , wav_header.data_size);
    Ok(())
}