use std::fs;

pub fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt").expect("Input file not found!");
    let matrix = matrixify(input);
    let total_rolls = num_rolls(matrix);
    println!("Total number of rolls accessible: {}", total_rolls);
}

fn num_rolls(matrix: Vec<Vec<u8>>) -> i32 {
    let mut out = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix.first().unwrap().len() {
            if *matrix.get(y).unwrap().get(x).unwrap() == 0{
                continue;
            }
            let mut neighbouring = 0;
            // top
            if let Some(top_row) = matrix.get(y + 1) {
                if x != 0 {
                    neighbouring += top_row.get(x - 1).unwrap_or(&0);
                }
                neighbouring += top_row.get(x).unwrap_or(&0);
                neighbouring += top_row.get(x + 1).unwrap_or(&0);
            }
            // middle
            if let Some(middle_row) = matrix.get(y) {
                if x != 0 {
                    neighbouring += middle_row.get(x - 1).unwrap_or(&0);
                }
                neighbouring += middle_row.get(x + 1).unwrap_or(&0);
            }
            if y != 0 {
                // bottom
                if let Some(bottom_row) = matrix.get(y - 1) {
                    if x != 0 {
                        neighbouring += bottom_row.get(x - 1).unwrap_or(&0);
                    }
                    neighbouring += bottom_row.get(x).unwrap_or(&0);
                    neighbouring += bottom_row.get(x + 1).unwrap_or(&0);
                }
            }
            if neighbouring < 4 {
                out += 1;
            }
        }
    }
    out
}

fn matrixify(input: String) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for line in input.split("\n") {
        if line.is_empty(){continue;}
        let mut buffer: Vec<u8> = Vec::new();
        for c in line.chars() {
            if c == '.' {
                buffer.push(0);
            } else {
                buffer.push(1);
            }
        }
        out.push(buffer);
    }
    out
}
