pub fn run(input: &str) {
    let lines = parse_input(input);
    println!("Day 10, part one: {}", part1(&lines));
    println!("Day 10, part two: {}", part2(&lines));
}

fn illegal_char_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected token {}", c),
    }
}

fn matching_char_for(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unexpected token {}", c),
    }
}

fn matching_char_score(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected token {}", c),
    }
}

enum Result {
    Corrupted(usize),
    Incomplete(Vec<char>),
}

// if line is corrupt, return Corrupted<score>
// if line is not corrupt, return Incomplete<stack> of the leftover stack
fn parse(line: &str) -> Result {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(o) = stack.pop() {
                    if o != matching_char_for(c) {
                        return Result::Corrupted(illegal_char_score(c));
                    }
                } else {
                    return Result::Corrupted(illegal_char_score(c));
                }
            }
            _ => panic!("unexpected token: {}", c),
        }
    }
    Result::Incomplete(stack)
}

fn part1(lines: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|line| {
            if let Result::Corrupted(score) = parse(line) {
                Some(score)
            } else {
                None
            }
        })
        .sum()
}

fn completion_score(stack: &[char]) -> usize {
    stack
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + matching_char_score(*c))
}

fn part2(lines: &[String]) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            if let Result::Incomplete(stack) = parse(line) {
                Some(stack)
            } else {
                None
            }
        })
        .map(|stack| completion_score(&stack))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[test]
fn test() {
    let test_input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    let lines = parse_input(test_input);
    assert_eq!(10, lines.len());
    assert_eq!(26397, part1(&lines));
    assert_eq!(288957, part2(&lines));
}
