use std::num::ParseIntError;
use std::str::FromStr;

/// Problem: https://adventofcode.com/2018/day/4
/// Read the guard log and find the guard who's asleep the longest
/// eg. [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up

struct Minute {
    y: usize,
    m: usize,
    d: usize,
    h: usize,
    n: usize,
}

impl FromStr for Minute {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Minute, Self::Err> {
        // String format is [1518-11-01 00:00]
        let parts: Vec<&str> = s
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split_whitespace()
            .collect();
        assert_eq!(parts.len(), 2);
        let (date, time) = (parts[0], parts[1]);
        let date = date
            .split('-')
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        assert_eq!(date.len(), 3);
        let time = time
            .split(':')
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        assert_eq!(time.len(), 2);
        Ok(Minute {
            y: date[0],
            m: date[1],
            d: date[2],
            h: time[0],
            n: time[1],
        })
    }
}

#[test]
fn test_minute_from_str() {
    let input = "[1518-11-01 04:28]";
    let minute = input.parse::<Minute>().unwrap();
    assert_eq!(minute.y, 1518);
    assert_eq!(minute.m, 11);
    assert_eq!(minute.d, 01);
    assert_eq!(minute.h, 04);
    assert_eq!(minute.n, 28);
}

/// The different kinds of log entry possible
#[derive(PartialEq, Eq, Hash, Debug)]
enum EntryType {
    /// Guard n started his shift
    ShiftStart(usize),
    /// Current guard went to sleep
    Sleep,
    /// Current guard woke up
    Wake,
}

impl FromStr for EntryType {
    /// eg. [1518-11-01 00:00] Guard #10 begins shift
    /// [1518-11-01 00:05] falls asleep
    /// [1518-11-01 00:25] wakes up
    type Err = String;

    fn from_str(s: &str) -> Result<EntryType, Self::Err> {
        // We'll assume that the [date-part] is gone and we're left with everything after "] "
        use EntryType::*;
        if s.starts_with("Guard #") {
            s.splitn(2, '#')
                .nth(1)
                .and_then(|s| s.splitn(2, ' ').nth(0))
                .and_then(|num| num.parse::<usize>().ok())
                .map_or(Err(format!("Unable to parse guard ID: {} ", s)), |num| {
                    Ok(ShiftStart(num))
                })
        } else if s == "falls asleep" {
            Ok(Sleep)
        } else if s == "wakes up" {
            Ok(Wake)
        } else {
            Err(format!("Unrecognised log line: {}", s))
        }
    }
}

#[test]
fn test_entry_type_parse() {
    let shift10: EntryType = "Guard #10 begins shift".parse().unwrap();
    let sleep: EntryType = "falls asleep".parse().unwrap();
    let wake: EntryType = "wakes up".parse().unwrap();
    use EntryType::*;
    assert_eq!(shift10, ShiftStart(10));
    assert_eq!(sleep, Sleep);
    assert_eq!(wake, Wake);
}
