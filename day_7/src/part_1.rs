use std::fs;

pub fn main(){
    let input = fs::read_to_string("input.txt").expect("No input file found!");
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().unwrap().len();
    let mut beams: Vec<usize> = vec![0; width];
    beams[lines.first().unwrap().find("S").unwrap()] = 1;
    let mut intersections = 0;


    for line in lines.iter(){
        for (i, c) in line.chars().enumerate(){
            if c == '^' && beams[i] == 1{
                if i > 0{
                    beams[i - 1] = 1;
                }
                if i < beams.len() - 1{
                    beams[i+1] = 1
                }

                beams[i] = 0;
                intersections += 1;

            }
        }

    }

    println!("Total intersections: {}", intersections);

}
