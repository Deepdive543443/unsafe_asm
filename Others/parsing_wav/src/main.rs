// This is a tryout on parsing header of .wav file, based on Nom library.
// It has been a hard time on discovering how to parse a header only with stdlib.
// Mostly because of Rust doesn't recommand us to touching unsafe feature, like 
// we does in C/C++, which is likely casting a struct pointer.
// 
// Thanks for the works Nom library (https://github.com/rust-bakery/nom). This is 
// by far the most elegant implementation I found in the communities. But I believe
// my approach could still be improved. Please let me know if you have any suggestion 
// that helps to make the implementation simpler and elegant
use std::mem;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use nom::{number::complete::{le_u16, le_u32}, IResult};
use nom::character::complete::anychar;
use chksum_hash_md5 as md5;

#[repr(C)]
#[derive(Default)]
struct WavHeader {
    riff           :[char; 4],
    size           : u32,
    wavefmt        :[char; 8],
    format_size    : u32,
    format_type    : u16,
    num_channels   : u16,
    sample_rate    : u32,
    bytes_per_sec  : u32,
    mono           : u16,
    bits_per_sample: u16,
    chunk_mark     :[char; 4],
    data_size      : u32
}

fn parse_wav_header(input: &[u8]) -> IResult<&[u8], WavHeader> {
    let mut wav_header = WavHeader::default();
    let mut ptr = input;
    
    for i in 0..4 { (ptr, wav_header.riff[i])       = anychar(ptr)?; }
    (ptr, wav_header.size)                          = le_u32(ptr)?;
    for i in 0..8 { (ptr, wav_header.wavefmt[i])    = anychar(ptr)?; }
    (ptr, wav_header.format_size)                   = le_u32(ptr)?;
    (ptr, wav_header.format_type)                   = le_u16(ptr)?;
    (ptr, wav_header.num_channels)                  = le_u16(ptr)?;
    (ptr, wav_header.sample_rate)                   = le_u32(ptr)?;
    (ptr, wav_header.bytes_per_sec)                 = le_u32(ptr)?;
    (ptr, wav_header.mono)                          = le_u16(ptr)?;
    (ptr, wav_header.bits_per_sample)               = le_u16(ptr)?;
    for i in 0..4 { (ptr, wav_header.chunk_mark[i]) = anychar(ptr)?; }
    (ptr, wav_header.data_size)                     = le_u32(ptr)?;
    
    Ok((ptr, wav_header))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone())?;
    let mut buf: Vec<u8> = Vec::new();

    file.read_to_end(&mut buf)?;
    let (_, wav_header) = parse_wav_header(&mut buf).unwrap();
    println!("{:<15} : {}", "File size"      , wav_header.size);
    println!("{:<15} : {}", "format_size"    , wav_header.format_size);
    println!("{:<15} : {}", "format_type"    , wav_header.format_type);
    println!("{:<15} : {}", "num_channels"   , wav_header.num_channels);
    println!("{:<15} : {}", "sample_rate"    , wav_header.sample_rate);
    println!("{:<15} : {}", "bytes_per_sec"  , wav_header.bytes_per_sec);
    println!("{:<15} : {}", "mono"           , wav_header.mono);
    println!("{:<15} : {}", "bits_per_sample", wav_header.bits_per_sample);
    println!("{:<15} : {}", "Data_size"      , wav_header.data_size);
    println!("{:<15} : {}", "Size of struct" , mem::size_of::<WavHeader>());
    println!("{:<15} : {}", "RIFF"           , String::from_iter(wav_header.riff));
    println!("{:<15} : {}", "wavefmt"        , String::from_iter(wav_header.wavefmt));
    println!("{:<15} : {}", "chunk_mark"     , String::from_iter(wav_header.chunk_mark));
    println!("{:<15} : {}", "MD5 checksum"   , md5::hash(buf).to_hex_lowercase());
    Ok(())
}