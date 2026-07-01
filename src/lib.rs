#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[must_use]
/// # Panics
///
/// Will panic if it cannot open the file
/// or if there is an error processing a buffered line
pub fn read_file_to_vec(input_file: &String) -> Vec<String> {
    // Create a vector to contain the original file contents
    let mut input_lines: Vec<String> = Vec::new();

    // Read in the input file
    let file = File::open(input_file);
    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {err}"),
    };

    // Push each line to the vector
    for line in file_reader.lines() {
        match line {
            Ok(line) => input_lines.push(line),
            Err(err) => println!("[ ERROR ]: {err}"),
        }
    }
    input_lines
}

#[must_use]
pub fn partition_skipped_lines(input_lines: &[String], lines_skipped: usize) -> Vec<String> {
    // Creata a new vector to hold the lines we want to skip formatting
    let mut skipped_lines: Vec<String> = Vec::new();

    // Push each skipped line to the vector
    for line in &input_lines[0..lines_skipped] {
        skipped_lines.push(line.clone());
    }
    skipped_lines
}

#[must_use]
pub fn group_paragraphs(input_lines: &[String], lines_skipped: usize) -> Vec<Vec<String>> {
    // Create a new vector to hold the Vec<Tuple>
    let mut all_paragraphs: Vec<Vec<String>> = Vec::new();

    // Set the initial Pointer 1 index to the beginning
    let mut pointer1_index = lines_skipped;

    // While the Pointer 1 index is less than the length of the content vector...
    while pointer1_index < input_lines.len() {
        // If Pointer 1 hits a line with content in it...
        if input_lines[pointer1_index].trim() != "" {
            // Create a new vector to hold the (index, line-content) tuples
            let mut paragraph_vector: Vec<String> = Vec::new();
            // Set Pointer 2 to where Pointer 1 is
            let mut pointer2_index = pointer1_index;
            // While the Pointer 2 index is less than the length of the content vector...
            while pointer2_index < input_lines.len() {
                // If Pointer 2 hits a blank line...
                if input_lines[pointer2_index].trim() == "" {
                    // Push each line between Pointer 1 and Pointer 2 to a Vector
                    for line in &input_lines[pointer1_index..pointer2_index] {
                        paragraph_vector.push(line.clone());
                    }
                    pointer1_index = pointer2_index;
                    break;
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
    all_paragraphs
}

#[must_use]
pub fn paragraph_vectors_to_strings(all_paragraphs: Vec<Vec<String>>) -> Vec<String> {
    // Create a new vector to hold the formatted paragraphs
    let mut paragraph_strings: Vec<String> = Vec::new();

    // For each paragraph in the input vector
    for paragraph in all_paragraphs {
        // Join the paragraph's sentences with a space
        let sentences = paragraph.join(" ");
        // Add the whole paragraph onto the paragraph vector
        paragraph_strings.push(sentences);
    }
    paragraph_strings
}

#[must_use]
/// # Panics
///
/// Will panic if it cannot create the output file
pub fn create_output_file(input_file: &str) -> File {
    // Find the position of the period in the input file name, if there is one
    let period_index = match input_file.find('.') {
        Some(index) => index,
        None => input_file.len(),
    };
    // The output name is everything up to the period index
    let input_name = &input_file[..period_index];
    // The output extension is everything after and including the period index
    let input_extension = &input_file[period_index..];
    // Format the output name to include the original file name
    // and original extension if there was one
    let output_name = format!("{input_name}_publish{input_extension}");
    // Create the output file and return the output
    File::create(output_name).expect("Failed to Create Output File")
}

pub fn write_to_output(
    tab_indentation: bool,
    mut output_file: File,
    skipped_lines: Vec<String>,
    paragraph_strings: Vec<String>,
) {
    // Write the skipped lines exactly as they were
    for line in skipped_lines {
        let _ = writeln!(output_file, "{line}");
    }

    // Write a blank line between the skipped lines and the remaining
    let _ = writeln!(output_file);

    // Write the formatted paragraphs
    for paragraph in paragraph_strings {
        if tab_indentation {
            let _ = writeln!(output_file, "\t{paragraph}\n");
        } else {
            let _ = writeln!(output_file, "{paragraph}\n");
        }
    }
}
