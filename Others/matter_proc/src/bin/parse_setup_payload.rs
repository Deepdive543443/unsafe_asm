use clap::Parser;
use matter_proc::matter;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    qrcode: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let matter = matter::parse_qrcode(&args.qrcode[..])?;
    matter.print();
    Ok(())
}
