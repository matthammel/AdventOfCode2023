use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let digits: Vec<String> = vec![
        "one".into(),
        "two".into(),
        "three".into(),
        "four".into(),
        "five".into(),
        "six".into(),
        "seven".into(),
        "eight".into(),
        "nine".into(),
    ];

    let contents = fs::read_to_string("./src/day01/input2.txt")?;
    let mut values: Vec<u32> = vec![];
    for line in contents.lines() {
        println!("{}", line);
        let line_vec: Vec<char> = line.chars().collect();
        let mut first: char = 'a';
        let mut second: char = 'b';
        for (i, a) in line_vec.iter().enumerate() {
            if a.is_numeric() {
                first = a.clone();
                break;
            }
            if let Some(val) =
                get_digit_from_text(line_vec[i..].iter().collect(), digits.clone(), true)
            {
                first = val;
                break;
            }
        }
        let end = line_vec.len();
        for (j, b) in line_vec.iter().rev().enumerate() {
            if b.is_numeric() {
                second = b.clone();
                break;
            }
            if let Some(val) =
                get_digit_from_text(line_vec[0..end - j].iter().collect(), digits.clone(), false)
            {
                second = val;
                break;
            }
        }
        if first.is_numeric() && second.is_numeric() {
            let combined = format!("{}{}", first, second);
            println!("{}", combined);
            values.push(combined.parse()?)
        }
    }

    let sum: u32 = values.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}

fn get_digit_from_text(
    text: String,
    possible_digits: Vec<String>,
    start_with: bool,
) -> Option<char> {
    if start_with {
        for digit in possible_digits {
            if text.starts_with(&digit) {
                match digit.as_str() {
                    "one" => return Some('1'),
                    "two" => return Some('2'),
                    "three" => return Some('3'),
                    "four" => return Some('4'),
                    "five" => return Some('5'),
                    "six" => return Some('6'),
                    "seven" => return Some('7'),
                    "eight" => return Some('8'),
                    "nine" => return Some('9'),
                    _ => return None,
                }
            }
        }
    } else {
        for digit in possible_digits {
            if text.ends_with(&digit) {
                match digit.as_str() {
                    "one" => return Some('1'),
                    "two" => return Some('2'),
                    "three" => return Some('3'),
                    "four" => return Some('4'),
                    "five" => return Some('5'),
                    "six" => return Some('6'),
                    "seven" => return Some('7'),
                    "eight" => return Some('8'),
                    "nine" => return Some('9'),
                    _ => return None,
                }
            }
        }
    }

    return None;
}
