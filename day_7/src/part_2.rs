use std::{collections::HashMap, fs};

pub fn main(){
    let input = fs::read_to_string("input.txt").expect("No input file found!");
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().unwrap().len();
    let mut beams: Vec<usize> = vec![0; width];
    beams[lines.first().unwrap().find("S").unwrap()] = 1;
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    println!("Total timelines: {}", explore_tree(&lines, lines.first().unwrap().find("S").unwrap(), 2, &mut cache))

}


// dfs
fn explore_tree(lines: &Vec<&str>, x_pos: usize, y_pos: usize, cache: &mut HashMap<(usize, usize), u64>)-> u64{
    if y_pos >= lines.len(){
        return 1;
    }
    if let Some(res) = cache.get(&(x_pos, y_pos)){
        return *res;
    }
    // explore le
    if lines[y_pos].chars().nth(x_pos).unwrap() == '.'{
        return explore_tree(lines, x_pos, y_pos + 2, cache);
    }
    // we are on a beam splitter
    let mut total = 0;


    if x_pos > 0{
        total += explore_tree(lines, x_pos - 1, y_pos + 2, cache);
    }
    if x_pos < lines.first().unwrap().len() - 1{
        total += explore_tree(lines, x_pos + 1, y_pos + 2, cache);
    }

    cache.insert((x_pos, y_pos), total);
    total
}
