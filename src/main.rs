use advent_of_code::day1;
use std::env;
use std::fs;

fn read_data() -> Vec<usize> {
    let contents = fs::read_to_string("data/day1/day1.txt").expect("Unable to read the file");
    contents
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

fn main() {
    let problem = env::args().nth(1).unwrap();

    match problem.as_ref() {
        "day1" => {
            let input_data = read_data();
            day1::solve(input_data);
        }
        _ => {
            println!("We have not solved that problem yet. :-(")
        }
    }
    ()
}
