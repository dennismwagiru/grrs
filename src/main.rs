#![allow(unused)]

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let f = File::open(&args.path).expect("Could not read file");
    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let line_text = line.unwrap();
        if line_text.contains(&args.pattern) {
            println!("{}", line_text);
        }
    }
}
