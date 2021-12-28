use crate::Part;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let program = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&program),
            Part::Two => 0,
        }
    )
}

fn search_digit(
    programs: &[Vec<Inst>],
    d: usize,
    start_z: i64,
    cache: &mut HashMap<(usize, i64), Option<i64>>,
) -> Option<i64> {
    // println!(
    //     "Searching for {}th digit, start_z={}, {} known states",
    //     d,
    //     start_z,
    //     cache.len()
    // );
    if let Some(value) = cache.get(&(d, start_z)) {
        return *value;
    }
    // z cannot exceed 26^4 (4 base 26 digits stored in z)
    if start_z > 26i64.pow(4) {
        cache.insert((d, start_z), None);
        return None;
    }

    let mut alu = ALU::new();
    let mut input: [i64; 1] = [9];
    while input[0] > 0 {
        let z = alu.run(&programs[d], start_z, &input);
        if d == 13 {
            // last digit
            if z == 0 {
                return Some(input[0]);
            }
        } else {
            if let Some(value) = search_digit(programs, d + 1, z, cache) {
                cache.insert((d, start_z), Some(value * 10 + input[0]));
                return Some(value * 10 + input[0]);
            }
        }
        input[0] -= 1;
    }

    cache.insert((d, start_z), None);
    None
}

fn reverse_digits(n: i64) -> i64 {
    let mut n = n;
    let mut rev = 0;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev
}

fn part1(program: &[Inst]) -> i64 {
    let mut cache: HashMap<(usize, i64), Option<i64>> = HashMap::new();

    // split the program up into subprograms, each starts with an "imp w"
    // so that they can be run individually when probing
    let mut programs: Vec<Vec<Inst>> = vec![];
    for inst in program {
        if inst.code == Code::Inp {
            programs.push(vec![]);
        }
        let p = programs.len() - 1;
        programs[p].push(inst.clone());
    }

    reverse_digits(search_digit(&programs, 0, 0, &mut cache).unwrap())
}

struct ALU {
    var: [i64; 4],
}

impl ALU {
    fn new() -> ALU {
        ALU { var: [0, 0, 0, 0] }
    }

    // if op is a variable name, get the current value of that variable
    // if op is a literal value, get that value
    fn value(&self, op: Op) -> i64 {
        match op {
            Op::Var(v) => self.var[v as usize],
            Op::Lit(i) => i,
            Op::None => panic!("attempted to get value of empty operand"),
        }
    }

    // run a program.  returns final value of the z register.
    fn run(&mut self, program: &[Inst], start_z: i64, input: &[i64]) -> i64 {
        let mut ip = 0;
        self.var[0] = 0;
        self.var[1] = 0;
        self.var[2] = 0;
        self.var[3] = start_z;

        for inst in program {
            match inst.code {
                Code::Inp => {
                    self.inp(inst.op1, input[ip]);
                    ip += 1;
                }
                Code::Add => self.add(inst.op1, inst.op2),
                Code::Mul => self.mul(inst.op1, inst.op2),
                Code::Div => self.div(inst.op1, inst.op2),
                Code::Mod => self.modulo(inst.op1, inst.op2),
                Code::Eql => self.eql(inst.op1, inst.op2),
            }
        }
        self.var[3]
    }

    fn inp(&mut self, op1: Op, input: i64) {
        self.var[op1.var()] = input;
    }

    fn add(&mut self, op1: Op, op2: Op) {
        self.var[op1.var()] = self.value(op1) + self.value(op2);
    }

    fn mul(&mut self, op1: Op, op2: Op) {
        self.var[op1.var()] = self.value(op1) * self.value(op2);
    }

    fn div(&mut self, op1: Op, op2: Op) {
        self.var[op1.var()] = self.value(op1) / self.value(op2);
    }

    fn modulo(&mut self, op1: Op, op2: Op) {
        self.var[op1.var()] = self.value(op1) % self.value(op2);
    }

    fn eql(&mut self, op1: Op, op2: Op) {
        self.var[op1.var()] = if self.value(op1) == self.value(op2) {
            1
        } else {
            0
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Code {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl From<&str> for Code {
    fn from(s: &str) -> Self {
        match s {
            "inp" => Code::Inp,
            "add" => Code::Add,
            "mul" => Code::Mul,
            "div" => Code::Div,
            "mod" => Code::Mod,
            "eql" => Code::Eql,
            _ => panic!("unknown code {}", s),
        }
    }
}

#[derive(Clone, Copy)]
enum Op {
    Var(u8),
    Lit(i64),
    None,
}

impl Op {
    fn var(&self) -> usize {
        if let Op::Var(v) = self {
            *v as usize
        } else {
            panic!("Attempted to call var() on {:?}", self);
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Var(v) => format!("{}", (v + 119) as char),
                Op::Lit(i) => format!("{}", i),
                Op::None => format!(""),
            }
        )
    }
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "w" => Op::Var(0),
            "x" => Op::Var(1),
            "y" => Op::Var(2),
            "z" => Op::Var(3),
            _ => Op::Lit(s.parse().unwrap()),
        }
    }
}

#[derive(Clone)]
struct Inst {
    code: Code,
    op1: Op,
    op2: Op,
}

impl fmt::Debug for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.code, self.op1, self.op2)
    }
}

impl From<&str> for Inst {
    fn from(s: &str) -> Self {
        let op: Vec<&str> = s.split(' ').collect();
        Self {
            code: Code::from(op[0]),
            op1: Op::from(op[1]),
            op2: if op.len() > 2 {
                Op::from(op[2])
            } else {
                Op::None
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<Inst> {
    input.lines().map(|line| Inst::from(line)).collect()
}

#[test]
fn test() {
    let mut alu = ALU::new();
    let input1 = "\
inp x
mul x -1
";
    let program1 = parse_input(input1);
    alu.run(&program1, 0, &[4]);
    assert_eq!(alu.var[1], -4);

    let input2 = "\
inp z
inp x
mul z 3
eql z x
";
    let program2 = parse_input(input2);
    assert_eq!(0, alu.run(&program2, 0, &[3, 8]));
    assert_eq!(1, alu.run(&program2, 0, &[3, 9]));

    let input3 = "\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";
    let program3 = parse_input(input3);
    alu.run(&program3, 0, &[15]);
    assert_eq!(alu.var, [1, 1, 1, 1]);
    alu.run(&program3, 0, &[0]);
    assert_eq!(alu.var, [0, 0, 0, 0]);
    alu.run(&program3, 0, &[9]);
    assert_eq!(alu.var, [1, 0, 0, 1]);
}
