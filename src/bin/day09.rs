use std::collections::{HashMap, HashSet};

use aoc2021::Vec2;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input09.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "2199943210
    3987894921
    9856789892
    8767896789
    9899965678",
        Some(15),
        Some(1134),
    )
}
fn char_to_int(c: char) -> i64 {
    assert!(c.is_digit(10));
    (c as u8 - b'0') as i64
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let height_field: HashMap<Vec2, i64> = s
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, c)| {
                (
                    Vec2 {
                        x: x as i64,
                        y: y as i64,
                    },
                    char_to_int(c),
                )
            })
        })
        .collect();

    let mut sum = 0;
    let mut basin_sizes = Vec::new();

    for (p, v) in height_field.iter() {
        let lowest_neighbor = p
            .ortho_neighbors()
            .iter()
            .filter_map(|n| height_field.get(n))
            .min()
            .unwrap();
        if *v < *lowest_neighbor {
            sum += *v + 1;

            let mut front = HashSet::new();

            // let mut basin_size = 1;
            front.insert(*p);
            let mut basin_points: HashSet<Vec2> = HashSet::new();
            while !front.is_empty() {
                basin_points.extend(front.iter());
                front = front
                    .iter()
                    .flat_map(|f| {
                        f.ortho_neighbors()
                            .iter()
                            .filter(|n| {
                                height_field.contains_key(n)
                                    && height_field[n] > height_field[f]
                                    && height_field[n] < 9
                                    && !basin_points.contains(n)
                            })
                            .cloned()
                            .collect::<Vec<_>>()
                    })
                    .collect();
            }
            basin_sizes.push(basin_points.len() as i64);
        }
    }

    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    let res2 = basin_sizes[0..3].iter().cloned().reduce(|acc, v| acc * v);
    (Some(sum), res2)
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
