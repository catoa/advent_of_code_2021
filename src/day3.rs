use std::{include_str};
use std::collections::HashMap;


fn get_binary_value(bits: String) -> u32 {
    let mut value: u32 = 0;
    let num_bits = bits.len();
    for (idx, bit) in bits.chars().enumerate() {
        if bit == '1' {
            value += u32::pow(2, (num_bits - 1 - idx) as u32);
        }
    }
    value
}

#[derive(Debug)]
struct Tally {
    zeros: i32,
    ones: i32
}

impl Tally {
    fn new() -> Self {
        Tally {
            zeros: 0,
            ones: 0
        }
    }
}

fn part_a(input: &str) -> u32 {
    let mut bit_map : HashMap<usize, Tally> = HashMap::new();
    for line in input.trim().split('\n') {
        for (idx, bit) in line.chars().enumerate() {
            let mut tally = bit_map.entry(idx).or_insert(Tally::new());
            match bit {
                '0' => {
                    tally.zeros += 1;
                },
                '1' => tally.ones += 1,
                _ => panic!("Invalid bit: should be 0 or 1, but found {}", bit)
            }
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..bit_map.len() {
        let tally: &Tally = bit_map.get(&i).unwrap();
        let (num_zeros, num_ones) = (tally.zeros, tally.ones);
        if num_zeros > num_ones {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    let gamma_value = get_binary_value(gamma_rate);
    let epsilon_value = get_binary_value(epsilon_rate);
    println!("{}", gamma_value * epsilon_value);
    gamma_value * epsilon_value
}

fn part_b(_input: &str) {
    ()
}

pub fn solve() {
    let input = include_str!("../data/day3.txt");

    part_a(&input);

    part_b(&input);
}

#[cfg(test)]
mod tests {
    use crate::day3::*;
    #[test]
    fn it_works() {
        let test_input = include_str!("../data/day3_test.txt");
        
        assert_eq!(198, part_a(test_input))
    }
}