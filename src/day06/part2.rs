use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day06/input1.txt")?;
    let mut values: Vec<u64> = vec![];
    let mut line_num = 0;

    let mut times: String = String::from("0");

    let mut distances: String = String::from("0");

    for line in contents.lines() {
        if line_num == 1 {
            let split2: Vec<String> = line.split(":").map(str::to_string).collect();
            distances = split2[1].replace(" ", "");
            line_num += 1;
        }
        if line_num == 0 {
            let split1: Vec<String> = line.split(":").map(str::to_string).collect();
            times = split1[1].replace(" ", "");
            line_num += 1;
        }
    }

    let mut count = 0;
    let max_time: u64 = times.parse()?;
    let min_distance: u64 = distances.parse()?;
    println!("Evaluating: {}, {}", max_time, min_distance);
    for t in 0..max_time + 1 {
        let possible_distance = t * (max_time - t);
        if possible_distance >= min_distance {
            count += 1;
        }
    }
    if count > 0 {
        values.push(count);
    }

    let mut output: u64 = 1;
    for val in values {
        output *= val;
    }
    println!("Answer: {}", output);

    Ok(())
}
