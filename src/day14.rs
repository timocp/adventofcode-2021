use crate::Part;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let (polymer, rules) = parse_input(input);
    format!(
        "{}",
        apply(
            &polymer,
            &rules,
            match part {
                Part::One => 10,
                Part::Two => 40,
            }
        )
    )
}

fn apply(polymer: &Polymer, rules: &RuleSet, times: usize) -> usize {
    let mut p: Polymer = polymer.apply(rules);
    for _ in 1..times {
        p = p.apply(rules);
    }
    let tally = p.tally();
    tally.values().max().unwrap() - tally.values().min().unwrap()
}

// store polymer as the numbers of element pairs.  the final character
// is paired with a '$' indicating the end of the polymer.
struct Polymer(HashMap<(char, char), usize>);

impl Polymer {
    fn new(input: &str) -> Polymer {
        let mut hash = HashMap::new();
        let chars: Vec<char> = input.chars().collect();
        if !chars.is_empty() {
            for i in 0..(chars.len() - 1) {
                *hash.entry((chars[i], chars[i + 1])).or_insert(0) += 1;
            }
            hash.insert((chars[chars.len() - 1], '$'), 1);
        }
        Polymer(hash)
    }

    fn apply(&self, rules: &RuleSet) -> Polymer {
        let mut hash: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in self.0.iter() {
            if pair.1 == '$' {
                hash.insert(*pair, *count);
            } else if let Some(ins) = rules.get(&(pair.0, pair.1)) {
                *hash.entry((pair.0, *ins)).or_insert(0) += count;
                *hash.entry((*ins, pair.1)).or_insert(0) += count;
            } else {
                panic!("No rule for {:?}", pair);
            }
        }
        Polymer(hash)
    }

    fn tally(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for (pair, count) in self.0.iter() {
            *counts.entry(pair.0).or_insert(0) += count;
        }
        counts
    }
}

// Rules are a map of character pairs to the character to insert between them
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
    let (polymer, rules) = parse_input(test_input);
    assert_eq!(16, rules.len());
    assert_eq!(1588, apply(&polymer, &rules, 10));
    assert_eq!(2188189693529, apply(&polymer, &rules, 40));
}
