use std::error::Error;
use std::str::FromStr;
use std::iter::Iterator;

/// Problem: https://adventofcode.com/2018/day/3
/// You have a fabric with rectangles cut out of it
/// Find how many square inches of fabric are cut my one or more rectangles

struct Rect {
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

#[test]
fn test_read_rect_from_str() {
    let input = "#123 @ 3,2: 5x4";
    let rect = input.parse::<Rect>().unwrap();
    assert_eq!(rect.x, 3);
    assert_eq!(rect.y, 2);
    assert_eq!(rect.width, 5);
    assert_eq!(rect.height, 4);
}
