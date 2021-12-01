pub fn run(input: &str) {
    let input = parse_input(input);
    println!("Day 1, part one: {}", count_increases(input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

fn count_increases(input: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1
        }
    }
    count
}

#[test]
fn test_count_increases() {
    assert_eq!(
        7,
        count_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    )
}
