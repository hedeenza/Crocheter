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
