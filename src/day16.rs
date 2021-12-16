use crate::Part;
use std::collections::VecDeque;

pub fn run(input: &str, part: Part) -> String {
    format!(
        "{}",
        match part {
            Part::One => parse_input(input).sum_versions(),
            Part::Two => 0,
        }
    )
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    content: Content,
}

impl Packet {
    #[allow(dead_code)]
    fn literal(&self) -> u64 {
        if let Content::Literal(n) = self.content {
            n
        } else {
            panic!("Not a literal value")
        }
    }

    #[allow(dead_code)]
    fn subpacket(&self, i: usize) -> &Packet {
        if let Content::Subpackets(subpackets) = &self.content {
            &subpackets[i]
        } else {
            panic!("No subpackets")
        }
    }

    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.content {
                Content::Literal(_) => 0,
                Content::Subpackets(subs) => subs.iter().map(|p| p.sum_versions()).sum(),
            }
    }
}

fn parse_number(size: u8, bits: &mut VecDeque<u8>) -> u64 {
    let mut value: u64 = 0;
    for _ in 0..size {
        value <<= 1;
        value += bits.pop_front().unwrap() as u64;
    }
    value
}

fn parse_packet(bits: &mut VecDeque<u8>) -> Packet {
    let version = parse_number(3, bits) as u8;
    let type_id = parse_number(3, bits) as u8;
    let content: Content;

    if type_id == 4 {
        // literal number
        let mut number: u64 = 0;
        loop {
            let group = parse_number(5, bits);
            number <<= 4;
            number += group & 0xf;
            if group & 0x10 == 0 {
                break;
            }
        }
        content = Content::Literal(number);
    } else {
        // operator, contains subpackets
        let mut subs = vec![];
        match bits.pop_front().unwrap() {
            0 => {
                // indicates number of bits that make up the subpackets
                let sub_length = parse_number(15, bits) as usize;
                let start_size = bits.len();
                while start_size - bits.len() < sub_length {
                    subs.push(parse_packet(bits));
                }
            }
            1 => {
                // indicates number of subpackets
                let count = parse_number(11, bits);
                for _ in 0..count {
                    subs.push(parse_packet(bits));
                }
            }
            _ => unreachable!(),
        }
        content = Content::Subpackets(subs);
    }

    Packet {
        version,
        type_id,
        content,
    }
}

// hex string into a queue of bits
fn parse_bits(input: &str) -> VecDeque<u8> {
    let mut bits = VecDeque::new();
    for c in input.chars() {
        if let Some(n) = c.to_digit(16) {
            for i in 0..4 {
                // 8, 4, 2, 1
                bits.push_back(if n & 1 << (3 - i) == 0 { 0 } else { 1 })
            }
        }
    }
    bits
}

// input string to Packet
fn parse_input(input: &str) -> Packet {
    parse_packet(&mut parse_bits(input))
}

#[test]
fn test() {
    let test_input = "D2FE28";
    assert_eq!(
        VecDeque::from(vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0
        ]),
        parse_bits(test_input)
    );
    let p = parse_packet(&mut parse_bits(test_input));
    assert_eq!(6, p.version);
    assert_eq!(4, p.type_id);
    assert_eq!(2021, p.literal());

    let test_input = "38006F45291200";
    assert_eq!(
        VecDeque::from(vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]),
        parse_bits(test_input)
    );
    let p = parse_packet(&mut parse_bits(test_input));
    assert_eq!(1, p.version);
    assert_eq!(6, p.type_id);
    assert_eq!(6, p.subpacket(0).version);
    assert_eq!(4, p.subpacket(0).type_id);
    assert_eq!(10, p.subpacket(0).literal());
    assert_eq!(2, p.subpacket(1).version);
    assert_eq!(4, p.subpacket(1).type_id);
    assert_eq!(20, p.subpacket(1).literal());

    let test_input = "EE00D40C823060";
    assert_eq!(
        VecDeque::from(vec![
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0
        ]),
        parse_bits(test_input)
    );
    let p = parse_packet(&mut parse_bits(test_input));
    assert_eq!(7, p.version);
    assert_eq!(3, p.type_id);
    assert_eq!(2, p.subpacket(0).version);
    assert_eq!(4, p.subpacket(0).type_id);
    assert_eq!(1, p.subpacket(0).literal());
    assert_eq!(4, p.subpacket(1).version);
    assert_eq!(4, p.subpacket(1).type_id);
    assert_eq!(2, p.subpacket(1).literal());
    assert_eq!(1, p.subpacket(2).version);
    assert_eq!(4, p.subpacket(2).type_id);
    assert_eq!(3, p.subpacket(2).literal());

    assert_eq!(16, parse_input("8A004A801A8002F478").sum_versions());
    assert_eq!(12, parse_input("620080001611562C8802118E34").sum_versions());
    assert_eq!(
        23,
        parse_input("C0015000016115A2E0802F182340").sum_versions()
    );
    assert_eq!(
        31,
        parse_input("A0016C880162017C3686B18A3D4780").sum_versions()
    );
}
