pub fn run(input: &str) {
    let lines = parse_input(input);
    println!("Day 10, part one: {}", syntax_error_score(&lines));
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

fn syntax_error_score(lines: &[String]) -> usize {
    let mut score = 0;
    for line in lines {
        let mut stack: Vec<char> = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(o) = stack.pop() {
                        if o != matching_char_for(c) {
                            score += illegal_char_score(c);
                            break;
                        }
                    } else {
                        score += illegal_char_score(c);
                        break;
                    }
                }
                _ => panic!("unexpected token: {}", c),
            }
        }
    }
    score
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
    assert_eq!(26397, syntax_error_score(&lines));
}
