use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Fissure {
    start: Point,
    end: Point
}

impl Fissure {
    fn parse_coordinate(coord: &str) -> Option<Point> {
        let (x, y) = coord.split_once(',')?;
        Some(Point {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        })
    }

    fn parse_line(line: &str) -> Option<Self> {
        let (coord1, coord2) = line.split_once("->")?;       
        Some(Fissure { start: Self::parse_coordinate(coord1)?, end: Self::parse_coordinate(coord2)? })
    }
}

fn read_data() -> Vec<Fissure> {
    let input = include_str!("../data/day5.txt");

    input
        .split('\n')
        .filter_map(Fissure::parse_line)
        .collect::<Vec<Fissure>>()
}

fn part_a(fissures: &Vec<Fissure>) -> i32 {
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    for fissure in fissures {
        if fissure.start.x == fissure.end.x {
            for y in min(fissure.start.y, fissure.end.y)..=max(fissure.start.y, fissure.end.y) {
                *point_map.entry(Point {x: fissure.start.x, y}).or_default() += 1;
            }
        } 
        if fissure.start.y == fissure.end.y {
            for x in min(fissure.start.x, fissure.end.x)..=max(fissure.start.x, fissure.end.x) {
                *point_map.entry(Point {x, y: fissure.start.y}).or_default() += 1;
            }
        }
    }
    point_map.values().filter(|fissure| **fissure >= 2).count() as i32
}

fn part_b(fissures: Vec<Fissure>) -> i32 {
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    for fissure in fissures {
        if fissure.start.x == fissure.end.x {
            for y in min(fissure.start.y, fissure.end.y)..=max(fissure.start.y, fissure.end.y) {
                *point_map.entry(Point {x: fissure.start.x, y}).or_default() += 1;
            }
        } else if fissure.start.y == fissure.end.y {
            for x in min(fissure.start.x, fissure.end.x)..=max(fissure.start.x, fissure.end.x) {
                *point_map.entry(Point {x, y: fissure.start.y}).or_default() += 1;
            }
        } else {
            let dx = if fissure.end.x - fissure.start.x > 0 { 1 } else { -1 };
            let dy = if fissure.end.y - fissure.start.y > 0 { 1 } else { -1 };
            let mut x = fissure.start.x;
            let mut y = fissure.start.y;

            *point_map.entry(Point {x, y}).or_default() += 1;
            while x != fissure.end.x {
                x += dx;
                y += dy;
                *point_map.entry(Point {x, y}).or_default() += 1;
            }
            assert!(y == fissure.end.y);
        }

       
    }

    point_map.values().filter(|fissure|**fissure >= 2).count() as i32  
}

pub fn solve() {
    let fissures = read_data();

    let part_a_result = part_a(&fissures);
    println!("Part A: {}", part_a_result);
    let part_b_result = part_b(fissures);
    println!("Part B: {}", part_b_result);
}
