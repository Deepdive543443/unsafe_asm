use clap::Parser;
use std::io;
mod matter;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    vendor_id: u32,

    #[arg(long)]
    product_id: u32,

    #[arg(long)]
    discriminator: u16,

    #[arg(long)]
    passcode: u32,

    #[arg(long, default_value_t = 0)]
    commissioning_flow: u8,

    #[arg(long, default_value_t = 2)]
    discovery_cap_bitmask: u32,
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
    println!("Manualcode : {}", matter.gen_manual_code());
    println!("QRCode     : {}", matter.gen_qr_code());
    Ok(())
}