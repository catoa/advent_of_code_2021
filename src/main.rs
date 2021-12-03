use advent_of_code::{day1, day2};
use std::env;

fn main() {
    let problem = env::args().nth(1).unwrap();

    match problem.as_ref() {
        "day1" => {
            day1::solve();
        }
        "day2" => {
            day2::solve();
        }
        _ => {
            println!("We have not solved that problem yet. :-(")
        }
    }
    ()
}
