use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_vec(input_file: &String) -> Vec<String> {
    // Create a vector to contain the original file contents
    let mut input_lines: Vec<String> = Vec::new();

    // Read in the input file
    let file = File::open(&input_file);
    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {}", err),
    };

    // Push each line to the vector
    for line in file_reader.lines() {
        match line {
            Ok(line) => input_lines.push(line),
            Err(err) => println!("[ ERROR ]: {}", err),
        }
    }
    input_lines
}

pub fn partition_skipped_lines(input_lines: Vec<String>, lines_skipped: usize) -> Vec<String> {
    // Creata a new vector to hold the lines we want to skip formatting
    let mut skipped_lines: Vec<String> = Vec::new();

    // Push each skipped line to the vector
    for line in &input_lines[0..lines_skipped] {
        skipped_lines.push(line.to_string());
    }
    skipped_lines
}
