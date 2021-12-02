pub fn run(input: &str) {
    let commands = parse_input(&input);
    let mut sub = Sub { hpos: 0, depth: 0 };
    sub.follow(&commands);
    println!("Day 1, part one: {:?}", sub.answer());
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    units: usize,
}

struct Sub {
    hpos: usize,
    depth: usize,
}

impl Sub {
    fn follow(&mut self, commands: &Vec<Command>) {
        for command in commands {
            match &command.dir {
                Direction::Forward => self.hpos += command.units,
                Direction::Down => self.depth += command.units,
                Direction::Up => self.depth -= command.units,
            }
        }
    }

    fn answer(&self) -> usize {
        self.hpos * self.depth
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        match words[1].parse() {
            Ok(units) => {
                if line.starts_with("forward") {
                    commands.push(Command {
                        dir: Direction::Forward,
                        units: units,
                    })
                } else if line.starts_with("down") {
                    commands.push(Command {
                        dir: Direction::Down,
                        units: units,
                    })
                } else if line.starts_with("up") {
                    commands.push(Command {
                        dir: Direction::Up,
                        units: units,
                    })
                } else {
                    eprintln!("parse error: invalid direction: {}", line)
                }
            }
            Err(e) => eprintln!("parse error: {}: {}", e, line),
        }
    }
    commands
}

#[test]
fn test_follow() {
    let mut sub = Sub { hpos: 0, depth: 0 };
    let commands = parse_input(
        "forward 5
down 5
forward 8
up 3
down 8
forward 2
",
    );
    sub.follow(&commands);
    assert_eq!(15, sub.hpos);
    assert_eq!(10, sub.depth);
    assert_eq!(150, sub.answer());
}
