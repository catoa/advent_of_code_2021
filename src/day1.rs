use std::fs;

fn read_data(path: &str) -> Vec<usize> {
    let contents = fs::read_to_string(path).expect("Unable to read the file");
    contents
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

pub fn solve() {
    let input = read_data("data/day1.txt");

    let has_increase = |acc: usize, s: &[usize]| {
        if &s[1] > &s[0] {
            acc + 1
        } else {
            acc + 0
        }
    };

    // Part A => 1477
    println!("{}", &input.windows(2).fold(0, has_increase));
    // Part B => 1523
    println!(
        "{:?}",
        &input
            .windows(3)
            .map(|s| s.iter().sum::<usize>())
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, has_increase)
    );
}
