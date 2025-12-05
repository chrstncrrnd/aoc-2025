use std::fs;

type Matrix = Vec<Vec<u8>>;

pub fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt").expect("Input file not found!");
    let mut matrix = matrixify(input);
    let mut total_removed = 0;
    let mut removed = 10000;
    while removed > 0 { 
        (removed, matrix) =  num_rolls(matrix);
        total_removed += removed;
    }
    println!("Total removed: {}", total_removed);
    
}

fn num_rolls(matrix: Matrix) -> (i32, Matrix){
    let mut out = 0;
    let mut out_matrix = matrix.clone();
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
                out_matrix[y][x] = 0;
            }
        }
    }
    (out, out_matrix)
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
