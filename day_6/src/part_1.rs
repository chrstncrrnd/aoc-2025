use std::fs;

pub fn main() {
    let input = fs::read_to_string("input.txt").expect("No input file!");
    let mut input_lines = input.split("\n").collect::<Vec<&str>>();
    input_lines.pop();

    let ops = input_lines
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut buffer = ops.iter().map(|op| match *op{
       "+" => 0,
       "*" => 1,
       // big number so that we know there's an error
       _ => 1000000
    }).collect::<Vec<u64>>();

    for line in input_lines {
        for (i, c) in line.split_whitespace().enumerate() {
            let n: u64 = c.parse().unwrap();
            if ops[i] == "+" {
                buffer[i] += n;
            } else {
                buffer[i] *= n;
            }
        }
    }

    let mut total: u64 = 0;

    for element in buffer {
        total += element;
    }

    println!("Total is: {}", total);
}
