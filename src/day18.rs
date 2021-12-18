use crate::Part;
use std::fmt;
use std::ops::Add;

pub fn run(input: &str, part: Part) -> String {
    let sf_numbers = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&sf_numbers),
            Part::Two => 0,
        }
    )
}

fn part1(input: &[SfNumber]) -> u32 {
    add_sf_list(input).magnitude()
}

fn parse_input(input: &str) -> Vec<SfNumber> {
    input.lines().map(SfNumber::from).collect()
}

fn add_sf_list(list: &[SfNumber]) -> SfNumber {
    // why not able to use an iterator?
    // *list.iter().reduce(|acc, sf_num| &(acc + sf_num)).unwrap()
    // Error[E0515]: cannot return reference to temporary value

    let mut acc: SfNumber = &list[0] + &list[1];
    if list.len() > 2 {
        for num in list.iter().skip(2) {
            acc = &acc + num;
        }
    }
    acc
}

// An earlier attempt of this tried using an actual tree data structure, but being able to do the
// explode operation, which requires mutating far parts of the tree proved too difficult (for me)
// in rust.
//
// Instead, the number is stored as a vector of tokens from the string.  This allows for much
// simpler in-place modification during explode().

#[derive(Clone, Copy, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Comma,
    Number(u32),
}

impl Token {
    fn number(&self) -> Option<u32> {
        if let Token::Number(num) = self {
            Some(*num)
        } else {
            None
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            '0'..='9' => Token::Number(c as u32 - 48),
            _ => panic!("Unexpected input character: {:?}", c),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::LeftBracket => "[".to_string(),
                Token::RightBracket => "]".to_string(),
                Token::Comma => ",".to_string(),
                Token::Number(num) => format!("{}", num),
            }
        )
    }
}

struct SfNumber {
    n: Vec<Token>,
}

impl From<&str> for SfNumber {
    fn from(s: &str) -> Self {
        Self {
            n: s.chars().map(Token::from).collect(),
        }
    }
}

impl Add for &SfNumber {
    type Output = SfNumber;

    fn add(self, other: Self) -> SfNumber {
        let mut n = Vec::with_capacity(self.n.len() + other.n.len() + 3);
        n.push(Token::LeftBracket);
        n.extend(self.n.iter());
        n.push(Token::Comma);
        n.extend(other.n.iter());
        n.push(Token::RightBracket);
        let mut new = SfNumber { n };
        new.reduce();
        new
    }
}

impl SfNumber {
    fn reduce(&mut self) {
        // println!("REDUCING:      {:?}", self);
        loop {
            if self.explode() {
                // println!("After explode: {:?}", self);
                continue;
            }
            if self.split() {
                // println!("After split:   {:?}", self);
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        let mut target: Option<usize> = None;

        for (i, c) in self.n.iter().enumerate() {
            match c {
                Token::LeftBracket => {
                    if depth == 4 {
                        target = Some(i);
                        break;
                    }
                    depth += 1;
                }
                Token::RightBracket => {
                    depth -= 1;
                }
                _ => (),
            }
        }

        if let Some(target) = target {
            // replace this pair with a zero
            let removed: Vec<Token> = self
                .n
                .splice(target..(target + 5), vec![Token::Number(0)])
                .collect();
            let lv = removed[1].number().unwrap();
            let rv = removed[3].number().unwrap();

            // copy the removed pair to the nearest value left or right (if present)
            for i in (0..target).rev() {
                if let Token::Number(num) = self.n[i] {
                    self.n[i] = Token::Number(num + lv);
                    break;
                }
            }
            for i in (target + 1)..self.n.len() {
                if let Token::Number(num) = self.n[i] {
                    self.n[i] = Token::Number(num + rv);
                    break;
                }
            }
            return true;
        }

        false
    }

    fn split(&mut self) -> bool {
        let mut target: Option<usize> = None;

        for (i, c) in self.n.iter().enumerate() {
            if let Token::Number(v) = c {
                if *v > 9 {
                    target = Some(i);
                    break;
                }
            }
        }

        if let Some(target) = target {
            // number is too high, replace with a pair
            let v = self.n[target].number().unwrap();
            self.n.splice(
                target..=target,
                vec![
                    Token::LeftBracket,
                    Token::Number(v / 2),
                    Token::Comma,
                    Token::Number((v + 1) / 2),
                    Token::RightBracket,
                ],
            );
            return true;
        }
        false
    }

    fn magnitude(&self) -> u32 {
        token_magnitude(&self.n)
    }
}

fn find_comma(tokens: &[Token]) -> usize {
    let mut depth = 0;
    for (i, c) in tokens.iter().enumerate() {
        match c {
            Token::LeftBracket => depth += 1,
            Token::RightBracket => depth -= 1,
            Token::Comma => {
                if depth == 1 {
                    return i;
                }
            }
            Token::Number(_) => (),
        }
    }
    unreachable!("Unable to find comma in {:?}", tokens);
}

fn token_magnitude(tokens: &[Token]) -> u32 {
    assert_eq!(Token::LeftBracket, tokens[0]);
    assert_eq!(Token::RightBracket, tokens[tokens.len() - 1]);

    let comma = find_comma(tokens);
    let left = match tokens[comma - 1] {
        Token::Number(num) => num,
        Token::RightBracket => token_magnitude(&tokens[1..comma]),
        _ => unreachable!(),
    };
    let right = match tokens[comma + 1] {
        Token::Number(num) => num,
        Token::LeftBracket => token_magnitude(&tokens[(comma + 1)..tokens.len() - 1]),
        _ => unreachable!(),
    };
    left * 3 + right * 2
}

impl fmt::Debug for SfNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.n
                .iter()
                .map(|t| format!("{:?}", t))
                .collect::<String>()
        )
    }
}

#[test]
fn test() {
    let test_input = "\
[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
";
    let numbers = parse_input(test_input);
    assert_eq!(7, numbers.len());
    assert_eq!(
        test_input,
        numbers
            .iter()
            .map(|sf| format!("{:?}\n", sf))
            .collect::<Vec<String>>()
            .join("")
    );
    assert_eq!(
        "[[1,2],[[3,4],5]]",
        format!(
            "{:?}",
            &SfNumber::from("[1,2]") + &SfNumber::from("[[3,4],5]")
        )
    );
    assert_eq!(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        format!(
            "{:?}",
            &SfNumber::from("[[[[4,3],4],4],[7,[[8,4],9]]]") + &SfNumber::from("[1,1]")
        )
    );

    let test_input = "\
[1,1]
[2,2]
[3,3]
[4,4]
";
    assert_eq!(
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        format!("{:?}", add_sf_list(&parse_input(test_input)))
    );

    let test_input = "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
";
    assert_eq!(
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        format!("{:?}", add_sf_list(&parse_input(test_input)))
    );

    let test_input = "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";
    assert_eq!(
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        format!("{:?}", add_sf_list(&parse_input(test_input)))
    );

    let test_input = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";
    assert_eq!(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        format!("{:?}", add_sf_list(&parse_input(test_input)))
    );

    assert_eq!(143, SfNumber::from("[[1,2],[[3,4],5]]").magnitude());
    assert_eq!(
        1384,
        SfNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude()
    );
    assert_eq!(
        445,
        SfNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude()
    );
    assert_eq!(
        791,
        SfNumber::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude()
    );
    assert_eq!(
        1137,
        SfNumber::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude()
    );
    assert_eq!(
        3488,
        SfNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude()
    );

    let test_input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
    assert_eq!(4140, part1(&parse_input(test_input)));
}
