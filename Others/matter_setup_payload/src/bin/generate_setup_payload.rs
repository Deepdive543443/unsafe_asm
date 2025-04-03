use clap::Parser;
use matter_proc::matter;
use matter_proc::MatterProcErr;
use qrcode_generator::QrCodeEcc;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    vendor_id: u16,

    #[arg(long)]
    product_id: u16,

    #[arg(long)]
    discriminator: u16,

    #[arg(long)]
    passcode: u32,

    #[arg(long, default_value_t = 0)]
    commissioning_flow: u8,

    #[arg(long, default_value_t = 2)]
    discovery_cap_bitmask: u8,

    #[arg(short, long, default_value_t = false)]
    save_qrcode: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let matter = matter::new(
        args.vendor_id,
        args.product_id,
        args.passcode,
        args.discriminator,
        args.commissioning_flow,
        args.discovery_cap_bitmask,
    )?;

    let manualcode = matter.gen_manual_code()?;
    let qrcode = matter.gen_qr_code()?;

    println!("Manualcode : {}", manualcode);
    println!("QRCode     : {}", &qrcode);

    if args.save_qrcode {
        match qrcode_generator::to_png_to_file(
            &qrcode,
            QrCodeEcc::Medium,
            256,
            format!("{}.png", &qrcode[3..]),
        ) {
            Ok(()) => Ok(()),
            Err(_) => MatterProcErr!(format!("Fail to save qrcode img {}", &qrcode[3..]))
        }?;
    }
    Ok(())
}
