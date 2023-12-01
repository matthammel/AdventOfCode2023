use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file

    let contents = fs::read_to_string("./src/day01/input1.txt")?;
    let mut values: Vec<u32> = vec![];
    for line in contents.lines() {
        println!("{}", line);
        let line_vec: Vec<char> = line.chars().collect();
        let mut first: char = 'a';
        let mut second: char = 'b';
        for a in line_vec.iter() {
            if a.is_numeric() {
                first = a.clone();
                break;
            }
        }

        for b in line_vec.iter().rev() {
            if b.is_numeric() {
                second = b.clone();
                break;
            }
        }
        if first.is_numeric() && second.is_numeric() {
            let combined = format!("{}{}", first, second);
            values.push(combined.parse()?)
        }
    }

    let sum: u32 = values.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}
