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

    // Createa a new vector to hold the Vec<Tuple>
    let mut all_paragraphs: Vec<Vec<String>> = Vec::new();

    // Set the initial Pointer 1 index to the beginning
    let mut pointer1_index = 0;

    // While the Pointer 1 index is less than the length of the content vector...
    while pointer1_index < input_lines.len() {
        // If Pointer 1 hits a line with content in it...
        if input_lines[pointer1_index].trim() != String::from("") {
            // Create a new vector to hold the (index, line-content) tuples
            let mut paragraph_vector: Vec<String> = Vec::new();
            // Set Pointer 2 to where Pointer 1 is
            let mut pointer2_index = pointer1_index;
            // While the Pointer 2 index is less than the length of the content vector...
            while pointer2_index < input_lines.len() {
                // If Pointer 2 hits a blank line...
                if input_lines[pointer2_index].trim() == String::from("") {
                    // Push each line between Pointer 1 and Pointer 2 to a Vector
                    for line in &mut input_lines[pointer1_index..pointer2_index] {
                        paragraph_vector.push(line.to_string());
                        // pointer1_index += 1;
                    }
                    pointer1_index = pointer2_index;
                    break
                }
                // Increment Pointer 2 by One
                pointer2_index += 1;
            }
            // Push the tuple vector to the output vector
            all_paragraphs.push(paragraph_vector);
        }
        // Increment Pointer 1 by One
        pointer1_index += 1;
    }

    for vector in all_paragraphs {
        println!("{:?}", vector);
    }

}
