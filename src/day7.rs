pub fn run(input: &str) {
    let crabs = parse_input(input);
    println!("Day 7, part one: {}", least_fuel(&crabs).unwrap());
}

fn least_fuel(crabs: &[usize]) -> Option<usize> {
    let last_pos = *crabs.iter().max().unwrap();
    (0..=last_pos).map(|pos| measure_fuel(crabs, pos)).min()
}

// measure fuel needed to move all crabs to a specific position
fn measure_fuel(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|&crab| if crab > pos { crab - pos } else { pos - crab })
        .sum()
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
fn test_day7() {
    let test_input = "16,1,2,0,4,2,7,1,2,14\n";

    let crabs = parse_input(test_input);
    assert_eq!(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], crabs);
    assert_eq!(41, measure_fuel(&crabs, 1));
    assert_eq!(37, measure_fuel(&crabs, 2));
    assert_eq!(39, measure_fuel(&crabs, 3));
    assert_eq!(71, measure_fuel(&crabs, 10));
    assert_eq!(37, least_fuel(&crabs).unwrap());
}
