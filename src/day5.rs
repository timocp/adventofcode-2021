use regex::Regex;
use std::fmt;

pub fn run(input: &str) {
    let grid = parse_input(input);
    println!(
        "Day 5, part one: {}",
        grid.count_overlapping_points(CountFlag::WithoutDiagonals)
    );
    println!(
        "Day 5, part two: {}",
        grid.count_overlapping_points(CountFlag::WithDiagonals)
    );
}

#[derive(PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x: x, y: y }
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
    dir: Direction,
}

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
    DiagUpRight,
    DiagDownRight,
}

#[derive(Clone, Copy, PartialEq)]
enum CountFlag {
    WithDiagonals,
    WithoutDiagonals,
}

impl Line {
    // Normalise lines so that x only increases.
    // y only increases except when it's an up diagonal
    fn new(start: Pos, end: Pos) -> Line {
        if start.x == end.x {
            Line {
                start: Pos::new(start.x, start.y.min(end.y)),
                end: Pos::new(end.x, start.y.max(end.y)),
                dir: Direction::Vertical,
            }
        } else if start.y == end.y {
            Line {
                start: Pos::new(start.x.min(end.x), start.y),
                end: Pos::new(start.x.max(end.x), start.y),
                dir: Direction::Horizontal,
            }
        } else if start.x < end.x {
            if start.y < end.y {
                // Down and to the right
                Line {
                    start: start,
                    end: end,
                    dir: Direction::DiagDownRight,
                }
            } else {
                // Up and to the right
                Line {
                    start: start,
                    end: end,
                    dir: Direction::DiagUpRight,
                }
            }
        } else {
            if start.y < end.y {
                // Down and to the left - reverse
                Line {
                    start: end,
                    end: start,
                    dir: Direction::DiagUpRight,
                }
            } else {
                // Up and to the left - reverse
                Line {
                    start: end,
                    end: start,
                    dir: Direction::DiagDownRight,
                }
            }
        }
    }

    fn covers(&self, pos: Pos, count_flag: CountFlag) -> bool {
        pos.x >= self.start.x
            && pos.x <= self.end.x
            && match self.dir {
                Direction::Vertical => {
                    pos.x == self.start.x && pos.y >= self.start.y && pos.y <= self.end.y
                }
                Direction::Horizontal => pos.y == self.start.y,
                Direction::DiagUpRight => {
                    count_flag == CountFlag::WithDiagonals
                        && pos.y >= self.end.y
                        && pos.y <= self.start.y
                        && pos.x - self.start.x == self.start.y - pos.y
                }
                Direction::DiagDownRight => {
                    count_flag == CountFlag::WithDiagonals
                        && pos.y >= self.start.y
                        && pos.y <= self.end.y
                        && pos.x - self.start.x == pos.y - self.start.y
                }
            }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?} ({:?})", self.start, self.end, self.dir)
    }
}

struct Grid {
    lines: Vec<Line>,
    min: Pos,
    max: Pos,
}

impl Grid {
    fn new(lines: Vec<Line>) -> Grid {
        let min = Pos::new(0, 0);
        let mut max = Pos::new(0, 0);
        for line in lines.iter() {
            if line.start.x > max.x {
                max.x = line.start.x;
            } else if line.end.x > max.x {
                max.x = line.end.x;
            }
            if line.start.y > max.y {
                max.y = line.start.y;
            } else if line.end.y > max.y {
                max.y = line.end.y;
            }
        }
        Grid {
            lines: lines,
            min,
            max,
        }
    }

    fn count_overlapping_points(&self, count_flag: CountFlag) -> usize {
        let mut total = 0;
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let mut count = 0;
                for line in self.lines.iter() {
                    if line.covers(Pos::new(x, y), count_flag) {
                        count += 1;
                        if count == 2 {
                            total += 1;
                            break;
                        }
                    }
                }
            }
        }
        total
    }
}

fn parse_input(input: &str) -> Grid {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    Grid::new(
        input
            .lines()
            .map(|line| re.captures(line).unwrap())
            .map(|caps| {
                Line::new(
                    Pos::new(
                        caps.get(1).unwrap().as_str().parse().unwrap(),
                        caps.get(2).unwrap().as_str().parse().unwrap(),
                    ),
                    Pos::new(
                        caps.get(3).unwrap().as_str().parse().unwrap(),
                        caps.get(4).unwrap().as_str().parse().unwrap(),
                    ),
                )
            })
            .collect(),
    )
}

#[test]
fn test() {
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
    let grid = parse_input(test_input);
    assert_eq!(10, grid.lines.len());
    assert_eq!(Pos::new(0, 9), grid.lines[0].start);
    assert_eq!(Pos::new(5, 9), grid.lines[0].end);
    assert_eq!(
        5,
        grid.count_overlapping_points(CountFlag::WithoutDiagonals)
    );
    assert_eq!(12, grid.count_overlapping_points(CountFlag::WithDiagonals));
}
