use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let response = loop {
        println!("What number would you like to see fizz buzz up to?");
        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u64>() {
            Ok(num) => break num,
            Err(err) => eprintln!("{err}"),
        }
    };

    (2..=response)
        .map(|n| {
            let s: String = [("Fizz", n % 3), ("Buzz", n % 5)]
                .iter()
                .filter(|(_, rem)| *rem == 0)
                .map(|(word, _)| *word)
                .collect();
            if s.is_empty() { n.to_string() } else { s }
        })
        .for_each(|line| println!("{line}"));

    Ok(())
}
