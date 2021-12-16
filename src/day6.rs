use crate::Part;
use std::collections::VecDeque;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        simulate_population(
            &input,
            match part {
                Part::One => 80,
                Part::Two => 256,
            }
        )
    )
}

fn simulate_population(fish: &[usize], days: usize) -> usize {
    // queue containing number of fish at each internal timer (0, 1, ... 8),
    let mut deq: VecDeque<usize> = VecDeque::from(vec![0; 9]);
    for &timer in fish.iter() {
        deq[timer] += 1;
    }

    for _ in 0..days {
        // anything at timer 0 resets to timer 6 and spawns a new fish at timer 8
        let breeding = deq.pop_front().unwrap();
        deq[6] += breeding;
        deq.push_back(breeding);
    }

    deq.iter().sum()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[test]
fn test() {
    let test_input = "\
3,4,3,1,2
";
    let input = parse_input(test_input);
    assert_eq!(vec![3, 4, 3, 1, 2], input);

    assert_eq!(5, simulate_population(&input, 0));
    assert_eq!(5, simulate_population(&input, 1));
    assert_eq!(6, simulate_population(&input, 2));
    assert_eq!(7, simulate_population(&input, 3));
    assert_eq!(9, simulate_population(&input, 4));
    assert_eq!(10, simulate_population(&input, 5));
    assert_eq!(10, simulate_population(&input, 6));
    assert_eq!(10, simulate_population(&input, 7));
    assert_eq!(10, simulate_population(&input, 8));
    assert_eq!(11, simulate_population(&input, 9));
    assert_eq!(12, simulate_population(&input, 10));
    assert_eq!(15, simulate_population(&input, 11));
    assert_eq!(17, simulate_population(&input, 12));
    assert_eq!(19, simulate_population(&input, 13));
    assert_eq!(20, simulate_population(&input, 14));
    assert_eq!(20, simulate_population(&input, 15));
    assert_eq!(21, simulate_population(&input, 16));
    assert_eq!(22, simulate_population(&input, 17));
    assert_eq!(26, simulate_population(&input, 18));
}
