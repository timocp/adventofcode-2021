use std::collections::HashSet;

pub fn run(input: &str) {
    let grid = Grid::new(input);
    println!("Day 9, part one: {}", grid.part1());
    println!("Day 9, part one: {}", grid.part2());
}

struct Grid {
    data: Vec<Vec<u32>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>();
        let width = data[0].len();
        let height = data.len();
        Grid {
            data,
            width: width as i32,
            height: height as i32,
        }
    }

    fn get(&self, x: i32, y: i32) -> u32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            9
        } else {
            self.data[y as usize][x as usize]
        }
    }

    fn lowpoints(&self) -> Vec<(i32, i32)> {
        let mut points = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let this = self.get(x, y);
                if self.get(x - 1, y) > this
                    && self.get(x + 1, y) > this
                    && self.get(x, y - 1) > this
                    && self.get(x, y + 1) > this
                {
                    points.push((x, y));
                }
            }
        }
        points
    }

    fn part1(&self) -> u32 {
        self.lowpoints()
            .iter()
            .map(|p| self.get(p.0, p.1) + 1)
            .sum()
    }

    // recursively measure the size of a basin, including this point
    fn basin_size(&self, from: (i32, i32), seen: &mut HashSet<(i32, i32)>) -> usize {
        seen.insert(from);
        let mut size = 1;
        let this = self.get(from.0, from.1);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let to = (from.0 + dx, from.1 + dy);
            let that = self.get(to.0, to.1);
            if that >= this && that != 9 && !seen.contains(&to) {
                size += self.basin_size(to, seen);
            }
        }
        size
    }

    fn part2(&self) -> usize {
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        let mut basins: Vec<usize> = vec![];
        for start in self.lowpoints() {
            basins.push(self.basin_size(start, &mut seen));
        }
        basins.sort();
        basins.iter().rev().take(3).product()
    }
}

#[test]
fn test() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    let map = Grid::new(&test_input);
    assert_eq!(15, map.part1());
    assert_eq!(1134, map.part2());
}
