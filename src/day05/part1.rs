use std::fs;

struct MapRow {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day05/input1.txt")?;
    let mut need_seeds = true;
    let mut first_map = true;
    let mut seeds: Vec<String> = vec![];
    let mut maps: Vec<Vec<MapRow>> = vec![];
    let mut map_index = 0;
    for line in contents.lines() {
        //line 1:
        println!("{}", line);
        if need_seeds {
            let split1: Vec<String> = line.split(":").map(str::to_string).collect();
            println!("{}", split1[1]);
            seeds = split1[1].split_whitespace().map(str::to_string).collect();
            println!("{}", split1[1]);
            need_seeds = false;
        } else {
            if line.len() == 0 {
                if !first_map {
                    map_index += 1;
                }
                first_map = false;
                let new_map = vec![];
                maps.push(new_map);
            } else {
                let split_line: Vec<String> = line.split("-").map(str::to_string).collect();
                if split_line.len() > 1 {
                    //skip
                } else {
                    let vals: Vec<String> = line.split_whitespace().map(str::to_string).collect();
                    let row_data = MapRow {
                        destination_range_start: vals[0].parse()?,
                        source_range_start: vals[1].parse()?,
                        range_length: vals[2].parse()?,
                    };
                    maps[map_index].push(row_data);
                }
            }
        }
        //let count: u32 = pair[0].clone().parse()?;
    }
    let mut min_loc = u64::MAX;
    for seed in seeds {
        let mut source: u64 = seed.parse()?;
        let mut destination: u64 = source;
        for map in &maps {
            for row in map {
                if source >= row.source_range_start
                    && source <= row.source_range_start + row.range_length
                {
                    destination = row.destination_range_start + (source - row.source_range_start);
                }
            }
            source = destination;
        }
        if destination < min_loc {
            min_loc = destination;
        }
    }

    println!("Answer: {}", min_loc);

    Ok(())
}
