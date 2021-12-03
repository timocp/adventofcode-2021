pub fn run(input: &str) {
    let input = parse_input(&input);
    let (gamma, epsilon) = calc_power(&input);
    println!("Day 3, part one: {}", gamma * epsilon);
}

fn calc_power(input: &Vec<usize>) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let half = input.len() / 2;
    for n in 0..=15 {
        let count = input.iter().filter(|&row| row & (1 << 15 - n) > 0).count();
        if count > 0 {
            if count > half {
                gamma += 1 << 15 - n;
            } else {
                epsilon += 1 << 15 - n;
            }
        }
    }
    (gamma, epsilon)
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect()
}

#[test]
fn test_calc() {
    let test_input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
    let input = parse_input(&test_input);
    assert_eq!((22, 9), calc_power(&input));
}
