use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day04/input1.txt")?;

    let mut total_prize = 0;
    let mut counts = std::iter::repeat(1)
        .take(contents.lines().count())
        .collect::<Vec<_>>();
    for (index, line) in contents.lines().enumerate() {
        println!("{}", line);

        let split1: Vec<String> = line.split(":").map(str::to_string).collect();
        let sets: Vec<String> = split1[1].split("|").map(str::to_string).collect();

        let winning_numbers: Vec<String> = sets[0].split_whitespace().map(str::to_string).collect();
        let my_numbers: Vec<String> = sets[1].split_whitespace().map(str::to_string).collect();
        let mut matched = 0;
        for number in &my_numbers {
            for check in &winning_numbers {
                if check == number {
                    matched += 1
                }
            }
        }

        for add_to_idx in 1..matched + 1 {
            counts[index + add_to_idx] += counts[index]
        }
    }
    let mut sum = 0;
    for val in counts {
        sum += val;
    }
    println!("Answer: {}", sum);

    Ok(())
}
