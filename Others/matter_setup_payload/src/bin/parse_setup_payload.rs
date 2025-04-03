use clap::Parser;
use matter_proc::matter;
use matter_proc::MatterProcErr;
use qrcode_generator::QrCodeEcc;
use std::io;

#[derive(Parser)]
struct Args {
    qrcode: String,

    #[arg(short, long, default_value_t = false)]
    save_qrcode: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let matter = matter::parse_qrcode(&args.qrcode[..])?;
    println!("Flow                   : {}", matter.qrcode.flow);
    println!("Passcode               : {}", matter.qrcode.passcode);
    println!("Short Discriminator    : {}", matter.qrcode.discriminator >> 8);
    println!("Long Discriminator     : {}", matter.qrcode.discriminator);
    println!("Discovery Capabilities : {}", matter.qrcode.discovery);
    println!("Vendor Id              : {}   (0x{:04x})", matter.qrcode.vid, matter.qrcode.vid);
    println!("Product Id             : {}   (0x{:04x})", matter.qrcode.pid, matter.qrcode.pid);
    println!("ManualCode             : {}", matter.gen_manual_code()?);

    if args.save_qrcode {
        match qrcode_generator::to_png_to_file(
            &args.qrcode,
            QrCodeEcc::Medium,
            256,
            format!("{}.png", &args.qrcode[3..]),
        ) {
            Ok(()) => Ok(()),
            Err(_) => MatterProcErr!(format!("Fail to save qrcode img {}", &args.qrcode[3..]))
        }?;
    }
    Ok(())
}
