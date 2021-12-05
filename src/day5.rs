use regex::Regex;
use std::fmt;

pub fn run(input: &str) {
    let grid = parse_input(input);
    println!("Day 5, part one: {}", grid.count_overlapping_points());
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
}

impl Line {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn covers(&self, pos: Pos) -> bool {
        if self.horizontal() {
            pos.y == self.start.y
                && if self.end.x >= self.start.x {
                    pos.x >= self.start.x && pos.x <= self.end.x
                } else {
                    pos.x >= self.end.x && pos.x <= self.start.x
                }
        } else if self.vertical() {
            pos.x == self.start.x
                && if self.end.y >= self.start.y {
                    pos.y >= self.start.y && pos.y <= self.end.y
                } else {
                    pos.y >= self.end.y && pos.y <= self.start.y
                }
        } else {
            false
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.start, self.end)
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
            }
            if line.start.y > max.y {
                max.y = line.start.y;
            }
        }
        Grid {
            lines: lines,
            min,
            max,
        }
    }

    fn count_overlapping_points(&self) -> usize {
        let mut total = 0;
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let mut count = 0;
                for line in self.lines.iter() {
                    if line.covers(Pos::new(x, y)) {
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
            .map(|caps| Line {
                start: Pos::new(
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                ),
                end: Pos::new(
                    caps.get(3).unwrap().as_str().parse().unwrap(),
                    caps.get(4).unwrap().as_str().parse().unwrap(),
                ),
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
    assert_eq!(5, grid.count_overlapping_points());
}
