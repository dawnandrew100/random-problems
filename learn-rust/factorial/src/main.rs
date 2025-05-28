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
            "1" | "y" | "yes" => Ok(Choice::Yes),
            "2" | "n" | "no" => Ok(Choice::No),
            _ => Err("Please enter yes or no!"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    loop {
        let input_num = loop {
            println!("Enter the number you want to find the factorial of!");
            input.clear();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<u64>() {
                Ok(num) if num <= 20 => break num,
                Ok(_) => {
                    println!("Please enter a positive number up to 20!");
                    continue;
                }
                Err(err) => {
                    eprintln!("{err}");
                    continue;
                }
            }
        };
        let result = get_factorial(input_num);
        println!("{result}");

        let decision = loop {
            println!("Would you like to find another factorial?\n1. Yes\n2. No");
            input.clear();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<Choice>() {
                Ok(choice) => break choice,
                Err(err) => {
                    eprintln!("{err}");
                    continue;
                }
            }
        };
        match decision {
            Choice::Yes => continue,
            Choice::No => break Ok(()),
        }
    }
}

fn get_factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        return 1;
    }
    (2..=num).product()
}
