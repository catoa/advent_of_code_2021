use std::include_str;

enum Direction {
    Up,
    Forward,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Unknown input received"),
        }
    }
}

struct Movement {
    direction: Direction,
    units: i64,
}

impl Movement {
    fn parse_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let direction = Direction::from_str(parts.next().unwrap());
        let units = parts.next().unwrap().parse::<i64>().unwrap();
        Self { direction, units }
    }
}

fn part_a(input: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.trim().split('\n') {
        let mvmt = Movement::parse_line(line);
        match mvmt.direction {
            Direction::Forward => horizontal_pos += mvmt.units,
            Direction::Up => depth -= mvmt.units,
            Direction::Down => depth += mvmt.units,
        }
    }
    println!("{}", horizontal_pos * depth)
}

fn part_b(input: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.trim().split('\n') {
        let mvmt = Movement::parse_line(line);
        match mvmt.direction {
            Direction::Forward => {
                horizontal_pos += mvmt.units;
                if aim > 0 {
                    depth += aim * mvmt.units;
                }
            }
            Direction::Up => {
                aim -= mvmt.units;
            }
            Direction::Down => {
                aim += mvmt.units;
            }
        }
    }
    println!("{}", horizontal_pos * depth)
}

pub fn solve() {
    let input_data = include_str!("../data/day2.txt");

    part_a(input_data);

    part_b(input_data);
}
