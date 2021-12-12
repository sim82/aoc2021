use std::collections::{HashSet, VecDeque};

use aoc2021::bfs_count_paths;
use multimap::MultiMap;

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input12.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[
        (
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
            Some(10),
            Some(36),
        ),
        (
            "dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc",
            Some(19),
            Some(103),
        ),
        (
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
            Some(226),
            Some(3509),
        ),
    ]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let adj = s
        .trim()
        .lines()
        .flat_map(|s| {
            let (x, y) = s.trim().split_once('-').unwrap();
            [(x, y), (y, x)]
        })
        .collect::<MultiMap<_, _>>();

    let start = "start";
    let end = "end";

    let res1 = bfs_count_paths(
        (start, HashSet::<&str>::new()),
        |(cur, visited)| {
            adj.get_vec(cur)
                .unwrap()
                .iter()
                .cloned()
                .filter(|n| !visited.contains(n))
                .map(|n| {
                    let mut visited = visited.clone();
                    if cur.chars().all(|c| c.is_ascii_lowercase()) {
                        visited.insert(cur);
                    }
                    (n, visited)
                })
                .collect::<Vec<_>>()
        },
        |(node, _)| *node == end,
    );

    let res2 = bfs_count_paths(
        (start, true, HashSet::<&str>::new()),
        |(cur, can_reenter, visited)| {
            adj.get_vec(cur)
                .unwrap()
                .iter()
                .cloned()
                .filter(|n| (*can_reenter && *n != start) || !visited.contains(*n))
                .map(|n| {
                    let mut visited = visited.clone();
                    if cur.chars().all(|c| c.is_ascii_lowercase()) {
                        visited.insert(cur);
                    }
                    (n, *can_reenter && !visited.contains(n), visited)
                })
                .collect::<Vec<_>>()
        },
        |(node, _, _)| *node == end,
    );

    (Some(res1), Some(res2))
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
