use std::io;

fn main() {
    let mut side_a: u32;
    let mut side_b: u32;
    let mut side_c: u32;

    loop {
        println!("Enter the sides of your triangle separated by a space!");
        let mut triange_edges = String::new(); // Empty string to put user input

        io::stdin()
            .read_line(&mut triange_edges)
            .expect("Failed to read line");

        let sides: Vec<&str> = triange_edges.trim().split_whitespace().collect();

        if sides.len() != 3 {
            println!("Please enter exactly three integers separated by spaces!");
            continue;
        }

        // Parses string into u32, err result wrapper
        let a_val: Result<u32, _> = sides[0].parse();
        let b_val: Result<u32, _> = sides[1].parse();
        let c_val: Result<u32, _> = sides[2].parse();

        let mut invalid_inputs = Vec::new(); // Array for error results

        if a_val.is_err() {
            invalid_inputs.push(sides[0]);
        }
        if b_val.is_err() {
            invalid_inputs.push(sides[1]);
        }
        if c_val.is_err() {
            invalid_inputs.push(sides[2]);
        }

        if !invalid_inputs.is_empty() {
            println!(
                "Invalid inputs: {}. Please only enter positive integers!",
                invalid_inputs.join(", ")
            );
            continue;
        }

        // Gets result from result wrapper
        side_a = a_val.unwrap();
        side_b = b_val.unwrap();
        side_c = c_val.unwrap();

        if side_a == 0 || side_b == 0 || side_c == 0 {
            println!("All sides must be greater than 0! You entered: {side_a}, {side_b}, {side_c}");
            continue;
        }
        break;
    }

    println!(
        "This triangle is {}!",
        triangle_type(&side_a, &side_b, &side_c)
    );
}

fn triangle_type(a: &u32, b: &u32, c: &u32) -> &'static str {
    if a == b && b == c {
        return "Equilateral";
    } else if a == b || b == c || a == c {
        return "Isosceles";
    } else {
        return "Scalene";
    }
}
