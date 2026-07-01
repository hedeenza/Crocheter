use clap::Parser;
use crocheter::{read_file_to_vec, partition_skipped_lines, group_paragraphs, paragraph_vectors_to_strings};
use std::fs::File;
use std::io::Write;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: String,
    
    /// Tab-indentation for paragraphs
    #[arg(short, long, default_value_t = false)]
    tab_indentation: bool,
    
    /// Number of lines to skip
    #[arg(short, long, default_value_t = 0)]
    skip_lines: usize,
}

fn main() {
    let args = Cli::parse();

    let input_lines: Vec<String> = read_file_to_vec(&args.input);

    let skipped_lines: Vec<String> = partition_skipped_lines(input_lines.clone(), args.skip_lines);
    
    let all_paragraphs: Vec<Vec<String>> = group_paragraphs(input_lines.clone(), args.skip_lines);

    let paragraph_strings: Vec<String> = paragraph_vectors_to_strings(all_paragraphs);
    // // Create a new vector to hold the formatted paragraphs
    // let mut paragraph_strings: Vec<String> = Vec::new();
    //
    // // For each paragraph in the input vector
    // for paragraph in all_paragraphs {
    //     // Add a tab indentation to start each paragraph
    //     let mut base_string = String::new();
    //     // Join the paragraph's sentences with a space
    //     let sentences = paragraph.join(" ");
    //     // Add the sentence onto the starting tab indentation
    //     base_string.push_str(&sentences);
    //     // Add the whole paragraph onto the paragraph vector
    //     paragraph_strings.push(base_string);
    // }

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

    // Write the skipped lines exactly as they were
    for line in skipped_lines {
        let _ = writeln!(output_file, "{}", line);
    }

    // Write a blank line between the skipped lines and the remaining
    let _ = writeln!(output_file);

    // Write the formatted paragraphs
    for paragraph in paragraph_strings {
        if args.tab_indentation {
            let _ = writeln!(output_file, "\t{}\n", paragraph);
        } else {
            let _ = writeln!(output_file, "{}\n", paragraph);
        }
    }
}
