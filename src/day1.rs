pub fn solve(input: Vec<usize>) {
    find_increases(&input);
    find_increases(&input.windows(3).map(|s| s.iter().sum()).collect());
}

fn find_increases(input: &Vec<usize>) {
    let mut num_increases = 0;
    for window in input.windows(2) {
        if window[1] > window[0] {
            num_increases += 1;
        }
    }
    println!("{}", num_increases);
}
