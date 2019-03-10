use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::Iterator;
use std::str::FromStr;

/// Problem: https://adventofcode.com/2018/day/3
/// You have a fabric with rectangles cut out of it
/// Find how many square inches of fabric are cut my one or more rectangles

/// Represents a 1x1 point in a cloth
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

/// The sheet of cloth that the elves are cutting holes out of
struct Sheet {
    /// Count how many times each hole has had a cut attempt
    holes: HashMap<Point, usize>,
}

impl Sheet {
    /// Cuts a hole in the sheet
    fn cut(&mut self, rect: &Rect) {
        // For each x,y point in rect, increase the number of times the point has been cut
        (rect.x..=rect.right())
            .into_iter()
            .flat_map(|x| (rect.y..=rect.bottom()).into_iter().map(move |y| (x, y)))
            .for_each(|(x, y)| *self.holes.entry(Point { x, y }).or_insert(0) += 1);
    }
    /// Return the number of attempted cuts for this square inch
    #[cfg(test)]
    fn cut_count(&self, x: usize, y: usize) -> usize {
        self.holes.get(&Point { x, y }).unwrap_or(&0).clone()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Rect {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Rect {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Rect, Self::Err> {
        // The format of the string is:
        // #123 @ 3,2: 5x4
        // #ID  @ LEFT,TOP: WIDTHxHEIGHT
        let parts: Vec<&str> = s.split_whitespace().collect();
        assert_eq!(parts.len(), 4);
        let (id, pos, size) = (parts[0], parts[2], parts[3]);
        // Parse the ID
        let id = id.trim_start_matches('#').parse::<usize>().unwrap();
        // Parse the pos
        let pos: Result<Vec<usize>, _> = pos
            // Get rid of the ':' on the end
            .trim_end_matches(':')
            .splitn(2, ',')
            .map(|part| part.parse::<usize>())
            .collect();
        let (x, y) = match pos {
            Ok(pos) => (pos[0], pos[1]),
            Err(err) => return Err(err.into()),
        };
        // Parse the size
        let size: Result<Vec<usize>, _> = size
            .splitn(2, 'x')
            .map(|part| part.parse::<usize>())
            .collect();
        let (width, height) = match size {
            Ok(size) => (size[0], size[1]),
            Err(err) => return Err(err.into()),
        };
        Ok(Rect {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

impl Rect {
    /// Returns the x value of our right most edge
    fn right(&self) -> usize {
        self.x + self.width - 1
    }
    /// Returns the y value of our bottom most edge
    fn bottom(&self) -> usize {
        self.y + self.height - 1
    }
    /// Returns true if these two rects intersect
    fn intersects(&self, other: &Rect) -> bool {
        self.x <= other.right()
            && self.right() >= other.x
            && self.y <= other.bottom()
            && self.bottom() >= other.y
    }
}

#[test]
fn test_read_rect_from_str() {
    let input = "#123 @ 3,2: 5x4";
    let rect = input.parse::<Rect>().unwrap();
    assert_eq!(rect.x, 3);
    assert_eq!(rect.y, 2);
    assert_eq!(rect.width, 5);
    assert_eq!(rect.height, 4);
}

#[test]
fn test_rect_right() {
    let r = Rect {
        id: 0,
        x: 3,
        y: 1,
        width: 2,
        height: 6,
    };
    assert_eq!(r.right(), 4);
}

#[test]
fn test_rect_bottom() {
    let r = Rect {
        id: 0,
        x: 3,
        y: 1,
        width: 2,
        height: 6,
    };
    assert_eq!(r.bottom(), 6);
}

#[test]
fn test_whole_example() {
    let input = concat!("#1 @ 1,3: 4x4\n", "#2 @ 3,1: 4x4\n", "#3 @ 5,5: 2x2\n");
    let rects: Vec<Rect> = input
        .lines()
        .map(|line| line.parse::<Rect>().unwrap())
        .collect();
    // First check it has parsed the rects correctly
    assert_eq!(rects[1].id, 2);
    assert_eq!(rects[1].x, 3);
    assert_eq!(rects[2].y, 5);
    // Now cut all the holes
    let mut sheet = Sheet {
        holes: HashMap::new(),
    };
    rects.iter().for_each(|hole| sheet.cut(&hole));
    assert_eq!(sheet.cut_count(0, 3), 0);
    assert_eq!(sheet.cut_count(1, 3), 1);
    assert_eq!(sheet.cut_count(3, 3), 2);
    let answer = sheet
        .holes
        .values()
        .filter(|cut_count| **cut_count > 1)
        .count();
    assert_eq!(answer, 4);
    // Now check the intersections
    let (r1, r2, r3) = (&rects[0], &rects[1], &rects[2]);
    assert!(r1.intersects(&r2));
    assert!(r2.intersects(&r1));
    assert!(!r1.intersects(&r3));
    assert!(!r2.intersects(&r3));
}

pub fn part1() {
    // Model the sheet of paper
    let mut sheet = Sheet {
        holes: HashMap::new(),
    };
    // Cut a bunch of holes in it
    read_to_string("data/day3.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Rect>().unwrap())
        .for_each(|hole| sheet.cut(&hole));
    let answer = sheet.holes.values().filter(|v| **v > 1).count();
    // The count of hole points, is the total area
    println!("Day3 (part 1): {}", answer);
}

pub fn part2() {
    // Find out which rectangle doesn't overlap any others
    let rects: Vec<Rect> = read_to_string("data/day3.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Rect>().unwrap())
        .collect();
    let answer = rects
        .iter()
        // All other rects should not overlap
        .filter(|r1| {
            rects
                .iter()
                .filter(|r2| r2 != r1)
                .all(|r2| !r1.intersects(r2))
        })
        .nth(0);
    match answer {
        Some(answer) => println!("Day3: part(2): {}", answer.id),
        None => println!("Day3: part(2): {}", "UNKNOWN"),
    };
}
