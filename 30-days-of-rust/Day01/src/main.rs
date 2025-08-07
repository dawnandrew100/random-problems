use std::io;

fn main() -> io::Result<()> {
    println!("Hello, Rust!");

    println!("What's my name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let my_name = input.trim().to_string();
    excitement_reason(my_name);

    // Exercise 1
    println!("I can print 'Hello, World'!");

    // Exercise 2
    let x = 5;
    println!("{x}");
    let mut x = x; // shadows immutable x with mutable binding
    x = 10;
    println!("{x}");

    let float: f32 = 3.04;
    let boolean: bool = true;
    let string: &'static str = "my string";

    println!(
        "Some of the types I can print are\na float: {}\nbool: {}\nand string: {}",
        float, boolean, string
    );

    Ok(())
}

fn excitement_reason(name: String) {
    let reason = "I'm excited to learn Rust because it's fast and reliable";
    println!("Hello, my name is {} and {}.", name, reason);
}
