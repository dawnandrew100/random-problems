use rand::Rng;
use std::io;

#[derive(Debug, PartialEq)]
enum Flip {
    Heads,
    Tails,
}

enum Choice {
    Yes,
    No,
}

fn main() {
    let mut response = String::new();
    loop {
        println!("How many times would you like to flip the coin?");
        response.clear();
        io::stdin()
            .read_line(&mut response)
            .expect("Unable to read line");

        let Ok(num_coin_flips) = response.trim().parse::<u32>() else {
            println!(
                "Please enter a valid number between {} and {}.",
                u32::MIN,
                u32::MAX
            );
            continue;
        };
        let chosen_range: Vec<u32> = (0..num_coin_flips)
            .map(|_| rand::rng().random_range(1..=100))
            .collect();

        println!("{:?}", chosen_range);
        let chosen_range: Vec<Flip> = chosen_range
            .iter()
            .map(|&n| if n % 2 == 0 { Flip::Heads } else { Flip::Tails })
            .collect();

        println!("{:?}", chosen_range);

        let num_heads = chosen_range.iter().filter(|&n| *n == Flip::Heads).count();
        let num_tails = chosen_range.iter().filter(|&n| *n == Flip::Tails).count();

        println!("You got {num_heads} heads and {num_tails} tails!");

        println!("\nWould you like to continue?\n1. Yes\n2. No");
        response.clear();
        let decision = loop {
            io::stdin()
                .read_line(&mut response)
                .expect("Unable to read line");

            let Ok(ans) = response.trim().parse::<u8>() else {
                println!("Please enter either 1 or 2!");
                continue;
            };
            match ans {
                1 => break Choice::Yes,
                2 => break Choice::No,
                _ => {
                    println!("Please enter either 1 or 2!");
                    continue;
                }
            }
        };

        match decision {
            Choice::Yes => continue,
            Choice::No => break,
        }
    }
}
