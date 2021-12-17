use crate::Part;
use regex::Regex;
use std::ops::Range;

pub fn run(input: &str, part: Part) -> String {
    let target = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => search_highest_shot(&target),
            Part::Two => 0,
        }
    )
}

#[derive(Debug)]
struct Target {
    x: Range<i64>,
    y: Range<i64>,
}

impl Target {
    fn contains(&self, x: i64, y: i64) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

// Returns the max height the probe reached if it lands in the target area
// Returns None if it misses
fn fire(dx: i64, dy: i64, target: &Target) -> Option<i64> {
    let mut dx = dx;
    let mut dy = dy;
    let mut x = 0;
    let mut y = 0;
    let mut max_y = y;
    while x <= target.x.end && y >= target.y.end {
        x += dx;
        y += dy;
        if y > max_y {
            max_y = y
        }
        if target.contains(x, y) {
            return Some(max_y);
        }
        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;
    }
    // too far beyond or below
    None
}

// possible starting x velocities that could end up in the X target range
fn x_candidates(target: &Target) -> Vec<i64> {
    let mut valid = vec![];
    'outer: for cand in 1..target.x.end {
        let mut x = 0;
        let mut dx = cand;
        while x <= target.x.end && dx > 0 {
            x += dx;
            if target.x.contains(&x) {
                valid.push(cand);
                continue 'outer;
            }
            dx -= 1;
        }
    }
    valid
}

// possible starting y velocities that could end up in the Y target range
// assumes a positive starting Y value velocity (as we are after max height)
fn y_candidates(target: &Target) -> Vec<i64> {
    let mut valid = vec![];
    // not sure how to limit starting dy here, just pick a big number
    'outer: for cand in 1..500 {
        let mut y = 0;
        let mut dy = cand;
        while y > target.y.start {
            y += dy;
            if target.y.contains(&y) {
                valid.push(cand);
                continue 'outer;
            }
            dy -= 1;
        }
    }
    valid
}

fn search_highest_shot(target: &Target) -> i64 {
    let mut max_height = 0;

    let x_candidates = x_candidates(target);
    let y_candidates = y_candidates(target);
    for &x in x_candidates.iter() {
        for &y in y_candidates.iter() {
            if let Some(height) = fire(x, y, target) {
                if height > max_height {
                    max_height = height;
                }
            }
        }
    }

    max_height
}

fn parse_input(input: &str) -> Target {
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let m = re.captures(input).unwrap();
    Target {
        x: Range {
            start: m[1].parse().unwrap(),
            end: m[2].parse::<i64>().unwrap() + 1, // +1 as rust ranges are half-open
        },
        y: Range {
            start: m[3].parse().unwrap(),
            end: m[4].parse::<i64>().unwrap() + 1,
        },
    }
}

#[test]
fn test() {
    let test_input = "target area: x=20..30, y=-10..-5\n";
    let target = parse_input(test_input);
    println!("{:?}", target);
    assert_eq!(20..31, target.x);
    assert_eq!(-10..-4, target.y);
    assert!(target.contains(20, -10));
    assert!(target.contains(30, -6));
    assert!(!target.contains(19, -10));

    assert_eq!(Some(3), fire(7, 2, &target));
    assert_eq!(Some(6), fire(6, 3, &target));
    assert_eq!(Some(0), fire(9, 0, &target));
    assert_eq!(None, fire(17, -4, &target));

    assert_eq!(45, search_highest_shot(&target));
}
