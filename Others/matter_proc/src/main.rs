use clap::Parser;
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

fn main() {
    let args = Args::parse();

    let mat = matter::new(
        args.vendor_id,
        args.product_id,
        args.passcode,
        args.discriminator,
        args.commissioning_flow,
        args.discovery_cap_bitmask,
    );
    println!("Manualcode : {}", mat.gen_manual_code());
    println!("QRCode     : {}", mat.gen_qr_code());
}
