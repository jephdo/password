use clap::Parser;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::io::BufRead;

/// Simple random password generator
#[derive(Parser)]
struct Cli {
    /// Number of words to randomly generate
    #[arg(short, long, default_value_t = 3)]
    num_words: usize,
    /// Separator between each word
    #[arg(short, long, default_value_t = String::from("-"))]
    separator: String,
    /// Randomly choose one word to be capitalized
    #[arg(short, long, action)]
    capslock: bool,
}

fn read_dictionary() -> Vec<String> {
    let file = std::fs::File::open("dictionary.txt").expect("No such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args = Cli::parse();
    let lines = read_dictionary();
    let num = rand::thread_rng().gen_range(0..args.num_words);

    let words: Vec<String> = lines
        .choose_multiple(&mut rand::thread_rng(), args.num_words)
        .cloned()
        .enumerate()
        .map(|(i, w)| {
            if args.capslock & (i == num) {
                w.to_uppercase()
            } else {
                w
            }
        })
        .collect();

    println!("{}", words.join(&args.separator))
}
