use deku::prelude::*;
use std::io;
use verhoeff::Verhoeff;
mod base38;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct QRCode {
    #[deku(bits = 4)]
    padding: u8,

    #[deku(bits = 27)]
    pub passcode: u32,

    #[deku(bits = 12)]
    pub discriminator: u16,

    #[deku(bits = 8)]
    pub discovery: u8,

    #[deku(bits = 2)]
    pub flow: u8,

    #[deku(bits = 16)]
    pub pid: u16,

    #[deku(bits = 16)]
    pub vid: u16,

    #[deku(bits = 3)]
    version: u8,
}

pub struct Matter {
    pub qrcode: QRCode,
    short_discriminator: u8,
}

const CHUNK1_DISC_MSBITS_LEN: i32 = 2;
const CHUNK1_DISC_MSBITS_POS: i32 = 0;
const CHUNK1_VID_PID_POS: i32 = CHUNK1_DISC_MSBITS_LEN + CHUNK1_DISC_MSBITS_POS;
const CHUNK2_DISC_LSBITS_LEN: i32 = 2;
const CHUNK2_PASS_LSBITS_LEN: i32 = 14;
const CHUNK2_PASS_LSBITS_POS: i32 = 0;
const CHUNK2_DISC_LSBITS_POS: i32 = CHUNK2_PASS_LSBITS_POS + CHUNK2_PASS_LSBITS_LEN;
const CHUNK3_PASS_MSBITS_LEN: i32 = 13;
const CHUNK3_PASS_MSBITS_POS: i32 = 0;
const DISC_LEN: i32 = 4;
const PASS_LEN: i32 = 27;
const QR_VER: u8 = 0;
const QR_PADDING: u8 = 0;

#[macro_export]
macro_rules! MatterProcErr {
    ($ErrMsg: expr) => {Err(io::Error::new(io::ErrorKind::Other, $ErrMsg))};
}

impl Matter {
    fn chunk1(&self) -> u8 {
        let disc_shift = DISC_LEN - CHUNK1_DISC_MSBITS_LEN;
        let disc_mask = (1 << CHUNK1_DISC_MSBITS_LEN) - 1;
        let disc_chunk = (self.short_discriminator >> disc_shift) & disc_mask;
        let vid_pid_present_flag = if self.qrcode.flow == 0 { 0 } else { 1 };
        return (disc_chunk << CHUNK1_DISC_MSBITS_POS) | (vid_pid_present_flag << CHUNK1_VID_PID_POS);
    }

    fn chunk2(&self) -> u32 {
        let disc_mask = (1 << CHUNK2_DISC_LSBITS_LEN) - 1;
        let pass_mask = (1 << CHUNK2_PASS_LSBITS_LEN) - 1;
        let disc_chunk = (self.short_discriminator & disc_mask) as u32;
        return ((self.qrcode.passcode & pass_mask) << CHUNK2_PASS_LSBITS_POS) | (disc_chunk << CHUNK2_DISC_LSBITS_POS);
    }

    fn chunk3(&self) -> u32 {
        let pass_shift = PASS_LEN - CHUNK3_PASS_MSBITS_LEN;
        let pass_mask = (1 << CHUNK3_PASS_MSBITS_LEN) - 1;
        return ((self.qrcode.passcode >> pass_shift) & pass_mask) << CHUNK3_PASS_MSBITS_POS;
    }

    pub fn gen_manual_code(&self) -> io::Result<String> {
        let mut output = format!(
            "{:01}{:05}{:04}",
            self.chunk1(),
            self.chunk2(),
            self.chunk3()
        );
        match self.qrcode.flow {
            0 => Ok(()),
            1 | 2 => {
                output += &format!("{:05}{:05}", self.qrcode.vid, self.qrcode.pid);
                Ok(())
            }
            3.. => MatterProcErr!("Invalid commisionning flow"),
        }?;
        output += &format!("{}", output.calculate_verhoeff_check_digit());
        Ok(output)
    }

    pub fn gen_qr_code(&self) -> io::Result<String> {
        let mut data_out = self.qrcode.to_bytes()?;
        data_out.reverse();
        Ok(format!("MT:{}", base38::encode(data_out)?))
    }
}

pub fn new(
    vid: u16,
    pid: u16,
    passcode: u32,
    discriminator: u16,
    flow: u8,
    cap_bitmask: u8,
) -> io::Result<Matter> {
    match flow {
        0..=2 => Ok(()),
        _ => MatterProcErr!(format!("Invalid Flow Value {}, should be 0, 1, 2", flow)),
    }?;

    match discriminator {
        0..=4095 => Ok(()),
        _ => MatterProcErr!(format!("Invalid discriminator value {}, should be a value between 0 and 4095", discriminator)),
    }?;

    match passcode {
        0        | 11111111 | 22222222 |
        33333333 | 44444444 | 55555555 |
        66666666 | 77777777 | 88888888 |
        99999999 | 12345678 | 87654321
        => MatterProcErr!(format!("Invalid passcode value {}, please checkout documents about invalid value of passcode", passcode)),
        _ => Ok(()),
    }?;

    Ok(Matter {
        qrcode: QRCode {
            padding: QR_PADDING,
            passcode: passcode,
            discriminator: discriminator,
            discovery: cap_bitmask,
            flow: flow,
            pid: pid,
            vid: vid,
            version: QR_VER,
        },
        short_discriminator: (discriminator >> 8) as u8,
    })
}

pub fn parse_qrcode(input: &str) -> io::Result<Matter> {
    match input.len() {
        22 => Ok(()),
        _ => MatterProcErr!(format!("Invalid input lenght {}", input.len())),
    }?;

    match &input[0..3] {
        "MT:" => Ok(()),
        _ => MatterProcErr!(format!("Invalid input header {}", &input[0..3])),
    }?;

    let mut bytes: Vec<u8> = base38::decode(input[3..].to_string())?;
    bytes.reverse();

    let (_rest, val) = QRCode::from_bytes((bytes.as_ref(), 0))?;
    Ok(Matter {
        qrcode: QRCode {
            padding: QR_PADDING,
            passcode: val.passcode,
            discriminator: val.discriminator,
            discovery: val.discovery,
            flow: val.flow,
            pid: val.pid,
            vid: val.vid,
            version: QR_VER,
        },
        short_discriminator: (val.discriminator >> 8) as u8,
    })
}
