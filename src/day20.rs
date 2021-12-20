use crate::Part;
use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

pub fn run(input: &str, part: Part) -> String {
    let (alg, image) = parse_input(input);
    format!(
        "{}",
        enhance(
            &image,
            &alg,
            match part {
                Part::One => 2,
                Part::Two => 50,
            }
        )
        .grid
        .len()
    )
}

fn enhance(image: &Image, alg: &[bool], count: usize) -> Image {
    let mut img = image.enhance(alg);

    for _ in 1..count {
        img = img.enhance(alg)
    }

    img
}

struct Image {
    grid: HashSet<(i32, i32)>,
    background: bool,
    iterations: usize,
}

impl Image {
    fn enhance(&self, alg: &[bool]) -> Self {
        let mut new_grid = HashSet::new();

        let row_range = self.row_range();
        let col_range = self.col_range();

        for row in (row_range.start - 1)..(row_range.end + 1) {
            for col in (col_range.start - 1)..(col_range.end + 1) {
                let mut rule = 0;
                for dr in [-1, 0, 1] {
                    for dc in [-1, 0, 1] {
                        rule <<= 1;
                        if row_range.contains(&(row + dr)) && col_range.contains(&(col + dc)) {
                            if self.grid.contains(&(row + dr, col + dc)) {
                                rule += 1;
                            }
                        } else if self.background {
                            rule += 1
                        }
                    }
                }
                if alg[rule] {
                    new_grid.insert((row, col));
                }
            }
        }

        Self {
            grid: new_grid,
            background: alg[if self.background { 511 } else { 0 }],
            iterations: self.iterations + 1,
        }
    }

    fn row_range(&self) -> Range<i32> {
        Range {
            start: self.grid.iter().map(|p| p.0).min().unwrap(),
            end: self.grid.iter().map(|p| p.0).max().unwrap() + 1,
        }
    }

    fn col_range(&self) -> Range<i32> {
        Range {
            start: self.grid.iter().map(|p| p.1).min().unwrap(),
            end: self.grid.iter().map(|p| p.1).max().unwrap() + 1,
        }
    }
}

impl From<&str> for Image {
    fn from(s: &str) -> Self {
        let mut grid = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.insert((row as i32, col as i32));
                };
            }
        }
        Self {
            grid,
            background: false,
            iterations: 0,
        }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if self.grid.is_empty() {
            s += "(empty)";
        } else {
            for row in self.row_range() {
                for col in self.col_range() {
                    if self.grid.contains(&(row, col)) {
                        s += "#";
                    } else {
                        s += ".";
                    }
                }
                s += "\n";
            }
        }
        write!(
            f,
            "rows: {:?}, cols: {:?}, iterations; {}, lit pixels: {} background: {}\n{}",
            self.row_range(),
            self.col_range(),
            self.iterations,
            self.grid.len(),
            self.background,
            s
        )
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let alg = input[0].chars().map(|c| c == '#').collect();
    let image = Image::from(input[1]);

    (alg, image)
}

#[test]
fn test() {
    let test_input = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    let (alg, image) = parse_input(test_input);
    assert_eq!(512, alg.len());
    assert_eq!(10, image.grid.len());
    assert_eq!(24, enhance(&image, &alg, 1).grid.len());
    assert_eq!(35, enhance(&image, &alg, 2).grid.len());
    assert_eq!(3351, enhance(&image, &alg, 50).grid.len());
}
