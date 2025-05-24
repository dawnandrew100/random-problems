use std::io;

enum Choice {
    Yes,
    No,
}

fn main() {
    let mut input = String::new();
    loop {
        let num: u32;
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
                    u32::MIN,
                    u32::MAX,
                );
                continue;
            }
        };

        let primes_to_num = prime_factor_faux_sieve(num);
        println!("{:?}\n", find_prime_factors(primes_to_num, num));

        let answer = loop {
            println!("Would you like to continue?\n1. yes\n2. no");

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

fn prime_factor_faux_sieve(target: u32) -> Vec<u32> {
    let upper_limit = (target as f64).sqrt() as u32;
    let poss_primes: Vec<u32> = (2..=upper_limit + 1)
        .filter(|&n| n == 2 || n % 2 != 0)
        .collect();

    let primes: Vec<u32> = poss_primes
        .iter()
        .cloned()
        .filter(|&n| {
            poss_primes
                .iter()
                .filter(|&&x| x != n)
                .take_while(|&&x| x * x <= n)
                .all(|&x| n % x != 0)
        })
        .collect();
    return primes;
}

fn find_prime_factors(primes: Vec<u32>, target: u32) -> Vec<u32> {
    let mut remainder = target;
    let mut prime_factors = Vec::new();
    for num in primes {
        while remainder % num == 0 {
            prime_factors.push(num);
            remainder = remainder / num;
        }
    }
    if remainder != 1 {
        prime_factors.push(remainder);
    }
    return prime_factors;
}
