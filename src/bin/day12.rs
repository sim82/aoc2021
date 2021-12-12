use std::collections::{HashSet, VecDeque};

use multimap::MultiMap;

type Output1 = i64;
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
    let input = s
        .trim()
        .lines()
        .map(|s| {
            let mut a = s.trim().split('-');
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect::<Vec<_>>();
    println!("{:?}", input);
    let mut adj = MultiMap::new();
    for (a, b) in input.iter() {
        // adj.insert(a.to_string(), b.to_string());
        // adj.insert(b.to_string(), a.to_string());

        adj.insert(a.to_string(), b.to_string());
        adj.insert(b.to_string(), a.to_string());
    }

    let start = "start";
    let end = "end";
    let mut queue = VecDeque::new();
    queue.push_back((start, HashSet::<&str>::new()));
    let mut res1 = 0;
    while !queue.is_empty() {
        let (next, visited) = queue.pop_front().unwrap();

        if next == end {
            res1 += 1;
            continue;
        }
        for n in adj
            .get_vec(next)
            .unwrap()
            .iter()
            .filter(|n| !visited.contains(n.as_str()))
        {
            let mut visited = visited.clone();
            if next.chars().all(|c| c.is_ascii_lowercase()) {
                visited.insert(next);
            }
            queue.push_back((n, visited));
        }
    }

    let start = "start";
    let end = "end";
    let mut queue = VecDeque::new();
    queue.push_back((vec![], start, HashSet::<&str>::new(), ""));
    let mut res2 = 0;
    while !queue.is_empty() {
        let (path, cur, visited, boost) = queue.pop_front().unwrap();
        if cur == end {
            res2 += 1;
            println!("done: {:?} {}", path, boost);
            continue;
        }
        for n in adj
            .get_vec(cur)
            .unwrap()
            .iter()
            .filter(|n| !visited.contains(n.as_str()) || (boost.is_empty() && *n != start))
        {
            let mut visited = visited.clone();
            if cur.chars().all(|c| c.is_ascii_lowercase()) {
                visited.insert(cur);
            }
            let mut path = path.clone();
            path.push(cur);
            let mut boost = boost;
            if visited.contains(n.as_str()) {
                boost = n;
            }
            queue.push_back((path, n, visited, boost));
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
    // let (example, ref1, ref2) = example();
    for (example, ref1, ref2) in example().iter().cloned() {
        // println!("example: {}", example);
        let (res1, res2) = puzzle(example);
        assert_eq!(res1, ref1);
        assert_eq!(res2, ref2);
    }
}
