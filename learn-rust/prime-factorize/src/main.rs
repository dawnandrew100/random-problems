use std::io;

fn main() {
    let num: u32 = loop {
        println!("Which number would you like to find the prime factors of?");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim().parse() {
            Ok(number) => break number,
            Err(_) => {
                println!(
                    "Please enter a valid number between {} and {}",
                    u32::MIN,
                    u32::MAX
                );
                continue;
            }
        };
    };
    let primes_to_num = prime_factor_sieve(num);
    println!("{:?}", find_prime_factors(primes_to_num, num));
}

fn prime_factor_sieve(target: u32) -> Vec<u32> {
    let upper_limit = (target as f64).sqrt() as u32;
    let poss_primes: Vec<u32> = (2..=upper_limit + 1)
        .filter(|&n| n == 2 || n % 2 != 0)
        .collect();

    let primes: Vec<u32> = poss_primes
        .iter()
        .cloned()
        .filter(|&n| poss_primes.iter().filter(|&&x| x != n).all(|&x| n % x != 0))
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
