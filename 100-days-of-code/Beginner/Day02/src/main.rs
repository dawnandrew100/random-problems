use std::io;

mod bmi;
mod tip;
use bmi::bmi_prompt;
use tip::tip_prompt;

fn main() -> io::Result<()> {
    let bmi = bmi_prompt();
    match bmi {
        Ok(num) => println!("Your bmi is {:.2}", num),
        Err(err) => eprintln!("Unable to calculate bmi due to error: {}", err),
    }
    let (tipped_total, people_num) = tip_prompt()?;
    println!("Your total factoring in a tip is ${:.2}", tipped_total);
    if people_num > 1.0 {
        println!(
            "Split between {people_num} people, each person would pay ${:.2}",
            tipped_total / people_num
        );
    }

    Ok(())
}
