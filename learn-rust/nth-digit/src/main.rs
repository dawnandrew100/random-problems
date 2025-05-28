use std::{fmt, io, str::FromStr};

fn main() -> std::io::Result<()> {
    let pi = std::f64::consts::PI.to_string();
    let e = std::f64::consts::E.to_string();
    let tau = std::f64::consts::TAU.to_string();

    let mut input = String::new();
    loop {
        let constant;
        println!("Would you like to see pi, e, or tau?");
        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<Irrational>() {
            Ok(c) => constant = c,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
        let mut digits = loop {
            println!("How many digits of {constant} would you like to see?");
            input.clear();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 && num <= 16 => break num,
                Ok(_) => {
                    println!("Enter a number from 1 to 16!");
                    continue;
                }
                Err(err) => {
                    eprintln!("{err}");
                    continue;
                }
            }
        };
        if digits > 1 {
            digits = digits + 1;
        }
        match constant {
            Irrational::Pi => println!("{}", &pi[..digits]),
            Irrational::E => println!("{}", &e[..digits]),
            Irrational::Tau => println!("{}", &tau[..digits]),
        }
        let decision = loop {
            println!("\nStart over?\n1. Yes\n2. No\n");
            input.clear();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<Choice>() {
                Ok(c) => break c,
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

#[derive(Clone, Copy)]
enum Choice {
    Yes,
    No,
}

#[derive(Clone, Copy)]
enum Irrational {
    Pi,
    E,
    Tau,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "1" | "y" | "yes" => Ok(Choice::Yes),
            "2" | "n" | "no" => Ok(Choice::No),
            _ => Err("Please enter either Yes or No"),
        }
    }
}

impl FromStr for Irrational {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "1" | "p" | "pi" => Ok(Irrational::Pi),
            "2" | "e" => Ok(Irrational::E),
            "3" | "t" | "tau" => Ok(Irrational::Tau),
            _ => Err("Please enter Pi, E, or Tau"),
        }
    }
}

impl fmt::Display for Irrational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Irrational::Pi => write!(f, "pi"),
            Irrational::E => write!(f, "e"),
            Irrational::Tau => write!(f, "tau"),
        }
    }
}
