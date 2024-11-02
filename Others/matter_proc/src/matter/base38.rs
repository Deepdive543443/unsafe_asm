const MAX_BYTES_IN_CHUNK: usize = 3;
const NUM_CHARS: [u32; 3] = [2, 4, 5];
const CODE: [char; 38] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '.',
];

pub fn encode(bytes: Vec<u8>) -> String {
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
            qrcode.push(CODE[val % CODE.len()]);
            val /= CODE.len();
        }
    }
    return qrcode;
}
