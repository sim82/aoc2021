use std::collections::HashMap;

use aoc2021::count_occurrences;
use itertools::Itertools;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input14.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C",
        Some(1588),
        Some(2188189693529),
    )]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    //let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    let mut lines = s.trim().lines();
    let template = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .chain(['@']) // ultra cheap: play the annoying 'extend alphabet' card...
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let rules = lines
        .map(|line| {
            let (a, b) = line.split_once("->").unwrap();
            let mut a = a.trim().chars();
            (
                [a.next().unwrap(), a.next().unwrap()],
                b.trim().chars().next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut res1 = None;
    let mut res2 = None;
    let mut dimer_count = count_occurrences(template.windows(2).map(|w| [w[0], w[1]]));
    for round in 0..40 {
        let insertions = dimer_count
            .iter()
            .filter_map(|(dimer, count)| {
                rules.get(dimer).map(|insertion| {
                    let count = *count as i64;
                    [
                        // insert X into ab -> + aX, + Xb and - ab
                        (count, [dimer[0], *insertion]),
                        (count, [*insertion, dimer[1]]),
                        (-count, *dimer),
                    ]
                })
            })
            .flatten()
            .collect::<Vec<_>>();
        for (count, dimer) in insertions {
            match dimer_count.entry(dimer) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    *e.get_mut() += count;
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(count);
                }
            }
        }

        if round == 9 || round == 39 {
            let mut monomer_count = HashMap::new();
            for (dimer, count) in dimer_count.iter() {
                match monomer_count.entry(dimer[0]) {
                    std::collections::hash_map::Entry::Occupied(mut e) => *e.get_mut() += count,
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(*count);
                    }
                }
            }
            let (min, max) = monomer_count.values().minmax().into_option().unwrap();
            if round == 9 {
                res1 = Some(max - min);
            } else {
                res2 = Some(max - min);
            }
        }
    }

    // println!("minmax: {} {}", min, max);
    (res1, res2)
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
