use std::num::ParseIntError;
use std::str::FromStr;

/// Problem: https://adventofcode.com/2018/day/4
/// Read the guard log and find the guard who's asleep the longest
/// eg. [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
        let (minute, time) = (parts[0], parts[1]);
        let minute = minute
            .split('-')
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        assert_eq!(minute.len(), 3);
        let time = time
            .split(':')
            .map(|part| part.parse::<usize>())
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        assert_eq!(time.len(), 2);
        Ok(Minute {
            y: minute[0],
            m: minute[1],
            d: minute[2],
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
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum EventType {
    /// Guard n started his shift
    ShiftStart(usize),
    /// Current guard went to sleep
    Sleep,
    /// Current guard woke up
    Wake,
}

impl FromStr for EventType {
    /// eg. [1518-11-01 00:00] Guard #10 begins shift
    /// [1518-11-01 00:05] falls asleep
    /// [1518-11-01 00:25] wakes up
    type Err = String;

    fn from_str(s: &str) -> Result<EventType, Self::Err> {
        // We'll assume that the [date-part] is gone and we're left with everything after "] "
        use EventType::*;
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
    let shift10: EventType = "Guard #10 begins shift".parse().unwrap();
    let sleep: EventType = "falls asleep".parse().unwrap();
    let wake: EventType = "wakes up".parse().unwrap();
    use EventType::*;
    assert_eq!(shift10, ShiftStart(10));
    assert_eq!(sleep, Sleep);
    assert_eq!(wake, Wake);
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct LogEntry {
    minute: Minute,
    event: EventType,
}

impl FromStr for LogEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<LogEntry, String> {
        let parts: Vec<&str> = s.splitn(2, ']').collect();
        assert_eq!(parts.len(), 2);
        let (minute, event) = (parts[0], parts[1]);
        let minute: Minute = minute.parse().map_err(|e: ParseIntError| {
            format!("Unable to parse minute: error: {} - input: {}", e, s)
        })?;
        let event: EventType = event.trim().parse()?;
        Ok(LogEntry { minute, event })
    }
}

#[test]
fn test_parse_log_entry() {
    let shift10: LogEntry = "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap();
    let sleep: LogEntry = "[1518-11-01 00:05] falls asleep".parse().unwrap();
    let wake: LogEntry = "[1518-11-01 00:25] wakes up".parse().unwrap();
    assert_eq!(
        shift10,
        LogEntry {
            minute: Minute {
                y: 1518,
                m: 11,
                d: 1,
                h: 0,
                n: 0
            },
            event: EventType::ShiftStart(10),
        }
    );
    assert_eq!(
        sleep,
        LogEntry {
            minute: Minute {
                y: 1518,
                m: 11,
                d: 1,
                h: 0,
                n: 5
            },
            event: EventType::Sleep,
        }
    );
    assert_eq!(
        wake,
        LogEntry {
            minute: Minute {
                y: 1518,
                m: 11,
                d: 1,
                h: 0,
                n: 25,
            },
            event: EventType::Wake,
        }
    );
}

#[test]
/// Make sure we can sort log entries magically
fn test_log_entry_sort() {
    let shift10: LogEntry = "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap();
    let sleep: LogEntry = "[1518-11-01 00:05] falls asleep".parse().unwrap();
    let wake: LogEntry = "[1518-11-01 00:25] wakes up".parse().unwrap();
    // Store the log entries in the reverse order
    let mut entries: Vec<LogEntry> = vec![wake.clone(), sleep.clone(), shift10.clone()];
    entries.sort();
    // Make sure that they are now sorted
    assert_eq!(entries[0], shift10);
    assert_eq!(entries[1], sleep);
    assert_eq!(entries[2], wake);

}
