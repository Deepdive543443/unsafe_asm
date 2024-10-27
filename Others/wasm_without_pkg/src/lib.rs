// ref: https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/#:~:text=Specifically%2C%20we%27ll%20want%20the%20wasm-bindgen%20command%20line%20utility%3A,the%20wasm-bindgen%20Guide%3A%20cargo%20new%20--lib%20no-modules%20no-modules%2FCargo.toml%3A

use wasm_bindgen::prelude::*;
use chksum_hash_md5 as md5;

#[wasm_bindgen]
pub fn checksum_md5(buf: &mut [u8], len: usize) -> String {
    let mut vec_buf: Vec<u8> = Vec::new();
    for i in 0..len { vec_buf.push(buf[i]); }
    return md5::hash(vec_buf).to_hex_lowercase().into();
}