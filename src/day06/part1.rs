use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day06/input1.txt")?;
    let mut values: Vec<u32> = vec![];
    let mut line_num = 0;

    let mut times: Vec<String> = vec![];

    let mut distances: Vec<String> = vec![];

    for line in contents.lines() {
        if line_num == 1 {
            let split2: Vec<String> = line.split(":").map(str::to_string).collect();
            distances = split2[1].split_whitespace().map(str::to_string).collect();
            line_num += 1;
        }
        if line_num == 0 {
            let split1: Vec<String> = line.split(":").map(str::to_string).collect();
            times = split1[1].split_whitespace().map(str::to_string).collect();
            line_num += 1;
        }
    }
    for entry in 0..times.len() {
        let mut count = 0;
        let max_time: u32 = times[entry].parse()?;
        let min_distance: u32 = distances[entry].parse()?;
        println!("Evaluating: {}, {}", max_time, min_distance);
        for t in 0..max_time + 1 {
            let possible_distance = t * (max_time - t);
            println!("Checking: {}, {}", t, possible_distance);
            if possible_distance >= min_distance {
                println!("Ding: {} > {}", possible_distance, min_distance);
                count += 1;
            }
        }
        if count > 0 {
            values.push(count);
        }
    }

    let mut output = 1;
    for val in values {
        output *= val;
    }
    println!("Answer: {}", output);

    Ok(())
}
