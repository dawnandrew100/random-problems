use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Welcome to the Band Name Generator.");

    println!("What's the name of the city you grew up in?");
    io::stdin().read_line(&mut input)?;
    let city = input.trim().to_string();

    input.clear();

    println!("What's your pet's name?");
    io::stdin().read_line(&mut input)?;
    let pet_name = input.trim().to_string();
    println!("Your band name could be {city} {pet_name}.");

    Ok(())
}
