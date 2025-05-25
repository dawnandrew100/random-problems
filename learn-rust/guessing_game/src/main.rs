use rand::Rng;
use std::cmp::Ordering;
use std::{io, str::FromStr};

#[derive(Clone, Copy)]
enum Choice {
    Yes,
    No,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "y" | "yes" | "1" => Ok(Choice::Yes),
            "n" | "no" | "2" => Ok(Choice::No),
            _ => Err("Please enter either Yes or No."),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut response = String::new();
    let mut num_guesses: u32 = 0;
    let min = 1;
    let max = 100;

    println!("\nA new secret number has been chosen!");
    println!("Guess the number from {min} to {max}!\n");

    loop {
        let secret_number = rand::rng().random_range(min..=max);

        loop {
            println!("Please input your guess!");
            response.clear();
            io::stdin().read_line(&mut response)?;

            let guess: usize;
            match response.trim().parse::<usize>() {
                Ok(num) if num >= min && num <= max => guess = num,
                Ok(_) => {
                    eprintln!("Number entered not within range {min}-{max}!");
                    continue;
                }
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            println!("You guessed: {guess}");
            num_guesses += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("\nToo small!\n"),
                Ordering::Greater => println!("\nToo big!\n"),
                Ordering::Equal => {
                    println!("\nYou win!");
                    println!("You guessed the secret number in {num_guesses} guesses!\n");
                    break;
                }
            }
        }
        let decision = loop {
            println!("\nWould you like to guess again?\n1. Yes\n2. No");
            response.clear();
            io::stdin().read_line(&mut response)?;
            match response.trim().parse::<Choice>() {
                Ok(choice) => break choice,
                Err(err) => eprintln!("{}", err),
            }
        };

        match decision {
            Choice::Yes => {
                num_guesses = 0;
                continue;
            }
            Choice::No => break Ok(()),
        }
    }
}
