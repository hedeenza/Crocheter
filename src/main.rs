use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


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

    // Create a new vector to hold the formatted paragraphs
    let mut paragraph_strings: Vec<String> = Vec::new();

    // For each paragraph in the input vector
    for paragraph in all_paragraphs {
        // Add a tab indentation to start each paragraph
        let mut base_string = String::new();
        // Join the paragraph's sentences with a space
        let sentences = paragraph.join(" ");
        // Add the sentence onto the starting tab indentation
        base_string.push_str(&sentences);
        // Add the whole paragraph onto the paragraph vector
        paragraph_strings.push(base_string);
    }

    let input_file = args.input;
    // Find the position of the period in the input file name, if there is one
    let period_index = match input_file.find(".") {
        Some(index) => index,
        None => input_file.len(),
    };
    // The output name is everything up to the period index
    let input_name = &input_file[..period_index];
    // The output extension is everything after and including the period index
    let input_extension = &input_file[period_index..];
    // Format the output name to include the original file name
    // and original extension if there was one
    let output_name = format!("{}_publish{}", input_name, input_extension);
    // Create the output file
    let mut output_file =
        File::create(output_name).expect("Failed to Create Output File");
    for paragraph in paragraph_strings {
        let _ = writeln!(output_file, "{}\n", paragraph);
    }
}
