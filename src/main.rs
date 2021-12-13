use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::slice::Iter;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn each() -> Iter<'static, Part> {
        static PARTS: [Part; 2] = [Part::One, Part::Two];
        PARTS.iter()
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 2 {
        let day = args[1].parse().unwrap();
        run(day);
    } else {
        let t0 = Instant::now();
        for day in 1..25 {
            run(day);
        }
        println!(
            "{:>73} {1:.3}s",
            "TOTAL:",
            Instant::now().duration_since(t0).as_secs_f64()
        );
    }
}

fn run(day: i32) {
    let filename = format!("input/day{}.txt", day);
    match read_file(&filename) {
        Ok(input) => {
            for &part in Part::each() {
                print!("Day {:02}, part {}:  ", day, part);
                let t0 = Instant::now();
                let result = match day {
                    1 => day1::run(&input, part),
                    2 => day2::run(&input, part),
                    3 => day3::run(&input, part),
                    4 => day4::run(&input, part),
                    5 => day5::run(&input, part),
                    6 => day6::run(&input, part),
                    7 => day7::run(&input, part),
                    8 => day8::run(&input, part),
                    9 => day9::run(&input, part),
                    10 => day10::run(&input, part),
                    11 => day11::run(&input, part),
                    12 => day12::run(&input, part),
                    13 => day13::run(&input, part),
                    _ => "Not implemented".to_string(),
                };
                println!(
                    "{:56} {1:.3}s",
                    result,
                    Instant::now().duration_since(t0).as_secs_f64()
                );
            }
        }
        Err(e) => eprintln!("{}: {}", filename, e),
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
