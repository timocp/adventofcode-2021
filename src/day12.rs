use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let system = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&system),
            Part::Two => part2(&system),
        }
    )
}

fn part1(system: &CaveSystem) -> usize {
    system.count_paths(&mut vec![0], Revisit::NotAllowed)
}

fn part2(system: &CaveSystem) -> usize {
    system.count_paths(&mut vec![0], Revisit::Allowed)
}

#[derive(Clone, Copy)]
enum Revisit {
    Allowed,
    NotAllowed,
}

#[derive(Debug, PartialEq)]
enum CaveType {
    Start,
    Big,
    Small,
    End,
}

#[derive(Debug)]
struct Cave {
    name: String,
    cave_type: CaveType,
}

#[derive(Debug)]
struct CaveSystem {
    caves: Vec<Cave>,       // vector of caves, index is cave number
    links: Vec<Vec<usize>>, // set of links between caves
}

// macro to push a cave id and then remove it to share vector in path search
macro_rules! explore_path {
    ($self:ident, $path:ident, $to_cave_id:expr, $revisit:expr) => {{
        $path.push($to_cave_id);
        let count = $self.count_paths(&mut $path, $revisit);
        $path.pop();
        count
    }};
}

impl CaveSystem {
    fn add_cave(&mut self, name: &str) -> usize {
        if let Some(i) = self.caves.iter().position(|cave| cave.name == name) {
            return i;
        }
        let cave_type = if name == "start" {
            CaveType::Start
        } else if name == "end" {
            CaveType::End
        } else if name.chars().next().unwrap().is_uppercase() {
            CaveType::Big
        } else {
            CaveType::Small
        };
        self.caves.push(Cave {
            name: name.to_owned(),
            cave_type,
        });
        self.links.push(vec![]);
        self.caves.len() - 1
    }

    fn add_link(&mut self, left: usize, right: usize) {
        self.links[left].push(right);
        self.links[right].push(left);
    }

    fn cave(&self, index: usize) -> &Cave {
        &self.caves[index]
    }

    fn count_paths(&self, mut path: &mut Vec<usize>, revisit: Revisit) -> usize {
        let this_cave_id = *path.last().unwrap();
        if self.cave(this_cave_id).cave_type == CaveType::End {
            // println!(
            //     "{}",
            //     path.iter()
            //         .map(|i| self.cave(*i).name.clone())
            //         .collect::<Vec<String>>()
            //         .join(" > ")
            // );
            return 1; // reached the end, this path counts
        }

        let v = self.links[this_cave_id]
            .iter()
            .map(|to_cave_id| {
                let to_cave = self.cave(*to_cave_id);
                match to_cave.cave_type {
                    CaveType::Start => 0, // may not revisit
                    CaveType::Big | CaveType::End => {
                        explore_path!(self, path, *to_cave_id, revisit)
                    }
                    CaveType::Small => {
                        if path.contains(to_cave_id) {
                            match revisit {
                                Revisit::Allowed => {
                                    // only allowed once per path
                                    explore_path!(self, path, *to_cave_id, Revisit::NotAllowed)
                                }
                                Revisit::NotAllowed => {
                                    0 // no small revisits are allowed
                                }
                            }
                        } else {
                            explore_path!(self, path, *to_cave_id, revisit)
                        }
                    }
                }
            })
            .sum();
        v
    }
}

fn parse_input(input: &str) -> CaveSystem {
    let mut system = CaveSystem {
        caves: vec![],
        links: vec![],
    };
    // we want start to always be cave number 0
    system.add_cave("start");
    for line in input.lines() {
        let names: Vec<&str> = line.split('-').collect();
        let left = system.add_cave(names[0]);
        let right = system.add_cave(names[1]);
        system.add_link(left, right);
    }
    system
}

#[test]
fn test() {
    let test_input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    let system = parse_input(test_input);
    assert_eq!(6, system.caves.len());
    assert_eq!("start", system.cave(0).name);
    assert_eq!(CaveType::Start, system.cave(0).cave_type);
    assert_eq!("A", system.cave(1).name);
    assert_eq!(CaveType::Big, system.cave(1).cave_type);
    assert_eq!("b", system.cave(2).name);
    assert_eq!(CaveType::Small, system.cave(2).cave_type);
    assert_eq!("end", system.cave(5).name);
    assert_eq!(CaveType::End, system.cave(5).cave_type);
    assert_eq!(1, system.count_paths(&mut vec![5], Revisit::NotAllowed));
    assert_eq!(10, system.count_paths(&mut vec![0], Revisit::NotAllowed));
    assert_eq!(10, part1(&system));
    assert_eq!(36, part2(&system));

    let test_input2 = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
    let system2 = parse_input(test_input2);
    assert_eq!(19, part1(&system2));
    assert_eq!(103, part2(&system2));

    let test_input3 = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
    let system3 = parse_input(test_input3);
    assert_eq!(226, part1(&system3));
    assert_eq!(3509, part2(&system3));
}
