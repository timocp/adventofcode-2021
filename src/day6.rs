pub fn run(input: &str) {
    let input = parse_input(input);
    println!("Day 6, part one: {}", simulate_population(&input, 80));
}

// For a fish with a certain timer, how many descendent fish are there after
// a number of days?
// TODO: may be necessary to memoize
fn simulate_fish(timer: usize, days: usize) -> usize {
    let mut timer = timer;
    let mut days = days;
    let mut count = 1; // myself
    while days > 0 {
        days -= 1;
        if timer == 0 {
            count += simulate_fish(8, days); // descendants
            timer = 6;
        } else {
            timer -= 1;
        }
    }
    count
}

fn simulate_population(fish: &Vec<usize>, days: usize) -> usize {
    fish.iter().map(|&timer| simulate_fish(timer, days)).sum()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

#[test]
fn test_day6() {
    let test_input = "\
3,4,3,1,2
";
    let input = parse_input(test_input);
    assert_eq!(vec![3, 4, 3, 1, 2], input);

    assert_eq!(1, simulate_fish(3, 0));
    assert_eq!(1, simulate_fish(3, 1));
    assert_eq!(1, simulate_fish(3, 2));
    assert_eq!(1, simulate_fish(3, 3));
    assert_eq!(2, simulate_fish(3, 4));
    assert_eq!(2, simulate_fish(3, 5));
    assert_eq!(2, simulate_fish(3, 6));
    assert_eq!(2, simulate_fish(3, 7));
    assert_eq!(2, simulate_fish(3, 8));
    assert_eq!(2, simulate_fish(3, 9));
    assert_eq!(2, simulate_fish(3, 10));
    assert_eq!(3, simulate_fish(3, 11));
    assert_eq!(3, simulate_fish(3, 12));
    assert_eq!(4, simulate_fish(3, 13));
    assert_eq!(4, simulate_fish(3, 14));
    assert_eq!(4, simulate_fish(3, 15));
    assert_eq!(4, simulate_fish(3, 16));
    assert_eq!(4, simulate_fish(3, 17));
    assert_eq!(5, simulate_fish(3, 18));

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
