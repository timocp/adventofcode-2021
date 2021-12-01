use std::fs::File;
use std::io;
use std::io::Read;

mod day1;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 2 {
        run(args[1].parse().unwrap());
    } else {
        for day in 1..25 {
            run(day)
        }
    }
}

fn run(day: i32) {
    let filename = format!("input/day{}.txt", day);
    match read_file(&filename) {
        Ok(data) => match day {
            1 => day1::run(&data),
            _ => eprintln!("Day {} not implemented", day),
        },
        Err(e) => eprintln!("{}: {}", filename, e),
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
