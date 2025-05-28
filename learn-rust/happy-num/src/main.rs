use std::io;

fn main() {
    let mut input = String::new();
    let upper_limit = loop {
        println!("How many happy numbers would you like to find?");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match input.trim().parse::<u64>() {
            Ok(num) => break num,
            Err(err) => {
                eprintln!("{err}! Please enter a valid positive integer!");
                continue;
            }
        }
    };
    let mut happy_nums = Vec::new();
    let mut num = 1;
    while happy_nums.len() < upper_limit as usize {
        let (vector, is_happy) = is_happy_num(num);
        if is_happy {
            happy_nums.push(num);
            println!("{}. {:?}", happy_nums.len(), vector);
        }
        num = num + 1;
    }
    println!("{:?}", happy_nums);
}

fn is_happy_num(curr: u64) -> (Vec<u64>, bool) {
    let mut journey: Vec<u64> = Vec::new();
    journey.push(curr as u64);
    if curr == 1 {
        return (journey, true);
    }

    let mut temp_curr = curr as u64;
    loop {
        temp_curr = square_digits_sum(temp_curr);
        if temp_curr == 1 || journey.contains(&temp_curr) {
            journey.push(temp_curr);
            break;
        }
        journey.push(temp_curr);
    }
    if journey.last() == Some(&1) {
        return (journey, true);
    } else {
        return (journey, false);
    }
}

fn square_digits_sum(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|c| {
            let digit = c.to_digit(10).unwrap() as u64;
            digit * digit
        })
        .fold(0u64, |acc, x| {
            acc.checked_add(x).expect("Sum overflowed u64")
        })
}
