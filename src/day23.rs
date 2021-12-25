use crate::Part;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    format!(
        "{}",
        match part {
            Part::One => solve(input),
            Part::Two => solve(&unfold_input(input)),
        }
    )
}

fn solve(input: &str) -> usize {
    let (map, state) = parse_input(input);
    state.cheapest_path(&map)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Pos {
    StartRoom(usize, usize), // row, x
    Hallway(usize),          // x
    DestRoom(usize),         // row
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pod {
    colour: char,
    pos: Pos,
}

impl Pod {
    fn target_col(&self) -> usize {
        match self.colour {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            'D' => 9,
            _ => unreachable!(),
        }
    }

    fn in_dest_room(&self) -> bool {
        matches!(self.pos, Pos::DestRoom(_))
    }

    // returns (row, col)
    fn pos(&self) -> (usize, usize) {
        match self.pos {
            Pos::StartRoom(depth, x) => (depth, x),
            Pos::Hallway(x) => (1, x),
            Pos::DestRoom(depth) => (depth, self.target_col()),
        }
    }

    fn x(&self) -> usize {
        self.pos().1
    }

    fn y(&self) -> usize {
        self.pos().0
    }
}

impl fmt::Debug for Pod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {:?}", self.colour, self.pos)
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pods: Vec<Pod>,
}

impl State {
    // cost is based entirely on colour of the pod moving
    fn cost(&self, colour: char) -> usize {
        match colour {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }

    // list of next allowed states.  The cost is based on who just moved.
    fn neighbours(&self, map: &Map) -> Vec<(State, usize)> {
        // println!("Looking for next states from:\n{}", self.print(map));
        let mut to = vec![];

        for (i, pod) in self.pods.iter().enumerate() {
            let cost = self.cost(pod.colour);
            match pod.pos {
                Pos::StartRoom(y, x) => {
                    // free to move?
                    if self.free_to_move_into_hallway(i) {
                        // println!("{}/{:?} is free to move into hallway", i, pod);
                        if self.hallway_clear(x, pod.target_col())
                            && self.dest_room_y(pod.target_col()).is_some()
                        {
                            // pod can move directly into its destination room
                            let nx = pod.target_col();
                            let ny = self.dest_room_y(pod.target_col()).unwrap();
                            // println!(
                            //     "{}/{:?} can move directly into dest room {},{} ",
                            //     i, pod, ny, nx
                            // );
                            to.push((
                                self.clone_with_move(i, Pos::DestRoom(ny)),
                                cost * (y - 1 + (x.max(nx) - x.min(nx)) + ny - 1),
                            ));
                        } else {
                            // out and left and stop
                            let mut nx = x - 1;
                            while self.is_empty(map, 1, nx) {
                                if nx != 3 && nx != 5 && nx != 7 && nx != 9 {
                                    // println!("{}/{:?} can move left into hallway({})", i, pod, nx);
                                    to.push((
                                        self.clone_with_move(i, Pos::Hallway(nx)),
                                        cost * (y - 1 + (x - nx)),
                                    ));
                                }
                                nx -= 1;
                            }

                            // out and right and stop
                            let mut nx = x + 1;
                            while self.is_empty(map, 1, nx) {
                                if nx != 3 && nx != 5 && nx != 7 && nx != 9 {
                                    // println!("{}/{:?} can move right into hallway({})", i, pod, nx);
                                    to.push((
                                        self.clone_with_move(i, Pos::Hallway(nx)),
                                        cost * (y - 1 + (nx - x)),
                                    ));
                                }
                                nx += 1;
                            }
                        }
                    }
                }
                Pos::Hallway(x) => {
                    let nx = pod.target_col();
                    if ((nx > x && self.hallway_clear(x + 1, nx))
                        || (nx < x && self.hallway_clear(nx, x - 1)))
                        && self.dest_room_y(nx).is_some()
                    {
                        let ny = self.dest_room_y(nx).unwrap();
                        // println!(
                        //     "{}/{:?} can move from hallway into dest room {}, {}",
                        //     i, pod, ny, nx
                        // );
                        to.push((
                            self.clone_with_move(i, Pos::DestRoom(ny)),
                            cost * (x.max(nx) - x.min(nx) + ny - 1),
                        ));
                    }
                }
                Pos::DestRoom(_) => (),
            }
        }

        to
    }

    fn goal_reached(&self) -> bool {
        self.pods.iter().all(|p| p.in_dest_room())
    }

    fn cheapest_path(&self, map: &Map) -> usize {
        let mut frontier: BinaryHeap<(Reverse<usize>, State)> = BinaryHeap::new();
        let mut cost_so_far: HashMap<State, usize> = HashMap::new();

        frontier.push((Reverse(0), self.clone()));
        cost_so_far.insert(self.clone(), 0);

        while let Some((_cost, current)) = frontier.pop() {
            // println!(
            //     "queue length={}, current (cost={:?})",
            //     frontier.len() + 1,
            //     cost,
            // );
            if current.goal_reached() {
                return *cost_so_far.get(&current).unwrap();
            }

            for (next_state, next_cost) in current.neighbours(map).into_iter() {
                let new_cost = cost_so_far.get(&current).unwrap() + next_cost;
                if !cost_so_far.contains_key(&next_state)
                    || new_cost < *cost_so_far.get(&next_state).unwrap()
                {
                    cost_so_far.insert(next_state.clone(), new_cost);
                    frontier.push((Reverse(new_cost), next_state));
                }
            }
        }

        panic!("no path found!");
    }

    fn clone_with_move(&self, p: usize, new_pos: Pos) -> Self {
        let mut new = self.clone();
        new.pods[p].pos = new_pos;
        new
    }

    fn free_to_move_into_hallway(&self, i: usize) -> bool {
        let pod = self.pods[i];
        !self
            .pods
            .iter()
            .any(|p| p.x() == pod.x() && p.y() < pod.y())
    }

    fn is_empty(&self, map: &Map, y: usize, x: usize) -> bool {
        map.grid[y][x] == Cell::Space && !self.pods.iter().any(|p| p.y() == y && p.x() == x)
    }

    fn hallway_clear(&self, x0: usize, x1: usize) -> bool {
        let range = x0.min(x1)..=x0.max(x1);
        !self
            .pods
            .iter()
            .any(|p| p.y() == 1 && range.contains(&p.x()))
    }

    fn dest_room_y(&self, x: usize) -> Option<usize> {
        let mut target_y = self.pods.len() / 4 + 1;
        for pod in self.pods.iter() {
            if pod.x() == x {
                if pod.target_col() != x {
                    // can't enter dest room, different colour is present
                    return None;
                }
                if pod.y() <= target_y {
                    target_y = pod.y() - 1;
                }
            }
        }
        Some(target_y)
    }

    #[allow(dead_code)]
    fn print(&self, map: &Map) -> String {
        let mut s = String::new();
        for (rn, row) in map.grid.iter().enumerate() {
            for (cn, cell) in row.iter().enumerate() {
                s.push(match cell {
                    Cell::Wall => '#',
                    Cell::Space => {
                        if let Some(p) = self.pods.iter().position(|p| p.pos() == (rn, cn)) {
                            self.pods[p].colour
                        } else {
                            '.'
                        }
                    }
                });
            }
            if rn < self.pods.len() / 4 {
                s += &format!(
                    "\t{}/{:?}  {}/{:?}  {}/{:?}  {}/{:?}",
                    rn * 4,
                    self.pods[rn * 4],
                    rn * 4 + 1,
                    self.pods[rn * 4 + 1],
                    rn * 4 + 2,
                    self.pods[rn * 4 + 2],
                    rn * 4 + 3,
                    self.pods[rn * 4 + 3],
                );
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Eq, PartialEq)]
enum Cell {
    Wall,
    Space,
}

struct Map {
    grid: Vec<Vec<Cell>>,
}

fn parse_input(input: &str) -> (Map, State) {
    let mut grid = vec![];
    let mut pods = vec![];

    for (y, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for c in line.chars() {
            match c {
                '#' | ' ' => grid[y].push(Cell::Wall),
                '.' => grid[y].push(Cell::Space),
                'A'..='D' => {
                    pods.push(Pod {
                        colour: c,
                        pos: Pos::StartRoom(y, grid[y].len()),
                    });
                    grid[y].push(Cell::Space);
                }
                _ => panic!("unexpected: {}", c),
            }
        }
    }

    for i in 0..pods.len() {
        let pod = pods[i];
        if pod.x() == pod.target_col() {
            // if there's nothing below us of a different colour, no need to move
            if !pods
                .iter()
                .any(|p| p.y() > pod.y() && p.x() == pod.x() && p.colour != pod.colour)
            {
                pods[i].pos = Pos::DestRoom(pod.y());
            }
        }
    }

    (Map { grid }, State { pods })
}

fn unfold_input(input: &str) -> String {
    let mut s = String::new();
    for (i, line) in input.lines().enumerate() {
        s.push_str(line);
        s.push('\n');
        if i == 2 {
            s.push_str("  #D#C#B#A#\n");
            s.push_str("  #D#B#A#C#\n");
        }
    }
    s
}

#[test]
fn test() {
    let test_input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
    let (map, state) = parse_input(test_input);
    println!("{}", state.print(&map));
    assert_eq!(12521, state.cheapest_path(&map));

    let (map, state) = parse_input(&unfold_input(test_input));
    println!("{}", state.print(&map));
    assert_eq!(44169, state.cheapest_path(&map));
}
