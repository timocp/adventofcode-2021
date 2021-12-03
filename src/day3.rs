pub fn run(input: &str) {
    let input = parse_input(input);
    let (gamma, epsilon) = calc_power_consumption(&input);
    println!("Day 3, part one: {}", gamma * epsilon);
    println!(
        "Day 3, part two: {}",
        calc_oxygen_generator_rating(&input) * calc_co2_scrubber_rating(&input)
    );
}

fn calc_power_consumption(input: &[usize]) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let half = input.len() / 2;
    for n in 0..=15 {
        let count = input
            .iter()
            .filter(|&row| row & (1 << (15 - n)) > 0)
            .count();
        if count > 0 {
            if count > half {
                gamma += 1 << (15 - n);
            } else {
                epsilon += 1 << (15 - n);
            }
        }
    }
    (gamma, epsilon)
}

fn calc_oxygen_generator_rating(input: &[usize]) -> usize {
    let mut list = input.to_owned();
    for n in 0..=15 {
        let count = list.iter().filter(|&row| row & (1 << (15 - n)) > 0).count();
        if count > 0 {
            if count * 2 >= list.len() {
                list = list
                    .into_iter()
                    .filter(|&row| row & (1 << (15 - n)) > 0)
                    .collect::<Vec<usize>>();
            } else {
                list = list
                    .into_iter()
                    .filter(|&row| row & (1 << (15 - n)) == 0)
                    .collect::<Vec<usize>>();
            }
            if list.len() == 1 {
                return list[0];
            }
        }
    }
    0
}

fn calc_co2_scrubber_rating(input: &[usize]) -> usize {
    let mut list = input.to_owned();
    for n in 0..=15 {
        let count = list.iter().filter(|&row| row & (1 << (15 - n)) > 0).count();
        if count > 0 {
            if count * 2 >= list.len() {
                list = list
                    .into_iter()
                    .filter(|&row| row & (1 << (15 - n)) == 0)
                    .collect::<Vec<usize>>();
            } else {
                list = list
                    .into_iter()
                    .filter(|&row| row & (1 << (15 - n)) > 0)
                    .collect::<Vec<usize>>();
            }
            if list.len() == 1 {
                return list[0];
            }
        }
    }
    0
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
    assert_eq!((22, 9), calc_power_consumption(&input));
    assert_eq!(23, calc_oxygen_generator_rating(&input));
    assert_eq!(10, calc_co2_scrubber_rating(&input));
}
