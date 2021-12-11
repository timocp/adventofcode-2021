use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let crabs = parse_input(input);
    format!(
        "{}",
        least_fuel(
            &crabs,
            match part {
                Part::One => measure_fuel,
                Part::Two => measure_fuel2,
            }
        )
        .unwrap()
    )
}

fn least_fuel(crabs: &[usize], fuel: impl Fn(&[usize], usize) -> usize) -> Option<usize> {
    let last_pos = *crabs.iter().max().unwrap();
    (0..=last_pos).map(|pos| fuel(crabs, pos)).min()
}

// measure fuel needed to move all crabs to a specific position
fn measure_fuel(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|&crab| if crab > pos { crab - pos } else { pos - crab })
        .sum()
}

fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

fn measure_fuel2(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|&crab| {
            if crab > pos {
                triangle_number(crab - pos)
            } else {
                triangle_number(pos - crab)
            }
        })
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
    assert_eq!(37, least_fuel(&crabs, measure_fuel).unwrap());

    assert_eq!(15, triangle_number(5));
    assert_eq!(1830, triangle_number(60));
    assert_eq!(206, measure_fuel2(&crabs, 2));
    assert_eq!(168, measure_fuel2(&crabs, 5));
    assert_eq!(168, least_fuel(&crabs, measure_fuel2).unwrap());
}
