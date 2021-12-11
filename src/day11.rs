use crate::Part;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let mut grid = Grid::new(input);
    for _ in 0..100 {
        grid = grid.step();
    }
    format!(
        "{}",
        match part {
            Part::One => grid.total_flashes,
            Part::Two => {
                while grid.flashes != grid.rows * grid.cols {
                    grid = grid.step();
                }
                grid.total_steps
            }
        }
    )
}

#[derive(Clone)]
struct Grid {
    state: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    flashes: usize,
    total_flashes: usize,
    total_steps: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let state: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - 48).collect())
            .collect();
        let rows = state.len();
        let cols = state[0].len();
        Grid {
            state,
            rows,
            cols,
            flashes: 0,
            total_flashes: 0,
            total_steps: 0,
        }
    }

    fn step(&self) -> Grid {
        let mut next = self.clone();

        // queue of octopus positions that need to flash
        let mut queue: Vec<(usize, usize)> = vec![];

        // first, each octopus increases energy by 1
        for row in 0..next.rows {
            for col in 0..next.cols {
                next.state[row][col] += 1;
                if next.state[row][col] > 9 {
                    queue.push((row as usize, col as usize));
                }
            }
        }
        // process flashes until none left
        while !queue.is_empty() {
            // dbg!(&next, &queue);
            let flash = queue.pop().unwrap();
            // println!("FLASH: {:?}", flash);
            for dr in -1..=1isize {
                for dc in -1..=1isize {
                    if next.valid_pos(flash.0 as isize + dr, flash.1 as isize + dc) {
                        let row = (flash.0 as isize + dr) as usize;
                        let col = (flash.1 as isize + dc) as usize;
                        if next.state[row][col] < 10 {
                            next.state[row][col] += 1;
                            if next.state[row][col] > 9 {
                                queue.push((row, col));
                            }
                        }
                    }
                }
            }
        }

        // anything that flashed is now 0
        next.flashes = 0;
        for row in 0..next.rows {
            for col in 0..next.cols {
                if next.state[row][col] > 9 {
                    next.state[row][col] = 0;
                    next.flashes += 1
                }
            }
        }
        next.total_flashes += next.flashes;
        next.total_steps += 1;

        next
    }

    fn valid_pos(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.rows as isize && col >= 0 && col < self.cols as isize
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in &self.state {
            for oct in row {
                s += &format!("{}", oct);
                //s += &format!("{:2x}", oct);
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

#[test]
fn test() {
    let test_input = "\
11111
19991
19191
19991
11111
";
    let grid = Grid::new(test_input);
    let grid = grid.step();
    assert_eq!(
        "\
34543
40004
50005
40004
34543
",
        format!("{:?}", grid)
    );
    let grid = grid.step();
    assert_eq!(
        "\
45654
51115
61116
51115
45654
",
        format!("{:?}", grid)
    );

    let test_input2 = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
    let mut grid = Grid::new(test_input2);
    for _ in 0..100 {
        grid = grid.step();
    }
    assert_eq!(1656, grid.total_flashes);
}
