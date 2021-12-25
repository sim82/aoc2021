use std::collections::HashSet;

use aoc2021::{bool_field_bounds, Vec2};

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input25.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>",
        Some(58),
        None,
    )]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let mut h = HashSet::<Vec2>::new();
    let mut v = HashSet::<Vec2>::new();

    for (y, line) in s.trim().lines().map(|s| s.trim()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;
            if c == 'v' {
                v.insert(Vec2::new(x, y));
            } else if c == '>' {
                h.insert(Vec2::new(x, y));
            }
        }
    }
    let (_, hmax) = bool_field_bounds(&h);
    let (_, vmax) = bool_field_bounds(&v);
    let width = hmax.x.max(vmax.x) + 1;
    let height = hmax.y.max(vmax.y) + 1;
    let mut steps = None;
    for i in 1.. {
        let u = h.union(&v).collect::<HashSet<_>>();

        let mut newh = HashSet::new();
        for h in h.iter() {
            let n = Vec2 {
                x: (h.x + 1) % width,
                y: h.y,
            };
            newh.insert(if !u.contains(&n) { n } else { *h });
        }
        let u = newh.union(&v).collect::<HashSet<_>>();
        let mut newv = HashSet::new();

        for v in v.iter() {
            let n = Vec2 {
                x: v.x,
                y: (v.y + 1) % height,
            };
            newv.insert(if !u.contains(&n) { n } else { *v });
        }
        if h == newh && v == newv {
            steps = Some(i);
            break;
        }

        h = newh;
        v = newv;
    }

    (steps, None)
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
