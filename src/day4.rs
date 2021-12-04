pub fn run(input: &str) {
    let (numbers, boards) = parse_input(input);
    println!("Day 4, part one: {}", play(numbers, boards).score());
}

#[derive(Debug)]
struct Board {
    grid: [[usize; 5]; 5],
    mark: [[bool; 5]; 5],
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
        }
    }

    fn mark(&mut self, number: usize) {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] == number {
                    self.mark[y][x] = true;
                    return;
                }
            }
        }
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

    fn bingo(&self) -> bool {
        for y in 0..5 {
            if self.mark[y].iter().all(|v| *v) {
                return true;
            }
        }
        for x in 0..5 {
            if self.mark.iter().all(|row| row[x]) {
                return true;
            }
        }
        false
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
fn play(numbers: Vec<usize>, mut boards: Vec<Board>) -> GameResult {
    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.bingo() {
                return GameResult {
                    sum_of_unmarked_numbers: board.sum_unmarked_numbers(),
                    last_number_called: number,
                };
            }
        }
    }
    panic!("No winning board");
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

    let result = play(numbers, boards);
    assert_eq!(188, result.sum_of_unmarked_numbers);
    assert_eq!(24, result.last_number_called);
    assert_eq!(4512, result.score());
}
