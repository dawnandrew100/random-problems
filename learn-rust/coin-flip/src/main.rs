use rand::{Rng, distr::slice::Choose};
use std::{io, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Flip {
    Heads,
    Tails,
}

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
            _ => Err("Please enter either Yes or No"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let flip_picker = Choose::new(&[Flip::Heads, Flip::Tails]).expect("Length is not zero!");
    let mut response = String::new();
    loop {
        println!("How many times would you like to flip the coin?");
        response.clear();
        io::stdin().read_line(&mut response)?;

        let num_coin_flips = match response.trim().parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        /*
        rand::rng().sample_iter(&flip_picker) creates an infinite stream
        of choices from the flip picker.
        .take(num_coin_flips) limits the iterator to the first n values
        .copied() turns the enum references into values
        (so the vector is Vec<Flip> instead of Vec<&Flip>)
        .collect() turns the consumes the iterator and turns it into the vector
         */
        let chosen_range: Vec<Flip> = rand::rng()
            .sample_iter(&flip_picker)
            .take(num_coin_flips)
            .copied()
            .collect();

        println!("{:?}", chosen_range);

        let num_heads = chosen_range.iter().filter(|&n| *n == Flip::Heads).count();
        let num_tails = chosen_range.iter().filter(|&n| *n == Flip::Tails).count();

        println!("You got {num_heads} heads and {num_tails} tails!");

        let decision = loop {
            println!("\nWould you like to continue?\n1. Yes\n2. No");
            response.clear();
            io::stdin().read_line(&mut response)?;

            match response.trim().parse::<Choice>() {
                Ok(choice) => break choice,
                Err(err) => eprintln!("{}", err),
            };
        };

        match decision {
            Choice::Yes => continue,
            Choice::No => break Ok(()),
        }
    }
}
