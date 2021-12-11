use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let entries = parse_input(input);
    match part {
        Part::One => format!("{}", count_part1(&entries)),
        Part::Two => "(still thinking)".to_string(),
    }
}

struct Entry<'a> {
    signals: [&'a str; 10],
    output: [&'a str; 4],
}

impl Entry<'_> {
    fn count_part1(&self) -> usize {
        self.output
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    }
}

fn count_part1(entries: &[Entry]) -> usize {
    entries.iter().map(|entry| entry.count_part1()).sum()
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let input: Vec<Vec<&str>> = line
                .split(" | ")
                .map(|strings| strings.split(' ').collect())
                .collect();
            let l = &input[0];
            let r = &input[1];
            Entry {
                signals: [l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7], l[8], l[9]],
                output: [r[0], r[1], r[2], r[3]],
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
    assert_eq!("be", entries[0].signals[0]);
    assert_eq!("edb", entries[0].signals[9]);
    assert_eq!("fgae", entries[9].output[0]);
    assert_eq!("bagce", entries[9].output[3]);
    assert_eq!(26, count_part1(&entries));
}
