use std::fs;


pub fn main(){
    let input_str = fs::read_to_string("input.txt").expect("Input file not found!");
    let mut intervals: Vec<(u64, u64)> = Vec::new();
    let mut past_half = false;

    let mut total = 0;


    for line in input_str.split("\n"){
        if line.is_empty(){
            past_half = true;
            continue;
        }
        if !past_half{
            let mut line_split = line.split("-");
            let low = line_split.next().expect("Not enough values in interval (low)!");
            let high = line_split.next().expect("Not enough values in interval (high)!");
            intervals.push((low.parse::<u64>().expect("Not an integer!"), high.parse::<u64>().expect("Not an integer (intervals)!")));
        }else{
            let num: u64 = line.parse().expect("Not an integer (num)!");
            for interval in intervals.iter(){
                if num >= interval.0 && num <= interval.1{
                    total += 1;
                    break;
                }
            }
        }
    }


    println!("Total number of fresh ingredients: {}", total);
}
