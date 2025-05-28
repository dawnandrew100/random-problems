use std::io;

fn main() {
    let mut input = String::new();
    let num = loop {
        println!("Enter a number to see how many steps it takes to get to 1!");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        match input.trim().parse::<usize>() {
            Ok(n) => break n,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        }
    };
    println!("{num} takes {} steps to get back to 1!", collatz(num));
}

fn collatz(num: usize) -> usize {
    let mut steps = 0;
    let mut curr = num;
    while curr != 1 {
        if curr % 2 == 0 {
            curr = curr / 2;
        } else {
            curr = 3 * curr + 1;
        }
        steps = steps + 1;
    }
    return steps;
}
