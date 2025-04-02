use clap::Parser;
use matter_proc::matter;
use std::io;

#[derive(Parser)]
struct Args {
    qrcode: String,
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
    Ok(())
}
