use crate::Part;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let (map, state) = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => state.cheapest_path(&map),
            Part::Two => 0,
        }
    )
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PodState {
    StartRoomTop,
    StartRoomBottom,
    EnteredHallway, // is moving and in hallway
    StoppedHallway, // has found a place to stop in the hallway
    LeavingHallway, // is moving again (and must leave hallway now)
    DestRoomTop,    // has arrived in destination room
    DestRoomBottom, // in final place
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pod {
    colour: char,
    pos: (usize, usize),
    state: PodState,
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

    fn in_target_col(&self) -> bool {
        self.pos.1 == self.target_col()
    }
}

impl fmt::Debug for Pod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:30}",
            format!(
                "{} at {:2},{:2} is {:?}",
                self.colour, self.pos.0, self.pos.1, self.state
            )
        )
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

        // if anything is moving in the hallway for the first time, noone else is allowed to move
        if let Some(i) = self
            .pods
            .iter()
            .position(|p| p.state == PodState::EnteredHallway)
        {
            let pod = self.pods[i];
            let cost = self.cost(pod.colour);
            for dx in [-1, 1] {
                let x = if dx == -1 {
                    self.pods[i].pos.1 - 1
                } else {
                    self.pods[i].pos.1 + 1
                };
                if x < 1 || x > 11 {
                    // at end of hallway, no moves allowed
                } else if let None = self.at((pod.pos.0, x)) {
                    if x == 3 || x == 5 || x == 7 || x == 9 {
                        // may move there but may not stop moving
                        // println!("[{}] {:?} may move {} and must keep moving", i, pod, dx);
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0, x);
                        to.push((new, cost));
                    } else {
                        // may move there and stop or keep going
                        // println!("[{}] {:?} may move {} and keep moving or stop", i, pod, dx);
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0, x);
                        to.push((new, cost));
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0, x);
                        new.pods[i].state = PodState::StoppedHallway;
                        to.push((new, cost));
                    }
                }
            }
            // this pod must stop before other moves
            return to;
        }

        // if anything is moving out of the hallway, noone else is allowed to move
        if let Some(i) = self
            .pods
            .iter()
            .position(|p| p.state == PodState::LeavingHallway)
        {
            // this pod must enter destination room before other moves
            let pod = self.pods[i];
            let cost = self.cost(pod.colour);
            assert_eq!(pod.pos.1, pod.target_col());
            if let None = self.at((pod.pos.0 + 1, pod.pos.1)) {
                // println!("[{}] {:?} can enter destination room", i, pod);
                let mut new = self.clone();
                new.pods[i].pos = (pod.pos.0 + 1, pod.pos.1);
                new.pods[i].state = PodState::DestRoomTop;
                to.push((new, cost));
            }
            // this pod must stop before other moves
            return to;
        }

        // otherwise, basically anything is allowed
        for (i, pod) in self.pods.iter().enumerate() {
            let cost = self.cost(pod.colour);
            match pod.state {
                PodState::StartRoomTop => {
                    if pod.in_target_col() {
                        if let Some(p2) = self.at((pod.pos.0 + 1, pod.pos.1)) {
                            if self.pods[p2].colour != pod.colour {
                                // println!("[{}] {:?} in target room already but has to move into hallway to get out of the way", i, pod);
                                let mut new = self.clone();
                                new.pods[i].pos = (pod.pos.0 - 1, pod.pos.1);
                                new.pods[i].state = PodState::EnteredHallway;
                                to.push((new, cost));
                            }
                        } else {
                            // println!(
                            //     "[{}] {:?} in target room already and can move south",
                            //     i, pod
                            // );
                            let mut new = self.clone();
                            new.pods[i].pos = (pod.pos.0 + 1, pod.pos.1);
                            new.pods[i].state = PodState::DestRoomBottom;
                            to.push((new, cost));
                        }
                    } else {
                        // assumed safe since noone is allowed to stop outside a room
                        // println!("[{}] {:?} can move north", i, pod);
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0 - 1, pod.pos.1);
                        new.pods[i].state = PodState::EnteredHallway;
                        to.push((new, cost));
                    }
                }
                PodState::StartRoomBottom => {
                    if pod.in_target_col() {
                        // println!(
                        //     "[{}] {:?} in target room already and doesn't need to move",
                        //     i, pod
                        // );
                    } else {
                        if let None = self.at((pod.pos.0 - 1, pod.pos.1)) {
                            // println!("[{}] {:?} can move north", i, pod);
                            let mut new = self.clone();
                            new.pods[i].pos = (pod.pos.0 - 1, pod.pos.1);
                            new.pods[i].state = PodState::StartRoomTop;
                            to.push((new, cost));
                        } else {
                            // println!("[{}] {:?} is blocked from moving north", i, pod);
                        }
                    }
                }
                PodState::StoppedHallway => {
                    // if i start moving now, it's to leave so it has to be towards the correct
                    // column AND the path must be clear
                    let mut clear = true;
                    let steps;
                    if pod.pos.1 > pod.target_col() {
                        steps = pod.pos.1 - pod.target_col();
                        for x in pod.target_col()..pod.pos.1 {
                            if let Some(_) = self.at((pod.pos.0, x)) {
                                clear = false;
                                break;
                            }
                        }
                    } else {
                        steps = pod.target_col() - pod.pos.1;
                        for x in (pod.pos.1 + 1)..=pod.target_col() {
                            if let Some(_) = self.at((pod.pos.0, x)) {
                                clear = false;
                                break;
                            }
                        }
                    }
                    // check target column does not contain a different colour
                    for dy in [1, 2] {
                        if let Some(p2) = self.at((pod.pos.0 + dy, pod.target_col())) {
                            if self.pods[p2].colour != pod.colour {
                                clear = false;
                            }
                        }
                    }
                    if clear {
                        // println!("[{}] {:?} can move to its target column", i, pod);
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0, pod.target_col());
                        new.pods[i].state = PodState::LeavingHallway;
                        to.push((new, cost * steps));
                    }
                }
                PodState::DestRoomTop => {
                    // pod has finished moving and is in the target room.  only move allowed is
                    // further in if it is empty
                    if let None = self.at((pod.pos.0 + 1, pod.pos.1)) {
                        // println!(
                        //     "[{}] {:?} can move to the bottom of destination room",
                        //     i, pod
                        // );
                        let mut new = self.clone();
                        new.pods[i].pos = (pod.pos.0 + 1, pod.pos.1);
                        new.pods[i].state = PodState::DestRoomBottom;
                        to.push((new, cost));
                    }
                }
                PodState::DestRoomBottom => {} // nothing to do
                _ => panic!("unhandled {:?}", pod.state),
            }
        }

        to
    }

    // index of pod at particular location (None if empty)
    // SLOW, consider replacing positions with a HashMap of (y,x) => Pod
    fn at(&self, pos: (usize, usize)) -> Option<usize> {
        self.pods.iter().position(|pod| pod.pos == pos)
    }

    fn goal_reached(&self) -> bool {
        self.pods.iter().all(|p| match p.colour {
            'A' => p.pos == (2, 3) || p.pos == (3, 3),
            'B' => p.pos == (2, 5) || p.pos == (3, 5),
            'C' => p.pos == (2, 7) || p.pos == (3, 7),
            'D' => p.pos == (2, 9) || p.pos == (3, 9),
            _ => unreachable!(),
        })
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

    fn print(&self, map: &Map) -> String {
        let mut s = String::new();
        for (rn, row) in map.grid.iter().enumerate() {
            for (cn, cell) in row.iter().enumerate() {
                s.push(match cell {
                    Cell::Wall => '#',
                    Cell::Space => {
                        if let Some(p) = self.pods.iter().position(|p| p.pos == (rn, cn)) {
                            self.pods[p].colour
                        } else {
                            '.'
                        }
                    }
                });
            }
            if rn < 4 {
                s += &format!(
                    "\t[{}] {:?}  [{}] {:?}",
                    rn,
                    self.pods[rn],
                    rn + 4,
                    self.pods[rn + 4]
                );
            }
            s.push('\n');
        }
        s
    }
}

enum Cell {
    Wall,
    Space,
}

struct Map {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

fn parse_input(input: &str) -> (Map, State) {
    let mut map = Map {
        grid: vec![],
        width: 0,
        height: 0,
    };
    let mut pods = vec![];

    for line in input.lines() {
        map.grid.push(vec![]);
        for c in line.chars() {
            match c {
                '#' | ' ' => map.grid[map.height].push(Cell::Wall),
                '.' => map.grid[map.height].push(Cell::Space),
                'A'..='D' => {
                    pods.push(Pod {
                        colour: c,
                        state: if map.height == 2 {
                            PodState::StartRoomTop
                        } else {
                            PodState::StartRoomBottom
                        },
                        pos: (map.height, map.grid[map.height].len()),
                    });
                    map.grid[map.height].push(Cell::Space);
                }
                _ => panic!("unexpected: {}", c),
            }
        }
        map.height += 1;
    }

    (map, State { pods })
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
    // assert_eq!()
}
