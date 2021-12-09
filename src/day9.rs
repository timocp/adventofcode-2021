pub fn run(input: &str) {
    let map = parse_input(input);
    println!("Day 9, part one: {}", part1(&map));
}

fn part1(map: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    let height = map.len();
    let width = map[0].len();
    for x in 0..width {
        for y in 0..height {
            let this = map[y][x];
            if (y == 0 || map[y - 1][x] > this)
                && (y == height - 1 || map[y + 1][x] > this)
                && (x == 0 || map[y][x - 1] > this)
                && (x == width - 1 || map[y][x + 1] > this)
            {
                println!("{},{} is a low point", y, x);
                sum += this + 1;
            }
        }
    }
    sum
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[test]
fn test() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    let map = parse_input(&test_input);
    dbg!(&map);
    assert_eq!(15, part1(&map));
}
