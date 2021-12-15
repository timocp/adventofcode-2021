use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let entries = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => count_part1(&entries),
            Part::Two => count_part2(&entries),
        }
    )
}

#[derive(Debug)]
struct Entry {
    signals: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl Entry {
    fn count_part1(&self) -> usize {
        self.output
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    }

    // references to positions:
    //  0000
    // 1    2
    // 1    2
    //  3333
    // 4    5
    // 4    5
    //  6666
    fn value(&self) -> usize {
        // work out which signal character belongs to which position via logical elimination. not a
        // general solution, makes lots of assumptions about the input but works
        let mut pos: [Option<&char>; 7] = [None; 7];

        // words known by number of active signals
        let one = self.signals.iter().find(|s| s.len() == 2).unwrap();
        let four = self.signals.iter().find(|s| s.len() == 4).unwrap();
        let seven = self.signals.iter().find(|s| s.len() == 3).unwrap();
        let eight = self.signals.iter().find(|s| s.len() == 7).unwrap();

        // unknown numbers by length
        let len5: Vec<Vec<char>> = self
            .signals
            .clone()
            .into_iter()
            .filter(|s| s.len() == 5)
            .collect(); // 2, 3 and 5
        let len6: Vec<Vec<char>> = self
            .signals
            .clone()
            .into_iter()
            .filter(|s| s.len() == 6)
            .collect(); // 0, 6 and 9

        // position 0 is the signal in 7 but not 1
        pos[0] = seven.iter().find(|c| !one.contains(c));

        // position 1 is the character in 4 which is only mentioned once in 5 digit numbers
        pos[1] = four
            .iter()
            .find(|c| len5.iter().filter(|w| w.contains(c)).count() == 1);

        // position 2 is the character in 1 which is mentioned twice in 6 digit numbers
        pos[2] = one
            .iter()
            .find(|c| len6.iter().filter(|w| w.contains(c)).count() == 2);

        // position 3 is the character in 4 but not 1, and not in position 1
        pos[3] = four
            .iter()
            .find(|c| !one.contains(c) && *c != pos[1].unwrap());

        // position 5 is the character in 1 that wasn't put into position 2
        pos[5] = one.iter().find(|c| *c != pos[2].unwrap());

        // only 2 characters are remaining
        let remaining: Vec<char> = eight
            .iter()
            .copied()
            .filter(|c| !pos.contains(&Some(c)))
            .collect();

        // position 4 is the remaining character mentioned once in 5 digit numbers
        pos[4] = remaining
            .iter()
            .find(|c| len5.iter().filter(|w| w.contains(c)).count() == 1);

        // position 6 is whatever's left
        pos[6] = remaining.iter().find(|c| *c != pos[4].unwrap());

        // pos is filled, remove options and refs
        let pos = pos.map(|p| *p.unwrap());

        // the output lights can now be mapped to real numbers
        self.output
            .iter()
            .map(|o| self.decode(&pos, o))
            .fold(0, |acc, c| acc * 10 + c)
    }

    fn decode(&self, pos: &[char], output: &[char]) -> usize {
        match output.len() {
            7 => 8,
            6 => {
                if !output.contains(&pos[3]) {
                    0
                } else if !output.contains(&pos[2]) {
                    6
                } else {
                    9
                }
            }
            5 => {
                if output.contains(&pos[4]) {
                    2
                } else if !output.contains(&pos[1]) {
                    3
                } else {
                    5
                }
            }
            4 => 4,
            3 => 7,
            2 => 1,
            _ => unreachable!(),
        }
    }
}

fn count_part1(entries: &[Entry]) -> usize {
    entries.iter().map(|entry| entry.count_part1()).sum()
}

fn count_part2(entries: &[Entry]) -> usize {
    entries.iter().map(|entry| entry.value()).sum()
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let input: Vec<Vec<Vec<char>>> = line
                .split(" | ")
                .map(|strings| strings.split(' ').map(|s| s.chars().collect()).collect())
                .collect();
            Entry {
                signals: input[0].to_owned(),
                output: input[1].to_owned(),
            }
        })
        .collect()
}

#[test]
fn test() {
    let test_input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    let entries = parse_input(test_input);
    assert_eq!(10, entries.len());
    assert_eq!(vec!['b', 'e'], entries[0].signals[0]);
    assert_eq!(vec!['e', 'd', 'b'], entries[0].signals[9]);
    assert_eq!(vec!['f', 'g', 'a', 'e'], entries[9].output[0]);
    assert_eq!(vec!['b', 'a', 'g', 'c', 'e'], entries[9].output[3]);
    assert_eq!(26, count_part1(&entries));
    assert_eq!(8394, entries[0].value());
    assert_eq!(9781, entries[1].value());
    assert_eq!(1197, entries[2].value());
    assert_eq!(9361, entries[3].value());
    assert_eq!(4873, entries[4].value());
    assert_eq!(8418, entries[5].value());
    assert_eq!(4548, entries[6].value());
    assert_eq!(1625, entries[7].value());
    assert_eq!(8717, entries[8].value());
    assert_eq!(4315, entries[9].value());
    assert_eq!(61229, count_part2(&entries));

    let test_input2 =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n";
    let entries = parse_input(test_input2);
    assert_eq!(5353, entries[0].value());
}
