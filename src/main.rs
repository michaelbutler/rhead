//! A simple implementation of the `head` command in Rust.
//! # Examples:
//! ```shell
//! $ cargo run -- -n 5 /etc/passwd
//! $ cargo run -- -n 5 < /etc/passwd
//! $ cargo run -- -c 3 < /etc/passwd
//! ```

use clap::Parser;
use std::io::{BufRead, BufReader, Read};
use std::{io, process::ExitCode};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The number of lines to print
    #[arg(short, long, default_value_t = 10)]
    number: u32,

    /// The number of characters to print (optional)
    #[arg(short, long, conflicts_with = "number", default_value_t = 0)]
    chars: u32,

    /// The file to read from (optional)
    file: Option<String>,
}

fn print_lines<R: BufRead>(reader: R, num_lines: u32) {
    // Loop through each line of the file,
    // breaking when we output the desired number of lines
    let mut count = num_lines;
    for line in reader.lines() {
        println!("{}", line.unwrap());
        count -= 1;
        if count == 0 {
            break;
        }
    }
}

fn print_chars<R: BufRead>(reader: R, num_chars: u32) {
    let mut count = num_chars;
    // Not efficient but gets the job done
    for byte in reader.bytes() {
        print!("{}", byte.unwrap() as char);
        count -= 1;
        if count == 0 {
            return;
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let stdin = std::io::stdin();
    let filename = args.file;

    let count = args.number;
    let chars = args.chars;

    if count == 0 {
        eprintln!("Invalid number of lines to print: {}", count);
        return ExitCode::FAILURE;
    }

    let reader: Box<dyn io::Read> = match filename {
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

    match chars {
        0 => print_lines(BufReader::new(reader) , count),
        _ => print_chars(BufReader::new(reader), chars),
    }

    ExitCode::SUCCESS
}
