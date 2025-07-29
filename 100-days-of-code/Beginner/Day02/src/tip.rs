use std::io;

pub fn tip_prompt() -> io::Result<(f64, f64)> {
    println!("Welcome to the tip calculator!");
    let mut ans = String::new();

    let total_bill = parse_input("What was your total bill?", &mut ans)?;
    let tip_percent = parse_input("What percentage would you like to tip?", &mut ans)?;
    let people_num = loop {
        match parse_input(
            "How many people are going to be splitting the bill?",
            &mut ans,
        ) {
            Ok(num) => {
                if num == num as usize as f64 && num > 0.0 {
                    break num;
                } else {
                    eprintln!("Please enter a positive whole number.")
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    };

    let tipped_total = calc_tip(total_bill, tip_percent);

    Ok((tipped_total, people_num))
}

fn parse_input(prompt: &str, input: &mut String) -> io::Result<f64> {
    let res = loop {
        input.clear();
        println!("{}", prompt);
        io::stdin().read_line(input)?;
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(err) => eprintln!("{}", err),
        }
    };
    Ok(res)
}

fn calc_tip(total: f64, tip: f64) -> f64 {
    let multiplier = 1 as f64 + (tip / 100 as f64);
    return total * multiplier;
}
