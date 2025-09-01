use clap::{Command, Arg};
use serde_json;
use blkrs::run_lsblk;
mod util; // pull util module into scope

fn main() {
    let matches = Command::new("lsblk")
        .version("0.0.1")
        .author("Werner Oscar")
        .about("lsblk in Rust")
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(device) = matches.get_one::<String>("device") {
        let output = serde_json::to_string_pretty(&run_lsblk(device)).unwrap();
        println!("{}", output);
    } else {
        println!("No device provided");
    }
}
