use std::fs;

#[derive(Debug)]
enum Packet {
    Literal {
        version: i32,
        ttype: i32,
        value: i64,
        bits_used: usize,
    },
    Operator {
        version: i32,
        op: Op,
        length_type: LengthType,
        children: Vec<Packet>,
    },
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    GT,
    LT,
    Eq,
}

#[derive(Debug, Clone, Copy)]
enum LengthType {
    NumBits(i64),
    NumPackets(i64),
}

fn parse_number(slice: &[i8]) -> i64 {
    slice.iter().fold(0, |acc, &b| (acc << 1) + b as i64)
}

fn parse_literal_number(b: &[i8]) -> (i64, usize) {
    let chunks = b.chunks(5);
    let mut bits: Vec<i8> = vec![];
    let mut bits_used = 0;
    for chunk in chunks {
        let num = &chunk[1..];
        bits.extend(num);
        bits_used += 5;
        if chunk[0] == 0 {
            break;
        }
    }

    let x3 = parse_number(&bits);
    return (x3, bits_used);
}

fn trebits_to_num(bits: &[i8]) -> i32 {
    match bits {
        [1, 1, 1] => 7,
        [1, 1, 0] => 6,
        [1, 0, 1] => 5,
        [1, 0, 0] => 4,
        [0, 1, 1] => 3,
        [0, 1, 0] => 2,
        [0, 0, 1] => 1,
        [0, 0, 0] => 0,
        _ => panic!("erhm"),
    }
}

fn hex_to_b2(i: &str) -> Vec<i8> {
    let mut b = vec![];
    for c in i.chars() {
        let n = match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("invalid"),
        };
        b.extend(n);
    }
    return b;
}

impl Packet {
    fn from_str(s: &str) -> Self {
        Self::from_bits(hex_to_b2(s))
    }
    fn from_bits(b: Vec<i8>) -> Self {
        // let b = hex_to_b2(s);
        let version = trebits_to_num(&b[0..3]);
        let type_id = trebits_to_num(&b[3..6]);
        match type_id {
            4 => {
                let (value, bits_used) = parse_literal_number(&b[6..]);
                Self::Literal {
                    version,
                    ttype: type_id,
                    value,
                    bits_used: bits_used + 6,
                }
            }
            _n => {
                let length_type = LengthType::from_bits(&b);
                let mut start = length_type.get_start();
                let mut packets = vec![];
                loop {
                    if length_type.parsed_enough(start, &packets) {
                        break;
                    }

                    let next_bits = &b[start..];
                    let packet = Self::from_bits(next_bits.into());
                    start += packet.used_bits();
                    packets.push(packet);
                }

                Self::Operator {
                    version,
                    op: Op::from_int(type_id),
                    length_type,
                    children: packets,
                }
            }
        }
    }

    fn used_bits(&self) -> usize {
        match self {
            Packet::Literal {
                version: _,
                ttype: _,
                value: _,
                bits_used,
            } => *bits_used,
            Packet::Operator {
                version: _,
                op: _,
                length_type: ltype,
                children,
            } => {
                let ltype_bits = ltype.used_bits();
                let children_bits: usize = children.iter().map(Packet::used_bits).sum();
                7 + ltype_bits + children_bits
            }
        }
    }
    fn sum_versions(&self) -> i32 {
        match self {
            Packet::Literal {
                version,
                ttype: _,
                value: _,
                bits_used: _,
            } => *version,
            Packet::Operator {
                version,
                op: _,
                length_type: _,
                children,
            } => {
                let child_versions: i32 = children.iter().map(Self::sum_versions).sum();
                *version + child_versions
            }
        }
    }
    fn value(&self) -> i64 {
        match self {
            Packet::Literal {
                version: _,
                ttype: _,
                value,
                bits_used: _,
            } => *value,
            Packet::Operator {
                version: _,
                op,
                length_type: _,
                children,
            } => match op {
                Op::Sum => children.iter().map(Self::value).sum(),
                Op::Product => children.iter().map(Self::value).product(),
                Op::Min => children.iter().map(Self::value).min().unwrap(),
                Op::Max => children.iter().map(Self::value).max().unwrap(),
                Op::GT => {
                    let one = &children[0];
                    let two = &children[1];
                    if one.value() > two.value() {
                        1
                    } else {
                        0
                    }
                }
                Op::LT => {
                    let one = &children[0];
                    let two = &children[1];
                    if one.value() < two.value() {
                        1
                    } else {
                        0
                    }
                }
                Op::Eq => {
                    let one = &children[0];
                    let two = &children[1];
                    if one.value() == two.value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

impl Op {
    fn from_int(t: i32) -> Self {
        match t {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::GT,
            6 => Self::LT,
            7 => Self::Eq,
            _ => panic!("invalid op type"),
        }
    }
}

impl LengthType {
    fn from_bits(b: &[i8]) -> Self {
        match b[6] {
            0 => {
                // 15 bits for how many more bits
                let num0 = &b[7..22];
                let num1 = parse_number(num0);
                LengthType::NumBits(num1)
            }
            1 => {
                let num0 = &b[7..18];
                let num1 = parse_number(num0);
                LengthType::NumPackets(num1)
            }
            _ => panic!("unknown length type"),
        }
    }

    fn used_bits(&self) -> usize {
        match self {
            LengthType::NumBits(_) => 15,
            LengthType::NumPackets(_) => 11,
        }
    }

    fn get_start(&self) -> usize {
        // + versioning bits
        self.used_bits() + 7
    }

    fn parsed_enough(&self, start: usize, subs: &[Packet]) -> bool {
        match self {
            LengthType::NumBits(bits) => start >= (22 + bits) as usize,
            LengthType::NumPackets(target) => subs.len() == *target as usize,
        }
    }
}

fn main() {
    let file_input = fs::read_to_string("./src/input.txt").unwrap();
    let packet = Packet::from_str(&file_input);
    println!("Part 1 : {:?}", packet.sum_versions());
    println!("Part 2 : {:?}", packet.value());
}
