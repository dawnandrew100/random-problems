use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    let user_num = loop {
        println!("Enter a number to translate to words!");
        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(num) => break num,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        }
    };

    let verbalized_num = number_lookup(user_num);
    println!("{verbalized_num}");
    Ok(())
}

fn number_lookup(number: usize) -> String {
    let mut transliteration = String::new();
    let mut suffix_temp = String::new();

    let num_string = number.to_string();
    let num_vector: Vec<char> = num_string.chars().collect(); // Handles tweens and teens
    let num_length = num_string.len();
    let mut remaining: usize;
    let mut skip_next: bool = false;

    let ones_digit = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens_digit = [
        "EMPTY", "EMPTY", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
        "ninety",
    ];
    let suffixes = [
        "EMPTY",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    if num_length == 1 {
        transliteration.push_str(ones_digit[number]);
        return transliteration;
    }
    for (i, c) in num_string.chars().enumerate() {
        remaining = num_length - i;

        if remaining % 3 == 1 && remaining > 3 {
            suffix_temp.push_str(suffixes[remaining / 3]);
            suffix_temp.push_str(" ");
        }
        // Numbers in teens vector take two spaces
        // 120,000 doesn't need to be read as one hundred twenty zero thousand
        if skip_next == true || c == '0' {
            skip_next = false;
            // Prevents one billion million thousand one
            if let Some(last_word) = transliteration.split_whitespace().last() {
                if suffixes.contains(&last_word) {
                    suffix_temp.clear();
                    continue;
                }
            }
            transliteration.push_str(&suffix_temp);
            suffix_temp.clear();
            continue;
        }
        let num = c.to_digit(10).unwrap(); // Converts number string to u32
        let num_usize: usize = num as usize; // Only usize can be used as index
        // Ones place
        if remaining % 3 == 1 {
            transliteration.push_str(ones_digit[num_usize]);
        // Tens place
        } else if remaining % 3 == 2 {
            // Special handling for tweens and teens
            if num == 1 {
                let temp = num_vector[i + 1].to_digit(10).unwrap();
                transliteration.push_str(teens[temp as usize]);
                skip_next = true;
            } else {
                transliteration.push_str(tens_digit[num_usize]);
            }
        // Fully transliterated
        } else if remaining == 0 {
            break;
        // Hundreds place
        } else if remaining % 3 == 0 {
            // 020 isn't zero twenty
            if num == 0 {
                continue;
            }
            transliteration.push_str(ones_digit[num_usize]);
            transliteration.push_str(" hundred");
        }

        transliteration.push_str(" ");
        transliteration.push_str(&suffix_temp);
        suffix_temp.clear();
    }
    return transliteration;
}
