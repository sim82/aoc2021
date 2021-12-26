use std::collections::HashSet;

use itertools::Itertools;
use rand::prelude::*;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input24.txt";

#[derive(Debug, Clone)]
enum Op {
    Inp(u8),
    Add(u8, u8),
    Mul(u8, u8),
    Div(u8, u8),
    Mod(u8, u8),
    Eql(u8, u8),
    AddDir(u8, i64),
    MulDir(u8, i64),
    DivDir(u8, i64),
    ModDir(u8, i64),
    EqlDir(u8, i64),
}

impl Op {
    fn opn(op: &str) -> u8 {
        match op {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!("bad op"),
        }
    }

    pub fn new(line: &str) -> Op {
        let mut s = line.split_whitespace();
        let opc = s.next().unwrap();
        let op1 = s.next().unwrap();
        let op1n = Op::opn(op1);

        if opc == "inp" {
            return Op::Inp(op1n);
        }

        let op2 = s.next().unwrap();
        if let Ok(op2) = op2.parse::<i64>() {
            match opc {
                "add" => Op::AddDir(op1n, op2),
                "mul" => Op::MulDir(op1n, op2),
                "div" => Op::DivDir(op1n, op2),
                "mod" => Op::ModDir(op1n, op2),
                "eql" => Op::EqlDir(op1n, op2),
                _ => panic!("bad opcode"),
            }
        } else {
            let op2n = Op::opn(op2);

            match opc {
                "add" => Op::Add(op1n, op2n),
                "mul" => Op::Mul(op1n, op2n),
                "div" => Op::Div(op1n, op2n),
                "mod" => Op::Mod(op1n, op2n),
                "eql" => Op::Eql(op1n, op2n),
                _ => panic!("bad opcode"),
            }
        }
    }
}

struct Alu<'a> {
    pub regs: [i64; 4],
    pub input: Vec<i64>,
    ops: &'a [Op],
}

impl<'a> Alu<'a> {
    pub fn new(ops: &'a [Op]) -> Self {
        Self {
            regs: [0; 4],
            input: vec![],
            ops,
        }
    }

    pub fn run(&mut self) {
        while self.ops.len() != 0 {
            match self.ops[0] {
                Op::Inp(r) => self.regs[r as usize] = self.input.pop().unwrap(),
                Op::Add(a, b) => self.regs[a as usize] += self.regs[b as usize],
                Op::Mul(a, b) => self.regs[a as usize] *= self.regs[b as usize],
                Op::Div(a, b) => self.regs[a as usize] /= self.regs[b as usize],
                Op::Mod(a, b) => self.regs[a as usize] %= self.regs[b as usize],
                Op::Eql(a, b) => {
                    self.regs[a as usize] = if self.regs[a as usize] == self.regs[b as usize] {
                        1
                    } else {
                        0
                    }
                }
                Op::AddDir(a, b) => self.regs[a as usize] += b,
                Op::MulDir(a, b) => self.regs[a as usize] *= b,
                Op::DivDir(a, b) => self.regs[a as usize] /= b,
                Op::ModDir(a, b) => self.regs[a as usize] %= b,
                Op::EqlDir(a, b) => {
                    self.regs[a as usize] = if self.regs[a as usize] == b { 1 } else { 0 }
                }
            }
            self.ops = &self.ops[1..];
        }
    }
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    let ops = s
        .trim()
        .lines()
        .map(|line| Op::new(line.trim()))
        .collect::<Vec<_>>();

    println!("ops: {:?}", ops);
    //    let mut input = [9; 14];
    let mut input = [7, 9, 6, 7, 9, 9, 7, 9, 8, 9, 9, 2, 5, 9];
    // 95299897997697

    // let mut up = [false; 14];
    let mut thrs = i64::MAX - 10000000;

    let mut best_z = i64::MAX - 10000000;
    let mut rng = rand::thread_rng();
    // let mut res = HashSet::new();
    // let highest = 13;
    let mut min = [1; 14];
    // let max = [9; 14];
    let mut best_input = [1; 14];
    for i in 0.. {
        let mut alu = Alu::new(&ops[..]);

        let mut new_input = input.clone();
        let pos = loop {
            let pos = rng.gen_range(0..14);
            if min[pos] != 9 {
                break pos;
            }
        };
        let mag = rng.gen_range(1..3);
        let dir = if new_input[pos] <= min[pos] {
            mag
        } else if new_input[pos] >= 9 {
            -mag
        } else if rng.gen_bool(0.5) {
            mag
        } else {
            -mag
        };

        new_input[pos] += dir;
        new_input[pos] = new_input[pos].clamp(1, 9);
        alu.input = new_input.into();
        // println!("input: {:?}", alu.input);

        alu.run();
        // println!("regs: {} -> {:?}", input, alu.regs);
        // if alu.regs[3] == 0 {
        //     println!("valid: {}", input);
        // }
        let zabs = alu.regs[3].abs();
        if zabs < thrs + 100000 {
            thrs = zabs;
            if zabs < best_z {
                println!("better {} {:?}", thrs, new_input);
                best_z = zabs;
            }

            let mut a = new_input;
            let mut b = best_input;
            a.reverse();

            if thrs == 0 && a > best_input {
                best_input = a;
                println!(
                    "valid: {:?} {}",
                    // res.len(),
                    new_input,
                    new_input.iter().rev().join("")
                );
                println!("{:?} {:?}", min, new_input);
                //let mut fill = false;
                let mut last9 = false;
                let mut fill = false;
                for (a, b) in new_input.iter().rev().zip(min.iter_mut().rev()) {
                    if fill {
                        *b = 1;
                        continue;
                    }
                    if last9 {
                        *b = *a;
                        fill = true;
                        continue;
                    }
                    if *a == 9 {
                        *b = 9;
                        last9 = true;
                        continue;
                    }
                }
                println!("min: {:?}", min);
                // min = new_input;
                // }
            }
            input = new_input;
        }
    }
    (None, None)
}

fn main() {
    let (res1, res2) = puzzle(&std::fs::read_to_string(INPUT_NAME).unwrap());
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

#[test]
fn test() {
    let ops = "inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2"
        .trim()
        .lines()
        .map(|line| Op::new(line.trim()))
        .collect::<Vec<_>>();

    let mut alu = Alu::new(&ops[..]);
    alu.input.push(0b0101);
    alu.run();
    assert_eq!(alu.regs, [0, 1, 0, 1]);

    let mut alu = Alu::new(&ops[..]);
    alu.input.push(0b1010);
    alu.run();
    assert_eq!(alu.regs, [1, 0, 1, 0]);
}
