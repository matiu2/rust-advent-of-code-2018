use std::collections::BTreeSet;
use std::fs::read_to_string;

/// Takes a series of radio tuning adjustments, and prints the final frequency
pub fn part1() {
    let data = read_to_string("data/day1.txt").unwrap();
    let answer: isize = data
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum();
    println!("Day 1 (Part 1): {}", answer);
}

pub fn part2() {
    let data = read_to_string("data/day1.txt").unwrap();
    let answer = do_part2(data);
    println!("Day 1 (Part 2): {}", answer);
}

/// Takes a list of \n separated frequency adjustments and starting at 0 finds the current
/// frequency by accumulating the adjustments. Once it hits the same frequency twice, it returns
/// that frequency
fn do_part2(data: String) -> isize {
    let mut frequencies: BTreeSet<isize> = BTreeSet::new();
    frequencies.insert(0);
    // This will be set to true once we're done
    let answer = data
        // Split into lines
        .lines()
        // Convert to integer adjustments
        .map(|line| line.parse::<isize>().unwrap())
        // Cycle the adjustments list forever
        .cycle()
        // Accumulate the adjustments
        .scan(0, |freq, adjustment| {
            *freq += adjustment;
            Some(*freq)
        })
        // We only care about the one that doubles up
        .filter(|freq| !frequencies.insert(*freq))
        // We only need the one
        .nth(0);
    answer.unwrap()
}

#[test]
fn test_part2() {
    let data: String = "+7\n+7\n-2\n-7\n-4".into();
    let answer = do_part2(data);
    assert_eq!(answer, 14);
}
