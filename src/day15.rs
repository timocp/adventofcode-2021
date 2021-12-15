use crate::Part;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let mut cave = Cave::new(input);
    if let Part::Two = part {
        cave = cave.embiggen();
    }
    format!("{}", cave.lowest_risk())
}

#[derive(Debug)]
struct Cave {
    risks: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Cave {
    fn new(input: &str) -> Cave {
        let risks: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line.chars().map(|c| c as usize - 48).collect())
            .collect();
        let height = risks.len();
        let width = risks[0].len();
        Cave {
            risks,
            height,
            width,
        }
    }

    // Return a new cave 5 times bigger in each axis
    fn embiggen(&self) -> Cave {
        let mut risks: Vec<Vec<usize>> = vec![vec![0; self.width * 5]; self.height * 5];
        let height = self.height * 5;
        let width = self.width * 5;

        for tile_row in 0..5 {
            for tile_col in 0..5 {
                for row in 0..self.width {
                    for col in 0..self.height {
                        risks[self.height * tile_row + row][self.width * tile_col + col] =
                            (self.risks[row][col] + tile_row + tile_col - 1) % 9 + 1;
                    }
                }
            }
        }

        Cave {
            risks,
            height,
            width,
        }
    }

    fn neighbours(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut to = vec![];
        if from.0 > 0 {
            to.push((from.0 - 1, from.1));
        }
        if from.0 < self.width - 1 {
            to.push((from.0 + 1, from.1));
        }
        if from.1 > 0 {
            to.push((from.0, from.1 - 1));
        }
        if from.1 < self.height - 1 {
            to.push((from.0, from.1 + 1));
        }
        to
    }

    fn cost(&self, to: (usize, usize)) -> usize {
        self.risks[to.0][to.1]
    }

    fn lowest_risk(&self) -> usize {
        let mut frontier: BinaryHeap<(Reverse<usize>, (usize, usize))> = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
        let mut cost_so_far: HashMap<(usize, usize), usize> = HashMap::new();
        let goal = (self.height - 1, self.width - 1);

        frontier.push((Reverse(0), (0, 0)));
        came_from.insert((0, 0), None);
        cost_so_far.insert((0, 0), 0);

        while let Some((_, current)) = frontier.pop() {
            if current == goal {
                return *cost_so_far.get(&current).unwrap();
            }

            for next in self.neighbours(current).into_iter() {
                let new_cost = cost_so_far.get(&current).unwrap() + self.cost(next);
                if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                    cost_so_far.insert(next, new_cost);
                    frontier.push((Reverse(new_cost), next));
                    came_from.insert(next, Some(current));
                }
            }
        }

        panic!("No path");
    }
}

#[test]
fn test() {
    let test_input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
    let cave = Cave::new(test_input);
    assert_eq!(vec![(1, 0), (0, 1)], cave.neighbours((0, 0)));
    assert_eq!(
        vec![(2, 3), (4, 3), (3, 2), (3, 4)],
        cave.neighbours((3, 3))
    );
    assert_eq!(vec![(8, 9), (9, 8)], cave.neighbours((9, 9)));
    assert_eq!(40, cave.lowest_risk());
    let cave = cave.embiggen();
    assert_eq!(315, cave.lowest_risk());
}
