//! A simple implementation of the `head` command in Rust.
//! # Examples:
//! ```shell
//! $ cargo run -- -n 5 /etc/passwd
//! $ cargo run -- -n 5 < /etc/passwd
//! ```

use clap::Parser;
use std::io::{BufRead, BufReader};
use std::{io, process::ExitCode};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The number of lines to print
    #[arg(short, long, default_value_t = 10)]
    number: u8,

    /// The file to read from (optional)
    file: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let stdin = std::io::stdin();

    let mut count = args.number;

    if count == 0 {
        eprintln!("Invalid number of lines to print: {}", count);
        return ExitCode::FAILURE;
    }

    let reader: Box<dyn io::Read> = match args.file {
        Some(f) => {
            let file = match std::fs::File::open(f) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            Box::new(file)
        }
        None => Box::new(stdin.lock()),
    };

    // Loop through each line of the file,
    // breaking when we output the desired number of lines
    for line in BufReader::new(reader).lines() {
        println!("{}", line.unwrap());
        count -= 1;
        if count == 0 {
            break;
        }
    }

    ExitCode::SUCCESS
}
