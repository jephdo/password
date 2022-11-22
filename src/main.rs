use clap::{Parser, Subcommand};
use password::{generate_pin, generate_random, generate_xkcd};

fn validate_separator(sep: &str) -> Result<(), String> {
    let chars: Vec<char> = sep.chars().collect();

    if let Some(ch) = chars.get(0) {
        match ch {
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '-' | '_' | '+' | '=' | ':' | '|'
            | '~' | '?' | '/' | '.' | ';' => Ok(()),
            _ => Err(String::from(
                "Separator must be one of '!@#$%^&*'-_+=:|~?.;",
            )),
        }
    } else {
        Ok(())
    }
}

/// Simple random password generator
#[derive(Parser, Debug)]
struct Arguments {
    #[clap(subcommand)]
    passwordtype: Password,
}

#[derive(Subcommand, Debug)]
enum Password {
    /// Generate a memorable password made of random words
    Xkcd {
        /// Number of words to randomly generate
        #[clap(short, long, default_value_t = 3)]
        num_words: usize,
        /// Separator between each word
        #[clap(short, long, default_value_t = String::from(""), validator = validate_separator)]
        separator: String,
        /// Randomly choose one word to be capitalized
        #[clap(short, long, action)]
        capitalize: bool,
    },
    /// Generate a password with completely random characters
    Random {
        /// Password length
        #[clap(short, long, default_value_t = 12)]
        length: usize,
    },
    /// Generate a random pin of numbers
    Pin {
        /// Pin length
        #[clap(short, long, default_value_t = 4)]
        length: usize,
    },
}

fn main() {
    let args = Arguments::parse();

    let password = match args.passwordtype {
        Password::Xkcd {
            num_words,
            separator,
            capitalize,
        } => generate_xkcd(num_words, &separator, capitalize),
        Password::Random { length } => generate_random(length),
        Password::Pin { length } => generate_pin(length),
    };

    println!("{}", password);
}
