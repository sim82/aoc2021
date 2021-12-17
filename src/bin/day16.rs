use itertools::Itertools;

type Output1 = u64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input16.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[
        ("8A004A801A8002F478", Some(16), Some(15)),
        ("C0015000016115A2E0802F182340", Some(23), Some(46)),
        ("A0016C880162017C3686B18A3D4780", Some(31), Some(54)),
        ("C200B40A82", Some(14), Some(3)),
        ("04005AC33890", Some(8), Some(54)),
        ("880086C3E88112", Some(15), Some(7)),
        ("CE00C43D881120", Some(11), Some(9)),
        ("D8005AC2A8F0", Some(13), Some(1)),
        ("F600BC2D8F", Some(19), Some(0)),
        ("9C005AC2F8F0", Some(16), Some(0)),
        ("9C0141080250320F1802104A08", Some(20), Some(1)),
    ]
}

fn read_packet(r: &mut NibbleReader<'_>) -> (u64, u64) {
    let mut version = r.read_int(3);
    let id = r.read_int(3);

    let v = match id {
        4 => r.read_literal(),
        _ => {
            let length_type = r.read_int(1);
            let mut vs = Vec::new();
            if length_type == 0 {
                let length = r.read_int(15) as usize;
                let start = r.pos();
                while r.pos() != start + length {
                    let (pv, v) = read_packet(r);
                    version += pv;
                    vs.push(v);
                }
            } else {
                let num = r.read_int(11);
                for _ in 0..num {
                    let (pv, v) = read_packet(r);
                    version += pv;
                    vs.push(v);
                }
            }

            match id {
                0 => vs.iter().sum::<u64>(),
                1 => vs.iter().cloned().reduce(|a, b| a * b).unwrap(),
                2 => vs.iter().cloned().min().unwrap(),
                3 => vs.iter().cloned().max().unwrap(),
                5 => {
                    assert!(vs.len() == 2);
                    if vs[0] > vs[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    assert!(vs.len() == 2);
                    if vs[0] < vs[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    assert!(vs.len() == 2);
                    if vs[0] == vs[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("bad operator {}", id),
            }
        }
    };
    (version, v)
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let nibbles = s.chars().collect::<Vec<_>>();
    let mut r = NibbleReader::new(&nibbles);

    let (version_sum, v) = read_packet(&mut r);
    (Some(version_sum), Some(v))
}

struct NibbleReader<'a> {
    nibbles: &'a [char],
    // ptr: usize,
    nibble: u8,
    mask: u8,
    pos: usize,
}

impl<'a> NibbleReader<'a> {
    pub fn new(input: &'a [char]) -> Self {
        Self {
            nibbles: input,
            nibble: 0,
            mask: 0,
            pos: 0,
        }
    }

    pub fn next_bit(&mut self) -> bool {
        if self.mask == 0 {
            self.nibble = u8::from_str_radix(&self.nibbles[0].to_string(), 16).unwrap();
            self.nibbles = &self.nibbles[1..];
            self.mask = 0b1000;
        }
        assert!(self.mask != 0);
        let res = (self.nibble & self.mask) != 0;
        self.mask >>= 1;
        self.pos += 1;
        res
    }
    pub fn read_int(&mut self, bits: usize) -> u64 {
        let mut ret = 0;
        for _ in 0..bits {
            ret <<= 1;
            if self.next_bit() {
                ret |= 0b1;
            }
        }
        ret
    }

    pub fn read_literal(&mut self) -> u64 {
        let mut ret = 0;
        loop {
            let v = self.read_int(5);
            ret <<= 4;
            ret |= v & 0b1111;
            if (v & 0b10000) == 0 {
                break;
            }
        }
        ret
    }
    pub fn drop_nibble(&mut self) {
        self.mask = 0;
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
}

fn main() {
    let (res1, res2) = puzzle(&std::fs::read_to_string(INPUT_NAME).unwrap());
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

#[test]
fn test() {
    for (example, ref1, ref2) in example().iter().cloned() {
        let (res1, res2) = puzzle(example);
        assert_eq!(res1, ref1);
        assert_eq!(res2, ref2);
    }
}

#[test]
fn test_bits() {
    let nibbles = "1E".chars().collect::<Vec<_>>();
    let mut r = NibbleReader::new(&nibbles);
    for _ in 0..3 {
        assert!(!r.next_bit());
    }
    assert!(r.next_bit());
    for _ in 0..3 {
        assert!(r.next_bit());
    }
    assert!(!r.next_bit());

    let mut r = NibbleReader::new(&nibbles);
    assert_eq!(r.read_int(3), 0b000);
    assert_eq!(r.read_int(3), 0b111);
    assert_eq!(r.read_int(2), 0b10);

    let nibbles = "D2FE28".chars().collect::<Vec<_>>();
    let mut r = NibbleReader::new(&nibbles);

    let version = r.read_int(3);
    let id = r.read_int(3);
    let lit = r.read_literal();
    r.drop_nibble();

    assert_eq!(version, 6);
    assert_eq!(id, 4);
    assert_eq!(lit, 2021);
}
