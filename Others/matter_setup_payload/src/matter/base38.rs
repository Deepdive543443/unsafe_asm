use std::io;

const MAX_BYTES_IN_CHUNK: usize = 3;
const MAX_ENCODED_BYTES_IN_CHUNK: usize = 5;
const NUM_CHARS: [usize; 3] = [2, 4, 5];
const CODE: [&str; 38] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "-", ".",
];
const RADIX: i64 = CODE.len() as i64;

macro_rules! Base38Err {
    ($ErrMsg: expr) => {Err(io::Error::new(io::ErrorKind::Other, $ErrMsg))};
}

pub fn encode(bytes: Vec<u8>) -> io::Result<String> {
    if bytes.len() == 0 {
        Base38Err!("No Bytes provided")
    } else {
        Ok(())
    }?;

    let mut qrcode = String::new();

    for i in (0..bytes.len()).step_by(MAX_BYTES_IN_CHUNK) {
        let num_bytes_in_chunk = if (i + MAX_BYTES_IN_CHUNK) > bytes.len() {
            bytes.len() - i
        } else {
            MAX_BYTES_IN_CHUNK
        };

        let mut val: usize = 0;
        for j in i..(i + num_bytes_in_chunk) {
            val += (bytes[j] as usize) << (8 * (j - i))
        }
        for _ in 0..NUM_CHARS[num_bytes_in_chunk - 1] {
            qrcode += CODE[val % CODE.len()];
            val /= CODE.len();
        }
    }
    Ok(qrcode)
}

pub fn decode(chars: String) -> io::Result<Vec<u8>> {
    let mut bytearray: Vec<u8> = Vec::new();
    for i in (0..chars.len()).step_by(MAX_ENCODED_BYTES_IN_CHUNK) {
        let chars_in_chunk = if i + MAX_ENCODED_BYTES_IN_CHUNK > chars.len() {
            chars.len() - i
        } else {
            MAX_ENCODED_BYTES_IN_CHUNK
        };

        let mut value: i64 = 0;
        for j in (i..(i + chars_in_chunk)).rev() {
            let char_idx = match CODE.iter().position(|n| *n == &chars[j..j + 1]) {
                Some(x) => Ok(x as i64),
                None => Base38Err!(format!("Cannot decode character {}", &chars[j..j + 1])),
            }?;
            value = value * RADIX + char_idx;
        }

        let bytes_in_chunk: usize = match NUM_CHARS.iter().position(|n| *n == chars_in_chunk) {
            Some(x) => Ok(x + 1),
            None => Base38Err!(format!("Invalid chars in chunk {}", chars_in_chunk)),
        }?;
        for _ in 0..bytes_in_chunk {
            bytearray.push((value & 0xFF) as u8);
            value = value >> 8;
        }
    }
    Ok(bytearray)
}
