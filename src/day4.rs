use std::ops::{Deref, DerefMut};
use std::str::FromStr;

const NUM_BOARD_ROWS: usize = 5;

fn read_data(input: &str) -> Option<(Vec<i64>, Vec<BingoBoard>)> {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let calls: Vec<i64> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();

    let boards: Vec<BingoBoard> = lines[1..]
        .iter()
        .filter_map(|&s| {
            if s.trim().len() == 0 {
                None
            } else {
                s.parse::<BingoRow>().ok()
            }
        })
        .collect::<Vec<BingoRow>>()
        .chunks(NUM_BOARD_ROWS)
        .map(|chunk| BingoBoard {
            grid: chunk.to_vec(),
            is_done: false,
        })
        .collect();

    Some((calls, boards))
}

impl FromStr for BingoRow {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<Space> = s
            .split_whitespace()
            .map(|num| Space {
                value: num.parse::<i64>().unwrap(),
                is_selected: false,
            })
            .collect();
        Ok(BingoRow { vals })
    }
}

#[derive(Debug, Clone)]
struct Space {
    value: i64,
    is_selected: bool,
}

impl Space {
    fn select(&mut self) {
        self.is_selected = true;
    }
}

#[derive(Debug, Clone)]
struct BingoRow {
    vals: Vec<Space>,
}

impl Deref for BingoRow {
    type Target = Vec<Space>;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl DerefMut for BingoRow {
    fn deref_mut(self: &mut BingoRow) -> &mut Self::Target {
        &mut self.vals
    }
}

#[derive(Debug, Clone)]
struct BingoBoard {
    grid: Vec<BingoRow>,
    is_done: bool,
}

impl BingoBoard {
    fn check_rows(&self) -> bool {
        for row in &self.grid {
            if row.iter().all(|space| space.is_selected) {
                return true;
            }
        }
        false
    }

    fn get_column(&self, i: usize) -> Vec<Space> {
        let mut column: Vec<Space> = Vec::new();
        for row in self.grid.iter() {
            column.push(row[i].clone());
        }
        column
    }

    fn check_cols(&self) -> bool {
        for i in 0..self.grid[0].len() {
            let column = BingoBoard::get_column(&self, i);
            if column.iter().all(|space| space.is_selected) {
                return true;
            }
        }
        false
    }

    fn mark_space(&mut self, val: i64) {
        for row in self.grid.iter_mut() {
            for space in row.iter_mut() {
                if space.value == val {
                    space.select();
                    break;
                }
            }
        }
    }

    fn has_win(&self) -> bool {
        self.check_rows() || self.check_cols()
    }

    fn unmarked_total(&self) -> i64 {
        let mut total = 0;
        for row in &self.grid {
            for space in row.iter() {
                if !space.is_selected {
                    total += space.value
                }
            }
        }
        total
    }
}

fn part_a(calls: Vec<i64>, mut boards: Vec<BingoBoard>) -> i64 {
    for call in calls {
        for board in boards.iter_mut() {
            board.mark_space(call);

            if board.has_win() {
                return board.unmarked_total() * call;
            }
        }
    }
    -1
}

fn part_b(calls: Vec<i64>, mut boards: Vec<BingoBoard>) -> i64 {
    let mut scores = Vec::new();

    for call in calls {
        for board in boards.iter_mut() {
            if !board.is_done {
                board.mark_space(call);

                if board.has_win() {
                    board.is_done = true;
                    scores.push(board.unmarked_total() * call);
                }
            }
        }
    }
    *scores.last().unwrap()
}

pub fn solve() {
    let input = include_str!("../data/day4.txt");
    let (calls, boards) = read_data(input).unwrap();
    println!("Part A: {}", part_a(calls.clone(), boards.clone()));
    println!("Part B: {}", part_b(calls, boards));
}

#[cfg(test)]
mod tests {
    use crate::day4::*;
    #[test]
    fn it_works() {
        let test_input = include_str!("../data/day4.txt");
        let (calls, boards) = read_data(test_input).unwrap();

        assert_eq!(10680, part_a(calls, boards));
    }

    #[test]
    fn it_works_part_b() {
        let test_input = include_str!("../data/day4.txt");
        let (calls, boards) = read_data(test_input).unwrap();
        assert_eq!(31892, part_b(calls, boards));
    }
}
