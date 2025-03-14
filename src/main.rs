use clap::Parser;
use dotenvy::dotenv;
use std::env;

fn main() {
    const RADIX: u32 = 10;
    dotenv().ok();

    let dictionary = env::var("DICTIONARY").expect("DICTIONARY must be set.");

    let options = Options::parse();
    let mut message = options.message;

    if message.chars().all(char::is_alphanumeric) {
        message = message
            .chars()
            .map(|x| match x {
                '0'..='9' => {
                    let index = x.to_digit(RADIX).unwrap();
                    dictionary.chars().nth(index as usize).unwrap()
                }
                _ => x,
            })
            .collect();
    } else {
        message = "I can flip numbers only.".to_string();
    }

    println!("{}", message.chars().rev().collect::<String>());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o.o )");
    println!("    =( I )=");
}

#[derive(Parser)]
struct Options {
    #[clap(default_value = "07734")]
    /// What the number says?
    message: String,
}
