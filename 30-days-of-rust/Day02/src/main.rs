use std::io;

fn main() -> io::Result<()> {
    // Exercise 1
    let my_age = 27;
    println!("{my_age}");
    let mut my_height = 177;
    println!("{my_height}");
    my_height = 178;
    println!("{my_height}");
    let my_name = "Andrew";
    println!("{my_name}");
    let is_student = true;
    println!("{is_student}");
    let birth_year = 2025 - my_age;
    println!("{birth_year}");

    // Exercise 2
    let my_integer = 1;
    let my_float = 1.0;
    println!("Integer = {my_integer}\nFloat = {my_float}");
    let is_learning_rust = true;
    println!("{is_learning_rust}");
    let favourite_letter = 'a';
    println!("{favourite_letter}");
    let my_scores = [98, 88, 99, 100, 96];
    println!("{my_scores:?}");
    let hobby = "programming";
    println!("{hobby}");
    println!("I enjoy {hobby}");

    Ok(())
}
