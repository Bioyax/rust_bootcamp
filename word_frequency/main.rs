use clap::Parser;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The text to process. If not provided, reads from stdin.
    text: Option<String>,

    /// Ignore case when counting words
    #[arg(long)]
    ignore_case: bool,

    /// Minimum length of words to count
    #[arg(long, default_value_t = 1)]
    min_length: usize,

    /// Show only the top N words
    #[arg(short, long)]
    top: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    if let Some(text) = args.text {
        input = text;
    } else if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read from stdin: {}", e);
        return;
    }

    let mut word_counts = HashMap::new();

    for word in input.split_whitespace() {
        // Remove punctuation from the end of the word
        let trimmed_word = word.trim_end_matches(|c: char| !c.is_alphanumeric());

        let processed_word = if args.ignore_case {
            trimmed_word.to_lowercase()
        } else {
            trimmed_word.to_string()
        };

        if processed_word.len() >= args.min_length {
            *word_counts.entry(processed_word).or_insert(0) += 1;
        }
    }

    let mut sorted_counts: Vec<_> = word_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let results = sorted_counts.into_iter();

    if let Some(top_n) = args.top {
        for (word, count) in results.take(top_n) {
            println!("{}: {}", word, count);
        }
    } else {
        for (word, count) in results {
            println!("{}: {}", word, count);
        }
    }
}
