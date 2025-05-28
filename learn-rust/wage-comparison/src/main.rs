use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut base_rate;
    let mut base_hours;
    let mut compare_rate;

    loop {
        base_rate = loop {
            println!("Enter your rate to compare against");
            input.clear();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(err) => eprintln!("{err}"),
            }
        };

        base_hours = loop {
            println!("Enter your hours worked at this job");
            input.clear();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(err) => eprintln!("{err}"),
            }
        };

        compare_rate = loop {
            println!("Enter your second rate");
            input.clear();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(err) => eprintln!("{err}"),
            }
        };

        let decision = loop {
            println!(
                "You entered your base rate as {base_rate} \
            and your hours at that job as {base_hours}.\nYou also \
            entered your rate at the second job as {compare_rate}."
            );
            println!("\nIs this correct?\n1. Yes\n2. No");
            input.clear();
            io::stdin().read_line(&mut input)?;
            match input.trim().parse::<Choice>() {
                Ok(choice) => break choice,
                Err(err) => eprintln!("{err}"),
            }
        };

        match decision {
            Choice::Yes => break,
            Choice::No => continue,
        }
    }
    let total = base_rate * base_hours;
    let second_hours = total / compare_rate;

    println!("You're making ${total:.2} total at the first job!");
    println!(
        "To make the same amount, you'd need to work {second_hours:.2} hrs at the second job!"
    );

    Ok(())
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
            _ => Err("Please enter either Yes or No."),
        }
    }
}
