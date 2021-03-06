use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    match part {
        Part::One => format!("{}", count_increases(&input)),
        Part::Two => format!("{}", count_sliding_increases(&input)),
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

fn count_increases(input: &[i32]) -> i32 {
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1
        }
    }
    count
}

fn count_sliding_increases(input: &[i32]) -> i32 {
    let mut count = 0;
    let mut sum = input[0] + input[1] + input[2]; // first window
    for i in 1..input.len() - 2 {
        let lastsum = sum;
        sum -= input[i - 1]; // take out first number of previous window
        sum += input[i + 2]; // add in last number of this window
        if sum > lastsum {
            count += 1
        }
    }
    count
}

#[test]
fn test_count_increases() {
    assert_eq!(
        7,
        count_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    )
}

#[test]
fn test_count_sliding_increases() {
    assert_eq!(
        5,
        count_sliding_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    )
}
