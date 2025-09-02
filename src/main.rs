use clap::{Parser, ArgAction};
use serde_json;
use blkrs::run_lsblk;


#[derive(Parser)]
#[command(version = "0.0.1", author = "Werner Oscar", name = "lsblk-rs", about = "lsblk in Rust")]

struct Opts {
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(short, long)]
    debug: bool,
    
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "Get device info")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Device to get info about")]
    device: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            let output = run_lsblk(&device);
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
    }
}
