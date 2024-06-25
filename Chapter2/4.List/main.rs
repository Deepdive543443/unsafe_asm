extern "C" {
    fn byte_list() -> u8;
    fn short_list() -> u16;
}

fn main () {
    let mut unsigned_8: u8 = 0;
    let mut unsigned_16: u16 = 0;

    unsafe {
        unsigned_8 = byte_list();
        unsigned_16 = short_list();
    }

    println!("Sum of List from ASM: u8:{} u16:{}", unsigned_8, unsigned_16);
}