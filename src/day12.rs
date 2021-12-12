use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let system = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => system.count_paths(&[0]),
            Part::Two => 0,
        }
    )
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

    fn count_paths(&self, path: &[usize]) -> usize {
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
                        self.count_paths(&path_append(path, *to_cave_id))
                    }
                    CaveType::Small => {
                        if path.contains(to_cave_id) {
                            // may not revisit
                            0
                        } else {
                            self.count_paths(&path_append(path, *to_cave_id))
                        }
                    }
                }
            })
            .sum();
        v
    }
}

fn path_append(path: &[usize], value: usize) -> Vec<usize> {
    let mut new = path.to_owned();
    new.push(value);
    new
}

fn parse_input(input: &str) -> CaveSystem {
    let mut system = CaveSystem {
        caves: vec![],
        links: vec![],
    };
    // we want start to alwats be cave number 0
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
    assert_eq!(1, system.count_paths(&[5]));
    assert_eq!(10, system.count_paths(&[0]));

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
    assert_eq!(19, system2.count_paths(&[0]));

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
    assert_eq!(226, system3.count_paths(&[0]));
}
