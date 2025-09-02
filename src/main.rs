use std::fs::OpenOptions;
use blkrs::run_lsblk;
use clap::{ArgAction, Parser};
use env_logger::{Builder, Target};
use log::{Level, LevelFilter};
use serde_json;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Werner Oscar",
    name = "lsblk-rs",
    about = "lsblk in Rust"
)]
struct Opts {
    #[clap(short, long, action = ArgAction::Count, help = "Set verbosity level")]
    verbose_level: u8,

    #[clap(long, help = "Enable logging to a file", env = "BLKRS_LOG_FILE")]
    log_file: bool,

    #[clap(short, long, help = "Enable debug mode", env = "BLKRS_DEBUG")]
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

    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Debug);

    if opts.log_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("blkrs.log")
            .unwrap();
        builder.target(Target::Pipe(Box::new(file)));
    } else {
        builder.target(Target::Stderr);
    }

    builder.init();

    // Example of using global args
    if opts.debug {
        log::info!("Debug mode is enabled");
    }

    log::debug!("debug message logging is enabled");
    match opts.cmd {
        Command::Info(info_opts) => {
            match opts.verbose_level {
                0..=2 => println!("Running in verbose mode level {}", opts.verbose_level),
                _ => eprintln!("Select a valid verbosity level (0-2)"),
            }
            let output = serde_json::to_string_pretty(&run_lsblk(&info_opts.device)).unwrap();
            println!("{}", output);
        }
    }
}
