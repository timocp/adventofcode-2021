use crate::Part;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    match part {
        Part::One => format!("{}", part1(parse_input(input))),
        Part::Two => "".to_string(),
    }
}

fn part1(mut map: Map) -> usize {
    let mut step = 1;

    // println!("Initial state:\n{}", map);
    while map.step() {
        // println!("After {} steps:\n{}", step, map);
        step += 1;
    }
    // println!("After {} steps:\n{}", step, map);
    step
}

#[derive(Clone)]
enum Cucumber {
    EastFacing,
    SouthFacing,
    Empty,
}

impl From<char> for Cucumber {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::EastFacing,
            'v' => Self::SouthFacing,
            '.' => Self::Empty,
            _ => panic!("unexpected input: {}", c),
        }
    }
}

struct Map {
    cucumbers: Vec<Vec<Cucumber>>,
}

impl Map {
    // returns true if any cucumbers moved this step
    fn step(&mut self) -> bool {
        let mut changed = false;

        let height = self.cucumbers.len();
        let width = self.cucumbers[0].len();

        // east facing herd moves first
        let mut mvlist: Vec<(usize, usize, usize)> = vec![];
        for y in 0..height {
            for x in 0..width {
                if matches!(self.cucumbers[y][x], Cucumber::EastFacing) {
                    let east = if x + 1 < width { x + 1 } else { 0 };
                    if matches!(self.cucumbers[y][east], Cucumber::Empty) {
                        mvlist.push((y, x, east));
                        changed = true;
                    }
                }
            }
        }
        for (y, x, east) in mvlist {
            self.cucumbers[y][x] = Cucumber::Empty;
            self.cucumbers[y][east] = Cucumber::EastFacing;
        }

        // south facing herd moves next
        let mut mvlist: Vec<(usize, usize, usize)> = vec![];
        for y in 0..height {
            for x in 0..width {
                if matches!(self.cucumbers[y][x], Cucumber::SouthFacing) {
                    let south = if y + 1 < height { y + 1 } else { 0 };
                    if matches!(self.cucumbers[south][x], Cucumber::Empty) {
                        mvlist.push((y, x, south));
                        changed = true;
                    }
                }
            }
        }
        for (y, x, south) in mvlist {
            self.cucumbers[y][x] = Cucumber::Empty;
            self.cucumbers[south][x] = Cucumber::SouthFacing;
        }

        changed
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ef = 0;
        let mut sf = 0;
        for row in &self.cucumbers {
            for c in row {
                write!(
                    f,
                    "{}",
                    match c {
                        Cucumber::EastFacing => {
                            ef += 1;
                            '>'
                        }
                        Cucumber::SouthFacing => {
                            sf += 1;
                            'v'
                        }
                        Cucumber::Empty => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{} EF, {} SF", ef, sf)?;
        Ok(())
    }
}

fn parse_input(input: &str) -> Map {
    Map {
        cucumbers: input
            .lines()
            .map(|line| line.chars().map(Cucumber::from).collect())
            .collect(),
    }
}

#[test]
fn test() {
    let test_input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    assert_eq!(58, part1(parse_input(test_input)));
}
