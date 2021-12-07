use std::collections::HashMap;

use aoc2021::parser;

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input05.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    ",
        Some(5),
        Some(12),
    )
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let (_, line_segments) = parser::line_segment_list(s).unwrap();

    let mut points = HashMap::new();
    for (a, b) in line_segments.iter().filter(|(a, b)| a.y == b.y) {
        let mut xstart = a.x;
        let mut xend = b.x;
        if xstart > xend {
            std::mem::swap(&mut xstart, &mut xend);
        }
        for x in xstart..=xend {
            match points.entry((x, a.y)) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(1);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => *e.get_mut() += 1,
            }
        }
    }
    for (a, b) in line_segments.iter().filter(|(a, b)| a.x == b.x) {
        let mut ystart = a.y;
        let mut yend = b.y;
        if ystart > yend {
            std::mem::swap(&mut ystart, &mut yend);
        }
        for y in ystart..=yend {
            match points.entry((a.x, y)) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(1);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => *e.get_mut() += 1,
            }
        }
    }

    let res1 = points.values().filter(|v| **v >= 2).count();

    for (a, b) in line_segments
        .iter()
        .filter(|(a, b)| a.x != b.x && a.y != b.y)
    {
        let mut xstart = a.x;
        let xend = b.x;
        let xstep = (xend - xstart).signum();

        let mut ystart = a.y;
        let yend = b.y;
        let ystep = (yend - ystart).signum();
        let num_steps = (xend - xstart).abs() + 1;
        assert!(num_steps == (yend - ystart).abs() + 1);
        for _ in 0..num_steps {
            match points.entry((xstart, ystart)) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(1);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => *e.get_mut() += 1,
            }
            xstart += xstep;
            ystart += ystep;
        }
    }
    let res2 = points.values().filter(|v| **v >= 2).count();

    (Some(res1), Some(res2))
}

fn main() {
    let (res1, res2) = puzzle(&std::fs::read_to_string(INPUT_NAME).unwrap());
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

#[test]
fn test() {
    let (example, ref1, ref2) = example();
    let (res1, res2) = puzzle(example);
    assert_eq!(res1, ref1);
    assert_eq!(res2, ref2);
}
