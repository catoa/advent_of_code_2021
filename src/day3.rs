use std::collections::HashMap;
use std::include_str;

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
    ones: i32,
}

impl Tally {
    fn new() -> Self {
        Tally { zeros: 0, ones: 0 }
    }
}

fn part_a(input: &str) -> u32 {
    let mut bit_map: HashMap<usize, Tally> = HashMap::new();
    for line in input.split('\n') {
        for (idx, bit) in line.chars().enumerate() {
            let mut tally = bit_map.entry(idx).or_insert(Tally::new());
            match bit {
                '0' => {
                    tally.zeros += 1;
                }
                '1' => tally.ones += 1,
                _ => panic!("Invalid bit: should be 0 or 1, but found {}", bit),
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

    gamma_value * epsilon_value
}

fn part_b(input: &str) -> u32 {
    let bits: Vec<&str> = input.split('\n').collect();
    let width = bits[0].clone().len();

    // sanity check: ensure that first element in bits was not consumed
    assert_eq!(bits[0].len(), width);

    fn partition_by_bit_index(vec: Vec<&str>, i: usize) -> Option<(Vec<&str>, Vec<&str>)> {
        if vec.len() > 1 {
            let (left, right): (Vec<&str>, Vec<&str>) = vec
                .iter()
                .partition(|&line| line.chars().nth(i).unwrap() == '0');

            if left.len() > right.len() {
                Some((left, right))
            } else {
                Some((right, left))
            }
        } else {
            None
        }
    }

    let (mut most_common, mut least_common): (Vec<&str>, Vec<&str>) =
        partition_by_bit_index(bits, 0).unwrap();

    for i in 1..width {
        if most_common.len() > 1 {
            let (left, _) = partition_by_bit_index(most_common, i).unwrap();
            most_common = left;
        }
        if least_common.len() > 1 {
            let (_, right) = partition_by_bit_index(least_common, i).unwrap();
            least_common = right;
        }
    }

    let oxygen_rate = get_binary_value(most_common[0].to_string());
    let co2_rate = get_binary_value(least_common[0].to_string());

    oxygen_rate * co2_rate
}

pub fn solve() {
    let input = include_str!("../data/day3.txt").trim();

    println!("{}", part_a(&input));

    println!("{}", part_b(&input));
}

#[cfg(test)]
mod tests {
    use crate::day3::*;
    #[test]
    fn it_works() {
        let test_input = include_str!("../data/day3.txt");
        assert_eq!(1307354, part_a(test_input))
    }

    #[test]
    fn it_works_part_b() {
        let test_input = include_str!("../data/day3.txt");
        assert_eq!(482500, part_b(test_input))
    }
}
