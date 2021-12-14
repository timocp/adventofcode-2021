use crate::Part;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let (mut polymer, rules) = parse_input(input);
    match part {
        Part::One => format!("{}", part1(&mut polymer, &rules)),
        Part::Two => "?".to_string(),
    }
}

fn part1(polymer: &mut Polymer, rules: &RuleSet) -> usize {
    for _ in 0..10 {
        polymer.apply(rules);
    }
    let tally = polymer.tally();
    tally.values().max().unwrap() - tally.values().min().unwrap()
}

// would it be faster to use a real linked list?  there are a lot of inserts.
// would it be faster to return a new owned vec instead of modifying in-place?
struct Polymer(VecDeque<char>);

impl Polymer {
    fn new(input: &str) -> Polymer {
        let mut list = VecDeque::new();
        for c in input.chars() {
            list.push_back(c);
        }
        Polymer(list)
    }

    fn apply(&mut self, rule: &RuleSet) {
        let mut i = 0;
        while i < self.0.len() - 1 {
            let c = *self.0.get(i).unwrap();
            let nc = *self.0.get(i + 1).unwrap();
            if let Some(ins) = rule.get(&(c, nc)) {
                self.0.insert(i + 1, *ins);
                i += 1;
            }
            i += 1;
        }
    }

    fn tally(&self) -> HashMap<char, usize> {
        let mut map = HashMap::new();

        for c in self.0.iter() {
            *map.entry(*c).or_insert(0) += 1;
        }

        map
    }
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for c in self.0.iter() {
            write!(f, "{}", c)?;
        })
    }
}

type RuleSet = HashMap<(char, char), char>;

fn parse_input(input: &str) -> (Polymer, RuleSet) {
    let mut section = 0;
    let mut polymer = Polymer::new("");
    let mut ruleset = RuleSet::new();

    for line in input.lines() {
        if line.is_empty() {
            section = 1;
        } else if section == 0 {
            polymer = Polymer::new(line);
        } else {
            let rule: Vec<&str> = line.split(" -> ").collect();
            let from: Vec<char> = rule[0].chars().collect();
            let to: char = rule[1].chars().next().unwrap();
            ruleset.insert((from[0], from[1]), to);
        }
    }

    (polymer, ruleset)
}

#[test]
fn test() {
    let test_input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    let (mut polymer, rules) = parse_input(test_input);
    assert_eq!("NNCB", format!("{}", polymer));
    assert_eq!(16, rules.len());
    polymer.apply(&rules);
    assert_eq!("NCNBCHB", format!("{}", polymer));
    polymer.apply(&rules);
    assert_eq!("NBCCNBBBCBHCB", format!("{}", polymer));
    polymer.apply(&rules);
    assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", format!("{}", polymer));
    polymer.apply(&rules);
    assert_eq!(
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
        format!("{}", polymer)
    );
    polymer.apply(&rules);
    polymer.apply(&rules);
    polymer.apply(&rules);
    polymer.apply(&rules);
    polymer.apply(&rules);
    polymer.apply(&rules);
    assert_eq!(3073, format!("{}", polymer).len());

    let (mut polymer, rules) = parse_input(test_input);
    assert_eq!(1588, part1(&mut polymer, &rules));
}
