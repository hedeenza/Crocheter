use clap::Parser;
use crocheter::{read_file_to_vec, partition_skipped_lines, group_paragraphs, paragraph_vectors_to_strings, create_output_file, write_to_output};


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

    let output_file = create_output_file(&args.input);

    write_to_output(args.tab_indentation, output_file, skipped_lines, paragraph_strings);
}
