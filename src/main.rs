use clap::Parser;
use serde_json;
use blkrs::run_lsblk;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Werner Oscar",
    name = "lsblk-rs",
    about = "lsblk in Rust"
)]

struct Opts {
    #[clap(help = "Device to query")]
    device: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let output = serde_json::to_string_pretty(&run_lsblk(&opts.device)).unwrap();
    println!("Opts: {output}");
}
