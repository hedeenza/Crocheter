use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();

    // Read in the input file
    let file = File::open(&args.input);
    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {}", err),
    };

    // Create a vector to contain the original file contents
    let mut input_lines: Vec<String> = Vec::new();

    // Push each line to the vector
    for line in file_reader.lines() {
        match line {
            Ok(line) => input_lines.push(line),
            Err(err) => println!("[ ERROR ]: {}", err),
        }
    }
}
