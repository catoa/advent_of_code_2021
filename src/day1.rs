pub fn solve(input: Vec<usize>) {
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
