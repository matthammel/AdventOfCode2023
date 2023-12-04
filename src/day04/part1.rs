use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day04/input1.txt")?;

    let mut total_prize = 0;
    for line in contents.lines() {
        println!("{}", line);

        let split1: Vec<String> = line.split(":").map(str::to_string).collect();
        let sets: Vec<String> = split1[1].split("|").map(str::to_string).collect();

        let winning_numbers: Vec<String> = sets[0].split_whitespace().map(str::to_string).collect();
        let my_numbers: Vec<String> = sets[1].split_whitespace().map(str::to_string).collect();
        let mut prize = 0;
        for number in &my_numbers {
            for check in &winning_numbers {
                if check == number {
                    if prize == 0 {
                        prize = 1;
                    } else {
                        prize *= 2;
                    }
                }
            }
        }

        //let count: u32 = pair[0].clone().parse()?;

        total_prize += prize;
    }

    println!("Answer: {}", total_prize);

    Ok(())
}
