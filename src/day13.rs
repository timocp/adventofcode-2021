use crate::Part;
use std::collections::HashSet;

pub fn run(input: &str, part: Part) -> String {
    let (paper, folds) = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => fold_paper(&paper, folds[0]).len(),
            Part::Two => 0,
        }
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}

// set of dots are (row, column)
type Paper = HashSet<(usize, usize)>;

#[allow(dead_code)]
fn print_paper(paper: &Paper) -> String {
    let mut s = String::new();
    let maxrow = *paper.iter().map(|(row, _)| row).max().unwrap();
    let maxcol = *paper.iter().map(|(_, col)| col).max().unwrap();
    for row in 0..=maxrow {
        for col in 0..=maxcol {
            if paper.contains(&(row, col)) {
                s += "#";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    s
}

fn fold_paper(paper: &Paper, fold: Fold) -> Paper {
    let mut new = Paper::new();
    match fold {
        Fold::Y(y) => {
            for (row, col) in paper.iter() {
                if *row > y {
                    // any point BELOW the fold moves up
                    new.insert((y - (row - y), *col));
                } else {
                    new.insert((*row, *col));
                }
            }
        }
        Fold::X(x) => {
            for (row, col) in paper.iter() {
                if *col > x {
                    // any point to the RIGHT of the fold moves left
                    new.insert((*row, x - (col - x)));
                } else {
                    new.insert((*row, *col));
                }
            }
        }
    };
    new
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let mut section = 0;
    let mut paper = Paper::new();
    let mut folds = vec![];

    for line in input.lines() {
        if line.is_empty() {
            section = 1;
        } else if section == 0 {
            let dots: Vec<usize> = line.split(',').map(|num| num.parse().unwrap()).collect();
            paper.insert((dots[1], dots[0]));
        } else {
            let i: Vec<&str> = line.split('=').collect();
            folds.push(match i[0] {
                "fold along y" => Fold::Y(i[1].parse().unwrap()),
                "fold along x" => Fold::X(i[1].parse().unwrap()),
                _ => unreachable!(),
            });
        }
    }
    (paper, folds)
}

#[test]
fn test() {
    let test_input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    let (paper, folds) = parse_input(test_input);
    assert_eq!(18, paper.len());
    assert_eq!(2, folds.len());
    assert_eq!(Fold::Y(7), folds[0]);
    assert_eq!(Fold::X(5), folds[1]);
    let paper = fold_paper(&paper, folds[0]);
    assert_eq!(17, paper.len());
    let paper = fold_paper(&paper, folds[1]);
    assert_eq!(
        "\
#####
#...#
#...#
#...#
#####
",
        print_paper(&paper)
    );
}
