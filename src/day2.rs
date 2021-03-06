use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let commands = parse_input(input);
    let mut sub = Sub::new();
    match part {
        Part::One => sub.follow(&commands),
        Part::Two => sub.follow2(&commands),
    }
    format!("{}", sub.answer())
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
    aim: usize,
}

impl Sub {
    fn new() -> Sub {
        Sub {
            hpos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn follow(&mut self, commands: &[Command]) {
        for command in commands {
            match &command.dir {
                Direction::Forward => self.hpos += command.units,
                Direction::Down => self.depth += command.units,
                Direction::Up => self.depth -= command.units,
            }
        }
    }

    fn follow2(&mut self, commands: &[Command]) {
        for command in commands {
            match &command.dir {
                Direction::Forward => {
                    self.hpos += command.units;
                    self.depth += self.aim * command.units;
                }
                Direction::Down => self.aim += command.units,
                Direction::Up => self.aim -= command.units,
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
        let words: Vec<&str> = line.split(' ').collect();
        match words[1].parse() {
            Ok(units) => {
                if line.starts_with("forward") {
                    commands.push(Command {
                        dir: Direction::Forward,
                        units,
                    })
                } else if line.starts_with("down") {
                    commands.push(Command {
                        dir: Direction::Down,
                        units,
                    })
                } else if line.starts_with("up") {
                    commands.push(Command {
                        dir: Direction::Up,
                        units,
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
    let mut sub = Sub::new();
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

    sub = Sub::new();
    sub.follow2(&commands);
    assert_eq!(15, sub.hpos);
    assert_eq!(60, sub.depth);
    assert_eq!(900, sub.answer());
}
