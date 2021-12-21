use crate::Part;
use std::iter::Cycle;
use std::ops::RangeInclusive;

pub fn run(input: &str, part: Part) -> String {
    let mut game = Game::from(input);
    format!(
        "{}",
        match part {
            Part::One => game.part1(),
            Part::Two => 0,
        }
    )
}

#[derive(Debug)]
struct Game {
    pos: [usize; 2],
    score: [usize; 2],
    turn: usize,
    dice: Cycle<RangeInclusive<usize>>,
}

impl Game {
    fn turn(&mut self) {
        let player = self.turn % 2;
        for _ in 0..3 {
            self.pos[player] += self.dice.next().unwrap();
        }
        self.pos[player] = (self.pos[player] - 1) % 10 + 1;
        self.score[player] += self.pos[player];
        self.turn += 1;
    }

    fn play_until_end(&mut self) {
        while self.winner().is_none() {
            self.turn();
        }
    }

    fn rolls(&self) -> usize {
        self.turn * 3
    }

    fn winner(&self) -> Option<usize> {
        if self.score[0] >= 1000 {
            Some(0)
        } else if self.score[1] >= 1000 {
            Some(1)
        } else {
            None
        }
    }

    fn part1(&mut self) -> usize {
        self.play_until_end();
        self.score[self.winner().unwrap() + 1 % 2] * self.rolls()
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let start_pos = s
            .lines()
            .map(|line| line.chars().last().unwrap() as usize - 48)
            .collect::<Vec<_>>();
        Self {
            pos: [start_pos[0], start_pos[1]],
            score: [0, 0],
            turn: 0,
            dice: (1..=100).cycle(),
        }
    }
}

#[test]
fn test() {
    let test_input = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
    let mut game = Game::from(test_input);
    assert_eq!(4, game.pos[0]);
    assert_eq!(0, game.score[0]);
    assert_eq!(8, game.pos[1]);
    assert_eq!(0, game.score[1]);
    assert_eq!(0, game.turn);

    game.turn();
    assert_eq!(10, game.pos[0]);
    assert_eq!(10, game.score[0]);
    assert_eq!(8, game.pos[1]);
    assert_eq!(0, game.score[1]);
    assert_eq!(1, game.turn);

    game.turn();
    assert_eq!(10, game.pos[0]);
    assert_eq!(10, game.score[0]);
    assert_eq!(3, game.pos[1]);
    assert_eq!(3, game.score[1]);
    assert_eq!(2, game.turn);

    game.play_until_end();
    assert_eq!(10, game.pos[0]);
    assert_eq!(1000, game.score[0]);
    assert_eq!(3, game.pos[1]);
    assert_eq!(745, game.score[1]);
    assert_eq!(331, game.turn);

    let mut game = Game::from(test_input);
    assert_eq!(739785, game.part1());
}
