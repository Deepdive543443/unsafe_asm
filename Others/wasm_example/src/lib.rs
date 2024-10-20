use wasm_bindgen::prelude::*;
use chksum_hash_md5 as md5;

#[wasm_bindgen]
pub fn checksum_md5(buf: &mut [u8], len: usize) -> String {
    let mut vec_buf: Vec<u8> = Vec::new();
    for i in 0..len { vec_buf.push(buf[i]); }
    return md5::hash(vec_buf).to_hex_lowercase().into();
}
