use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::Add;
/// Problem - https://adventofcode.com/2018/day/2#part2
/// You have a bunch of boxes with IDs - you have to run a checksum over them
/// The checksum counts the exact same letter appearing twice and thrice, then
/// multiplies them together

/// Each box id will go in a group
enum BoxIDGroup {
    /// There are exactly two letters the same
    Two,
    /// There are exactly three letters the same
    Three,
    /// There are both
    Both,
    /// There are neither group (no 2 and no 3)
    Neither,
}

/// We can parse a box ID to choose which group it goes into
impl From<&str> for BoxIDGroup {
    fn from(boxid: &str) -> BoxIDGroup {
        // Count each individual character
        let mut char_count = BTreeMap::new();
        boxid
            .chars()
            .for_each(|ch| *char_count.entry(ch).or_insert(0) += 1);
        // Count the groups of 2s and 3s
        let has2s = char_count.values().any(|count| *count == 2);
        let has3s = char_count.values().any(|count| *count == 3);
        match (has2s, has3s) {
            (false, false) => BoxIDGroup::Neither,
            (true, false) => BoxIDGroup::Two,
            (false, true) => BoxIDGroup::Three,
            (true, true) => BoxIDGroup::Both,
        }
    }
}

/// Stores how many groups of 2 or 3 there are
#[derive(Debug, PartialEq)]
struct BoxIDScorer {
    twos: usize,
    threes: usize,
}

/// Allows us to add two BoxIDScorers together
impl Add for BoxIDScorer {
    type Output = BoxIDScorer;

    fn add(self, other: Self) -> Self {
        BoxIDScorer {
            twos: self.twos + other.twos,
            threes: self.threes + other.threes,
        }
    }
}

/// The default BoxIDScorer has no counts
impl BoxIDScorer {
    fn new() -> BoxIDScorer {
        BoxIDScorer { twos: 0, threes: 0 }
    }
}

/// We can convert a BoxIDGroup enum into a BoxIDScorer
impl From<BoxIDGroup> for BoxIDScorer {
    fn from(group: BoxIDGroup) -> BoxIDScorer {
        let mut twos = 0;
        let mut threes = 0;
        use BoxIDGroup::*;
        match group {
            Neither => (),
            Two => twos += 1,
            Three => threes += 1,
            Both => {
                twos += 1;
                threes += 1
            }
        }
        BoxIDScorer { twos, threes }
    }
}

/// BoxIDScorers can be summed
impl Sum for BoxIDScorer {
    fn sum<I>(iter: I) -> BoxIDScorer
    where
        I: Iterator<Item = BoxIDScorer>,
    {
        iter.fold(BoxIDScorer::new(), |sum, new| sum + new)
    }
}

/// Part1 - Find the checksum of all the box ids
pub fn part1() {
    let data = read_to_string("data/day2.txt").unwrap();
    let answer = do_part1(data);
    println!("Day2 (Part 1): Answer: {}", answer);
}

fn do_part1(data: String) -> usize {
    // counts for 2 letters and 3 letters words
    let counts: BoxIDScorer = data
        // Split the input into lines
        .lines()
        // Convert each line into a BoxIDGroup
        .map(|boxid| BoxIDGroup::from(boxid))
        // Convert each group into a score
        .map(|boxid_group| BoxIDScorer::from(boxid_group))
        // Sum the scores
        .sum();
    println!(
        "Day2 (Part 1): Groups of 2s: {} Groups of 3s: {}",
        counts.twos, counts.threes
    );
    // Calculate the checksum
    counts.twos * counts.threes
}

/// Part 2: find two boxes that differ by excactly one letter in the same place
pub fn part2() {
    let data = read_to_string("data/day2.txt").unwrap();
    let answer = do_part2(data);
    println!("Day2 (Part 2): Answer: {}", answer);
}

/// Returns the count of letters that are different (in the same position) between two strings
fn count_different_letters(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn do_part2(data: String) -> String {
    let pair = data
        .lines()
        // Compare every line with every other line
        .flat_map(|line1| data.lines().map(move |line2| (line1, line2)))
        // We only care about lines that differ by exactly one letter
        .filter(|(line1, line2)| count_different_letters(line1, line2) == 1)
        // Take the first one
        .nth(0)
        // Unwrap the option
        .unwrap();
    // We now have a pair of lines that differ by exactly one letter
    // We need to return the chars that are the same
    pair.0
        .chars()
        .zip(pair.1.chars())
        // We only care about chars that are the same
        .filter(|(a, b)| a == b)
        // We only want the single char (both are the same now anyway)
        .map(|(a, _b)| a)
        // Turn it into a String
        .collect()
}
