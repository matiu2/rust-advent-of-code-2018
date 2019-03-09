use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::iter::Iterator;
use std::ops::Add;
use std::str::FromStr;

/// Problem: https://adventofcode.com/2018/day/3
/// You have a fabric with rectangles cut out of it
/// Find how many square inches of fabric are cut my one or more rectangles

#[derive(PartialEq, Eq, Debug)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

/// Represents a 1x1 point in a cloth
#[derive(Hash, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Holes {
    /// All the points that have holes
    points: HashSet<Point>,
}

impl Add<Holes> for Holes {
    type Output = Holes;

    fn add(self, other: Holes) -> Holes {
        // Joins both sets together
        self.points.union(&other.points);
        self
    }
}

impl FromStr for Rect {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Rect, Self::Err> {
        // The format of the string is:
        // #123 @ 3,2: 5x4
        // #ID  @ LEFT,TOP: WIDTHxHEIGHT
        let parts: Vec<&str> = s.split_whitespace().collect();
        assert_eq!(parts.len(), 4);
        let (pos, size) = (parts[2], parts[3]);
        println!("pos: {} - size: {}", pos, size);
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
            x,
            y,
            width,
            height,
        })
    }
}

impl Rect {
    /// Returns the holes that this rectangle creates
    fn holes(&self) -> Holes {
        Holes {
            points: (self.x..=(self.x + self.width))
                .into_iter()
                .flat_map(move |x| {
                    (self.y..=(self.y + self.height))
                        .into_iter()
                        .map(move |y| (x, y))
                })
                .map(|(x, y)| Point { x, y })
                .collect(),
        }
    }
    /// Returns the x value of our right most edge
    fn right(&self) -> usize {
        self.x + self.width
    }
    /// Returns the y value of our bottom most edge
    fn bottom(&self) -> usize {
        self.y + self.height
    }
    /// Returns a tuple of (leftest rect, rightest rect) based on x
    fn left_right<'a>(&'a self, other: &'a Rect) -> (&Rect, &Rect) {
        if self.x <= other.x {
            (self, other)
        } else {
            (other, self)
        }
    }
    /// Returns the (highest, lowest) based on 'y'
    fn high_low<'a>(&'a self, other: &'a Rect) -> (&Rect, &Rect) {
        if self.y <= other.y {
            (self, other)
        } else {
            (other, self)
        }
    }
    // Returns a new rectangle, which is the intersection of two rectangles
    // If they never intersect, it returns None
    fn intersect(&self, other: &Rect) -> Option<Rect> {
        let (left, right) = self.left_right(other);
        let (high, low) = self.high_low(other);
        // If the two rectangles never meet horizontally,
        // or vertically there's no intersection
        if left.right() < right.x || high.bottom() < low.y {
            return None;
        }
        Some(Rect {
            x: right.x,
            y: low.y,
            width: min(self.right(), other.right()) - right.x,
            height: min(self.bottom(), other.bottom()) - low.y,
        })
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
    let r = Rect{x: 3, y: 1, width: 2, height: 6};
    assert_eq!(r.right(), 5);
}

#[test]
fn test_rect_bottom() {
    let r = Rect{x: 3, y: 1, width: 2, height: 6};
    assert_eq!(r.bottom(), 7);
}

#[test]
fn test_rect_holes() {
    let r = Rect{x: 3, y: 1, width: 2, height: 6};
    let holes = r.holes();
    // Top left
    assert!(holes.points.contains(&Point{x: 3, y: 1}));
    // Bottom right
    assert!(holes.points.contains(&Point{x: 5, y: 7}));
    // Too low
    assert!(!holes.points.contains(&Point{x: 5, y: 8}));
    // Too right
    assert!(!holes.points.contains(&Point{x: 6, y: 3}));
    // Too high
    assert!(!holes.points.contains(&Point{x: 3, y: 0}));
    // Too left
    assert!(!holes.points.contains(&Point{x: 2, y: 3}));
}

#[test]
fn test_rect_left_right() {
    let r1 = Rect{x: 3, y: 1, width: 2, height: 6};
    let r2 = Rect{x: 4, y: 2, width: 2, height: 6};
    let (left, right) = r1.left_right(&r2);
    assert_eq!(left, &r1);
    assert_eq!(right, &r2);
    let (left, right) = r2.left_right(&r1);
    assert_eq!(left, &r1);
    assert_eq!(right, &r2);
}

#[test]
fn test_rect_high_low() {
    let r1 = Rect{x: 3, y: 1, width: 2, height: 6};
    let r2 = Rect{x: 4, y: 2, width: 2, height: 6};
    let (high, low) = r1.high_low(&r2);
    assert_eq!(high, &r1);
    assert_eq!(low, &r2);
    let (high, low) = r2.high_low(&r1);
    assert_eq!(high, &r1);
    assert_eq!(low, &r2);
}

#[test]
fn test_rect_intersection_1() {
    let r1 = Rect{x: 3, y: 1, width: 2, height: 6};
    let r2 = Rect{x: 4, y: 2, width: 2, height: 6};
    let r3 = r1.intersect(&r2).unwrap();
    assert_eq!(r3.x, 4);
    assert_eq!(r3.y, 2);
    assert_eq!(r3.width, 1);
    assert_eq!(r3.height, 5);
    let r4 = r2.intersect(&r1).unwrap();
    assert_eq!(&r3, &r4)
}
