use std::io;

enum Choice {
    Yes,
    No,
}

fn main() {
    let mut input = String::new();
    loop {
        let num: u64;
        println!("Which number would you like to find the prime factors of?");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(number) => num = number,
            Err(_) => {
                println!(
                    "Please enter a valid number between {} and {}.",
                    u64::MIN,
                    u64::MAX,
                );
                continue;
            }
        };

        println!("{:?}\n", find_prime_factors(num));

        let answer = loop {
            println!("\nWould you like to continue?\n1. yes\n2. no");

            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let Ok(ans) = input.trim().parse::<u8>() else {
                println!("Please enter 1 or 2!");
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
        match answer {
            Choice::Yes => continue,
            Choice::No => break,
        }
    }
}

fn find_prime_factors(number: u64) -> Vec<u64> {
    let mut n = number;
    let mut factor = 2;
    let mut prime_factors: Vec<u64> = Vec::new();

    while factor * factor <= n {
        if n % factor == 0 {
            while n % factor == 0 {
                prime_factors.push(factor);
                n /= factor;
            }
        }
        factor += if factor == 2 { 1 } else { 2 };
    }

    if n > 1 {
        prime_factors.push(n);
    }
    return prime_factors;
}
