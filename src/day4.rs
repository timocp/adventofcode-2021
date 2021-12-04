use std::fmt;

pub fn run(input: &str) {
    let (numbers, boards) = parse_input(input);
    println!("Day 4, part one: {}", play_to_win(numbers, boards).score());
    let (numbers, boards) = parse_input(input);
    println!("Day 4, part two: {}", play_to_lose(numbers, boards).score());
}

struct Board {
    grid: [[usize; 5]; 5],
    mark: [[bool; 5]; 5],
    done: bool,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..5 {
            s += &self.grid[y].map(|v| format!("{:2}", v)).join(" ");
            s += "  ";
            s += &self.mark[y].map(|v| if v { "#" } else { "." }).join("");
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

impl Board {
    fn new(input: &[Vec<usize>]) -> Board {
        let mut grid = [[0; 5]; 5];
        for y in 0..5 {
            for x in 0..5 {
                grid[y][x] = input[y][x];
            }
        }
        Board {
            grid,
            mark: [[false; 5]; 5],
            done: false,
        }
    }

    // returns true if this caused bingo
    fn mark(&mut self, number: usize) -> bool {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] == number {
                    self.mark[y][x] = true;
                    if self.mark[y].iter().all(|v| *v) || self.mark.iter().all(|row| row[x]) {
                        self.done = true;
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        false
    }

    fn sum_unmarked_numbers(&self) -> usize {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.mark[y][x] {
                    sum += self.grid[y][x]
                }
            }
        }
        sum
    }
}

#[derive(Debug)]
struct GameResult {
    sum_of_unmarked_numbers: usize,
    last_number_called: usize,
}

impl GameResult {
    fn score(&self) -> usize {
        self.sum_of_unmarked_numbers * self.last_number_called
    }
}

// plays bingo. takes ownership of params and wrecks them as a side effect
fn play_to_win(numbers: Vec<usize>, mut boards: Vec<Board>) -> GameResult {
    for number in numbers {
        for board in &mut boards {
            if board.mark(number) {
                return GameResult {
                    sum_of_unmarked_numbers: board.sum_unmarked_numbers(),
                    last_number_called: number,
                };
            }
        }
    }
    panic!("No winning board");
}

fn play_to_lose(numbers: Vec<usize>, mut boards: Vec<Board>) -> GameResult {
    let mut boards_in_play = boards.len();
    for number in numbers {
        for board in &mut boards {
            if !board.done && board.mark(number) {
                boards_in_play -= 1;
                if boards_in_play == 0 {
                    return GameResult {
                        sum_of_unmarked_numbers: board.sum_unmarked_numbers(),
                        last_number_called: number,
                    };
                }
            }
        }
    }
    panic!("Final board didn't win");
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut numbers = vec![];
    let mut boards = vec![];
    for (n, para) in input.split("\n\n").enumerate() {
        if n == 0 {
            numbers = para.split(',').map(|s| s.parse().unwrap()).collect();
        } else {
            boards.push(Board::new(
                &para
                    .split('\n')
                    .map(|row| {
                        row.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>(),
            ));
        }
    }
    (numbers, boards)
}

#[test]
fn test_bingo() {
    let test_input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    let (numbers, boards) = parse_input(test_input);
    assert_eq!(
        numbers,
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ],
    );
    assert_eq!(3, boards.len());
    assert_eq!(22, boards[0].grid[0][0]);
    assert_eq!(7, boards[2].grid[4][4]);

    let result = play_to_win(numbers, boards);
    assert_eq!(188, result.sum_of_unmarked_numbers);
    assert_eq!(24, result.last_number_called);
    assert_eq!(4512, result.score());

    let (numbers, boards) = parse_input(test_input);
    let result = play_to_lose(numbers, boards);
    assert_eq!(148, result.sum_of_unmarked_numbers);
    assert_eq!(13, result.last_number_called);
    assert_eq!(1924, result.score());
}
