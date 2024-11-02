use verhoeff::Verhoeff;
mod base38;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CommissioningFlow {
    Standard,
    UserIntent,
    Custom
}

struct MatterPrivate {
    vid                 : u32,
    pid                 : u32,
    passcode            : u32,
    discriminator       : u16,
    short_discriminator : u8,
    flow                : CommissioningFlow,
    cap_bitmask         : u32
}

pub struct Matter {
    ctx: MatterPrivate
}

const CHUNK1_DISC_MSBITS_LEN: i32 = 2;
const CHUNK1_DISC_MSBITS_POS: i32 = 0;
const CHUNK1_VID_PID_POS    : i32 = CHUNK1_DISC_MSBITS_LEN + CHUNK1_DISC_MSBITS_POS;
const CHUNK2_DISC_LSBITS_LEN: i32 = 2;
const CHUNK2_PASS_LSBITS_LEN: i32 = 14;
const CHUNK2_PASS_LSBITS_POS: i32 = 0;
const CHUNK2_DISC_LSBITS_POS: i32 = CHUNK2_PASS_LSBITS_POS + CHUNK2_PASS_LSBITS_LEN;
const CHUNK3_PASS_MSBITS_LEN: i32 = 13;
const CHUNK3_PASS_MSBITS_POS: i32 = 0;
const DISC_LEN  : i32 = 4;
const PASS_LEN  : i32 = 27;
const QR_VER    : i32 = 0;
const QR_PADDING: i32 = 0;

fn to_bytes(bit_array: &String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut idx : usize = 0;
    let mut byte: u8 = 0;
    for c in bit_array.chars() {
        byte = byte << 1;
        idx += 1;
        if c == '1' { byte |= 0b1 }
        if idx % 8 == 0 { 
            bytes.push(byte);
            byte = 0
        }
    }
    return bytes;
}

impl MatterPrivate {
    fn chunk1(&self) -> u8 {
        let disc_shift = DISC_LEN - CHUNK1_DISC_MSBITS_LEN;
        let disc_mask  = (1 << CHUNK1_DISC_MSBITS_LEN) - 1;
        let disc_chunk = (self.short_discriminator >> disc_shift) & disc_mask; 
        let vid_pid_present_flag = if self.flow == CommissioningFlow::Standard { 0 } else { 1 };
        return (disc_chunk << CHUNK1_DISC_MSBITS_POS) | (vid_pid_present_flag << CHUNK1_VID_PID_POS);  
    }

    fn chunk2(&self) -> u32 {
        let disc_mask = (1 << CHUNK2_DISC_LSBITS_LEN) - 1;
        let pass_mask = (1 << CHUNK2_PASS_LSBITS_LEN) - 1;
        let disc_chunk = (self.short_discriminator & disc_mask) as u32;
        return ((self.passcode & pass_mask) << CHUNK2_PASS_LSBITS_POS) | (disc_chunk << CHUNK2_DISC_LSBITS_POS);
    }

    fn chunk3(&self) -> u32 {
        let pass_shift = PASS_LEN - CHUNK3_PASS_MSBITS_LEN;
        let pass_mask  = (1 << CHUNK3_PASS_MSBITS_LEN) - 1;
        return ((self.passcode >> pass_shift) & pass_mask) << CHUNK3_PASS_MSBITS_POS;
    }
}

impl Matter {
    pub fn gen_manual_code(&self) -> String {
        let mut output = format!("{:01}{:05}{:04}", self.ctx.chunk1(), self.ctx.chunk2(), self.ctx.chunk3());
        if self.ctx.flow != CommissioningFlow::Standard {
            output += &format!("{:05}", self.ctx.vid);
            output += &format!("{:05}", self.ctx.vid);
        }
        output += &format!("{}", output.calculate_verhoeff_check_digit());
        return output;
    }

    pub fn gen_qr_code(&self) -> String {
        let mut bits = format!("{:04b}", QR_PADDING);
        bits += &format!("{:027b}", self.ctx.passcode);
        bits += &format!("{:012b}", self.ctx.discriminator);
        bits += &format!("{:08b}" , self.ctx.cap_bitmask);

        match self.ctx.flow {
            CommissioningFlow::Standard   => bits += &format!("{:02b}" , CommissioningFlow::Standard   as u32),
            CommissioningFlow::UserIntent => bits += &format!("{:02b}" , CommissioningFlow::UserIntent as u32),
            CommissioningFlow::Custom     => bits += &format!("{:02b}" , CommissioningFlow::Custom     as u32),
        }

        bits += &format!("{:016b}", self.ctx.pid);
        bits += &format!("{:016b}", self.ctx.vid);
        bits += &format!("{:03b}", QR_VER);

        let mut bytes_rev: Vec<u8> = Vec::new();
        for byte in (to_bytes(&bits)).into_iter().rev() {
            bytes_rev.push(byte);
        }

        return format!("MT:{}", base38::encode(bytes_rev));
    }
}

pub fn new(vid: u32, pid: u32, passcode: u32, discriminator: u16, flow: u8, cap_bitmask: u32) -> Matter {
    let flow = match flow {
        0 => Ok(CommissioningFlow::Standard),
        1 => Ok(CommissioningFlow::UserIntent),
        2 => Ok(CommissioningFlow::Custom),
        _ => Err("Err flow")
    }.unwrap();

    let discriminator = match discriminator {
        0..=4095 => Ok(discriminator),
        _ => Err("Err discriminator")
    }.unwrap();

    let passcode = match passcode {
        0        | 11111111 | 22222222 | 33333333 | 44444444 | 
        55555555 | 66666666 | 77777777 | 88888888 | 99999999 | 
        12345678 | 87654321 => Err("Err passcode"),
        _ => Ok(passcode)
    }.unwrap();

    let short_discriminator: u8 = (discriminator >> 8) as u8;
    Matter{
        ctx: MatterPrivate{vid, pid, passcode, discriminator, short_discriminator, flow, cap_bitmask}
    }
}