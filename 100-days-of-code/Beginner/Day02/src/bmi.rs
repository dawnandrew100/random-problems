use std::io;

pub fn bmi_prompt() -> io::Result<f64> {
    println!("Welcome to Day 2!");
    println!("First, let's calculate your BMI!");

    let mut input = String::new();
    let height = loop {
        input.clear();
        println!("What's your height in metres?");
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => eprintln!("{}", err),
        }
    };

    let width = loop {
        input.clear();
        println!("What's your weight in kilograms?");
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => eprintln!("{}", err),
        }
    };

    let bmi = bmi_calc(width, height);

    Ok(bmi)
}

fn bmi_calc(w: f64, h: f64) -> f64 {
    w / (h * h)
}
