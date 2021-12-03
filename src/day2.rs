use std::include_str;

fn part_a(input: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.trim().split('\n') {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let units = parts.next().unwrap().parse::<i64>().unwrap();
        match direction {
            "forward" => horizontal_pos += units,
            "up" => depth -= units,
            "down" => depth += units,
            _ => eprintln!("Unknown input received"),
        }
    }
    println!("{}", horizontal_pos * depth)
}

pub fn solve() {
    let input_data = include_str!("../data/day2.txt");
}
