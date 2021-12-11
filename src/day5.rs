use crate::Part;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let lines = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => count_overlapping_points(&lines, CountFlag::WithoutDiagonals),
            Part::Two => count_overlapping_points(&lines, CountFlag::WithDiagonals),
        }
    )
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn step(&self, ordx: Ordering, ordy: Ordering) -> Pos {
        Pos {
            x: match ordx {
                Ordering::Less => self.x + 1,
                Ordering::Greater => self.x - 1,
                Ordering::Equal => self.x,
            },
            y: match ordy {
                Ordering::Less => self.y + 1,
                Ordering::Greater => self.y - 1,
                Ordering::Equal => self.y,
            },
        }
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Line {
    start: Pos,
    end: Pos,
}

#[derive(Clone, Copy, PartialEq)]
enum CountFlag {
    WithDiagonals,
    WithoutDiagonals,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.start, self.end)
    }
}

fn count_overlapping_points(lines: &[Line], count_flag: CountFlag) -> usize {
    let mut grid: HashMap<Pos, usize> = HashMap::new();

    for line in lines {
        if CountFlag::WithoutDiagonals == count_flag && !(line.horizontal() || line.vertical()) {
            continue;
        }
        let mut pos = line.start;
        let dx = line.start.x.cmp(&line.end.x);
        let dy = line.start.y.cmp(&line.end.y);
        loop {
            *grid.entry(pos).or_insert(0) += 1;
            if pos == line.end {
                break;
            }
            pos = pos.step(dx, dy);
        }
    }

    grid.values().filter(|&&n| n > 1).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| coords.split(','))
                .flatten()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|nums| Line {
            start: Pos::new(nums[0], nums[1]),
            end: Pos::new(nums[2], nums[3]),
        })
        .collect()
}

#[test]
fn test_day5() {
    let test_input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
    let lines = parse_input(test_input);
    assert_eq!(10, lines.len());
    assert_eq!(Pos::new(0, 9), lines[0].start);
    assert_eq!(Pos::new(5, 9), lines[0].end);
    assert_eq!(
        5,
        count_overlapping_points(&lines, CountFlag::WithoutDiagonals)
    );
    assert_eq!(
        12,
        count_overlapping_points(&lines, CountFlag::WithDiagonals)
    );
}
