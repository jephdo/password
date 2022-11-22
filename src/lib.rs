use rand::distributions::{Alphanumeric, Uniform};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::io::BufRead;

fn read_dictionary() -> Vec<String> {
    let file = std::fs::File::open("dictionary.txt").expect("No such file");
    let buf = std::io::BufReader::new(file);
    buf.lines().map(|line| line.unwrap()).collect()
}

pub fn generate_xkcd(num_words: usize, separator: &str, capitalize: bool) -> String {
    let lines = read_dictionary();

    let mut words: Vec<String> = lines
        .choose_multiple(&mut rand::thread_rng(), num_words)
        .cloned()
        .collect();
    if capitalize {
        let n = rand::thread_rng().gen_range(0..num_words);
        words[n] = words[n].to_uppercase();
    }

    words.join(&separator)
}

pub fn generate_pin(length: usize) -> String {
    thread_rng()
        .sample_iter(&Uniform::from(0..10))
        .take(length)
        .map(|d| char::from_digit(d, 10).unwrap())
        .collect()
}

pub fn generate_random(length: usize) -> String {
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#create-random-passwords-from-a-set-of-alphanumeric-characters
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
