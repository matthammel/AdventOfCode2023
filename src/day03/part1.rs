use std::fs;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read file
    let contents = fs::read_to_string("./src/day03/input1.txt")?;
    let mut values: Vec<u32> = vec![];
    let mut image: Vec<Vec<char>> = vec![];

    for line in contents.lines() {
        let line_vec: Vec<char> = line.chars().collect();

        image.push(line_vec);
    }
    let mut number: Vec<char> = vec![];
    let mut start_col = 0;
    let mut end_col = 0;
    let mut is_first: bool = true;
    for (row, row_) in image.iter().enumerate() {
        for (col, col_) in row_.iter().enumerate() {
            if col_.is_numeric() {
                if is_first {
                    start_col = col;
                    is_first = false;
                }
                end_col = col;
                number.push(col_.clone());
            } else {
                if !is_first {
                    //check for symbol
                    println!("Checking adjacent");
                    if is_adjacent(&image, row, start_col, end_col, image.len(), row_.len()) {
                        let s_val: String = number.into_iter().collect();
                        values.push(s_val.parse()?);
                    }

                    //reset the tickers
                    number = vec![];
                    start_col = 0;
                    end_col = 0;
                    is_first = true;
                }
            }
        }
    }

    let sum: u32 = values.iter().sum();
    println!("Answer: {}", sum);

    Ok(())
}

fn is_adjacent(
    image: &Vec<Vec<char>>,
    row: usize,
    start_col: usize,
    end_col: usize,
    max_row: usize,
    max_col: usize,
) -> bool {
    let retval = false;
    let mut begin = start_col;
    let mut end = end_col;

    //previous row
    if start_col > 0 {
        begin -= 1;
    }
    if end_col + 1 < max_col {
        end = end_col + 1;
    }
    if is_symbol(image[row][begin]) {
        return true;
    }
    if is_symbol(image[row][end]) {
        return true;
    }
    if row > 0 {
        for c in begin..end + 1 {
            if is_symbol(image[row - 1][c]) {
                return true;
            }
        }
    }
    if row + 1 < max_row {
        for c in begin..end + 1 {
            if is_symbol(image[row + 1][c]) {
                return true;
            }
        }
    }

    retval
}

fn is_symbol(character: char) -> bool {
    if character == '.' || character.is_alphanumeric() {
        return false;
    }
    println!("found symbol");
    return true;
}
