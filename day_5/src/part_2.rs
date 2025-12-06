use std::fs;

type Intvl = (u64, u64);

pub fn main() {
    let input_str = fs::read_to_string("input.txt").expect("Input file not found!");
    let mut intervals: Vec<Intvl> = Vec::new();

    let mut total = 0;

    for line in input_str.split("\n") {
        if line.is_empty() {
            break;
        }
        let mut line_split = line.split("-");
        let low = line_split
            .next()
            .expect("Not enough values in interval (low)!")
            .parse()
            .expect("Not an integer (low)");


        let high = line_split
            .next()
            .expect("Not enough values in interval (high)!")
            .parse()
            .expect("Not an integer (high)");

        add_to_intervals(&mut intervals, (low, high));
    }

    for interval in intervals.iter(){
        total += interval.1 - interval.0 + 1;
    }

    println!("Total is: {}",total);
}


// adds elements to intervals array such that all elements are mutally exclusive
fn add_to_intervals(intervals: &mut Vec<Intvl>, to_add: Intvl) {
    // first of all check if the element to add shares anything with the intervals that already
    // exist
    let mut overlap_element: Option<Intvl> = None;
    let mut removal_index = 0;
    for (index, i) in intervals.iter().enumerate(){
        // we have an overlap
        if to_add.0 <= i.1 && to_add.1 >= i.0{
            // create a new element which is the union of both
            let new_low = to_add.0.min(i.0);
            let new_high = to_add.1.max(i.1);
            overlap_element = Some((new_low, new_high));
            removal_index = index;
            break;
        }
    }

    if let Some(element) = overlap_element{
        // remove the element which the new element overlaps with, and recursively call
        // add_to_intervals because this new element might overlap other elements
        intervals.remove(removal_index);
        add_to_intervals(intervals, element);
        return;
    }

    // intervals ordered low to high 
    // all elements that make it to here are mutually exclusive to the others
    if intervals.is_empty() {
        intervals.push(to_add);
        return;
    }
    let mut index_to_add = intervals.len();

    for (i, inter_iter) in intervals.iter().enumerate() {
        if to_add.0 < inter_iter.0 {
            index_to_add = i;
            break;
        }
    }

    if index_to_add == intervals.len(){
        intervals.push(to_add);
    }else{
        intervals.insert(index_to_add, to_add);
    }

}
