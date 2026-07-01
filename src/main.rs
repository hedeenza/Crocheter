use clap::Parser;
use crocheter::{
    create_output_file, group_paragraphs, paragraph_vectors_to_strings, partition_skipped_lines,
    read_file_to_vec, write_to_output,
};

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
    // Parse command line arguments
    let args = Cli::parse();

    // Read input file to a vector
    let input_lines: Vec<String> = read_file_to_vec(&args.input);

    // Partition any lines the user wants to skip into its own vector
    let skipped_lines: Vec<String> = partition_skipped_lines(input_lines.clone(), args.skip_lines);

    // Group paragraphs by line breaks between content
    let all_paragraphs: Vec<Vec<String>> = group_paragraphs(input_lines.clone(), args.skip_lines);

    // Collapse paragraphs down to one string each
    let paragraph_strings: Vec<String> = paragraph_vectors_to_strings(all_paragraphs);

    // Create the output file to write to
    let output_file = create_output_file(&args.input);

    // Write to the output file
    write_to_output(
        args.tab_indentation,
        output_file,
        skipped_lines,
        paragraph_strings,
    );
}
