use crate::Part;
use std::collections::VecDeque;

pub fn run(input: &str, part: Part) -> String {
    let packet = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => packet.sum_versions(),
            Part::Two => packet.value(),
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
    fn literal(&self) -> u64 {
        if let Content::Literal(n) = self.content {
            n
        } else {
            panic!("Not a literal value");
        }
    }

    fn get_subpacket(&self, i: usize) -> &Packet {
        if let Content::Subpackets(subpackets) = &self.content {
            &subpackets[i]
        } else {
            panic!("No subpackets");
        }
    }

    fn sub_values(&self) -> impl Iterator<Item = u64> + '_ {
        if let Content::Subpackets(subs) = &self.content {
            subs.iter().map(|sub| sub.value())
        } else {
            panic!("No subpackets");
        }
    }

    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.content {
                Content::Literal(_) => 0,
                Content::Subpackets(subs) => subs.iter().map(|p| p.sum_versions()).sum(),
            }
    }

    fn value(&self) -> u64 {
        match self.type_id {
            0 => self.sum(),
            1 => self.product(),
            2 => self.minimum(),
            3 => self.maximum(),
            4 => self.literal(),
            5 => self.greater_than(),
            6 => self.less_than(),
            7 => self.equal_to(),
            _ => panic!("Unexpected type ID: {}", self.type_id),
        }
    }

    fn sum(&self) -> u64 {
        self.sub_values().sum()
    }

    fn product(&self) -> u64 {
        self.sub_values().product()
    }

    fn minimum(&self) -> u64 {
        self.sub_values().min().unwrap()
    }

    fn maximum(&self) -> u64 {
        self.sub_values().max().unwrap()
    }

    fn greater_than(&self) -> u64 {
        if self.get_subpacket(0).value() > self.get_subpacket(1).value() {
            1
        } else {
            0
        }
    }

    fn less_than(&self) -> u64 {
        if self.get_subpacket(0).value() < self.get_subpacket(1).value() {
            1
        } else {
            0
        }
    }

    fn equal_to(&self) -> u64 {
        if self.get_subpacket(0).value() == self.get_subpacket(1).value() {
            1
        } else {
            0
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
    assert_eq!(6, p.get_subpacket(0).version);
    assert_eq!(4, p.get_subpacket(0).type_id);
    assert_eq!(10, p.get_subpacket(0).literal());
    assert_eq!(2, p.get_subpacket(1).version);
    assert_eq!(4, p.get_subpacket(1).type_id);
    assert_eq!(20, p.get_subpacket(1).literal());

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
    assert_eq!(2, p.get_subpacket(0).version);
    assert_eq!(4, p.get_subpacket(0).type_id);
    assert_eq!(1, p.get_subpacket(0).literal());
    assert_eq!(4, p.get_subpacket(1).version);
    assert_eq!(4, p.get_subpacket(1).type_id);
    assert_eq!(2, p.get_subpacket(1).literal());
    assert_eq!(1, p.get_subpacket(2).version);
    assert_eq!(4, p.get_subpacket(2).type_id);
    assert_eq!(3, p.get_subpacket(2).literal());

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

    assert_eq!(3, parse_input("C200B40A82").value());
    assert_eq!(54, parse_input("04005AC33890").value());
    assert_eq!(7, parse_input("880086C3E88112").value());
    assert_eq!(9, parse_input("CE00C43D881120").value());
    assert_eq!(1, parse_input("D8005AC2A8F0").value());
    assert_eq!(0, parse_input("F600BC2D8F").value());
    assert_eq!(0, parse_input("9C005AC2F8F0").value());
    assert_eq!(1, parse_input("9C0141080250320F1802104A08").value());
}
