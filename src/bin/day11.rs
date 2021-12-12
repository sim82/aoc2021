use std::collections::HashSet;

use aoc2021::read_i64_field;

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input11.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526",
        Some(1656),
        Some(195),
    )
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let mut field = read_i64_field(s);
    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0.. {
        for (_, e) in field.iter_mut() {
            *e += 1;
        }
        let mut has_flashed = HashSet::new();
        loop {
            let flashing: HashSet<_> = field
                .iter()
                .filter_map(|(c, e)| {
                    if *e > 9 && !has_flashed.contains(c) {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .collect();
            // println!("flashing: {:?}", flashing);
            if flashing.is_empty() {
                break;
            }
            if i < 100 {
                res1 += flashing.len();
            }
            for c in flashing.iter() {
                for n in c.neighbors() {
                    match field.entry(n) {
                        std::collections::hash_map::Entry::Occupied(mut nf) => *nf.get_mut() += 1,
                        std::collections::hash_map::Entry::Vacant(_) => (),
                    }
                }
            }
            has_flashed.extend(flashing);
        }
        if has_flashed.len() == field.len() {
            res2 = i + 1;
            break;
        }
        for c in has_flashed {
            field.insert(c, 0);
        }
    }

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
