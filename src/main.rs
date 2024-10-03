//! A simple implementation of the `head` command in Rust.
//! # Examples:
//! ```shell
//! $ cargo run -- -n 5 /etc/passwd
//! $ cargo run -- -n 5 < /etc/passwd
//! $ cargo run -- -c 3 < myfile.txt
//! ```

use clap::Parser;
use std::io::{BufRead, BufReader, Write};
use std::vec;
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

fn print_chars<R: BufRead>(mut reader: R, n: u32) {
    let n = n as usize;
    let buffer_size = n.min(4096);
    let mut buffer = vec![0; buffer_size];
    let mut bytes_remaining = n;

    while bytes_remaining > 0 {
        let bytes_to_read = bytes_remaining.min(buffer.len());
        match reader.read(&mut buffer[..bytes_to_read]) {
            Ok(0) => break,
            Ok(bytes_read) => {
                // This has the possibilty of printing invalid UTF-8 characters
                // But it should not crash.
                io::stdout().write_all(&buffer[..bytes_read]).unwrap();
                bytes_remaining -= bytes_read;
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
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

    let mut br = BufReader::new(reader);

    match chars {
        0 => print_lines(&mut br, count),
        _ => print_chars(&mut br, chars),
    }

    ExitCode::SUCCESS
}
