use std::fs;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day02/input1.txt")?;
    let mut values: Vec<u32> = vec![];

    for line in contents.lines() {
        println!("{}", line);

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        let split1: Vec<String> = line.split(":").map(str::to_string).collect();
        let sets: Vec<String> = split1[1].split(";").map(str::to_string).collect();
        let mut is_legal = true;

        for set in sets {
            let colors: Vec<String> = set.split(",").map(str::to_string).collect();
            for color_pair in colors {
                let pair: Vec<String> = color_pair.split_whitespace().map(str::to_string).collect();
                let count: u32 = pair[0].clone().parse()?;
                let color = pair[1].clone();
                match color.as_str() {
                    "red" => {
                        if count > red_min {
                            red_min = count;
                        }
                    }
                    "green" => {
                        if count > green_min {
                            green_min = count;
                        }
                    }
                    "blue" => {
                        if count > blue_min {
                            blue_min = count;
                        }
                    }
                    _ => (),
                }
            }
        }

        let combined = red_min * green_min * blue_min;
        values.push(combined)
    }

    let sum: u32 = values.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}
