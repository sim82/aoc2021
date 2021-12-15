use std::fs::read_dir;

use aoc2021::{i64_field_bounds, read_i64_field, Vec2};

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input15.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581",
        Some(40),
        Some(315),
    )]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let field = read_i64_field(s);

    let (start, end) = i64_field_bounds(&field);

    println!("{:?} {:?}", start, end);
    let dijkstra_res = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |f| {
            f.ortho_neighbors()
                .iter()
                .filter_map(|n| field.get(n).map(|score| (*n, *score)))
                .collect::<Vec<_>>()
        },
        |f| *f == end,
    );

    let width = end.x - start.x + 1;
    let height = end.y - start.y + 1;
    let end2 = Vec2 {
        x: (end.x + 1) * 5 - 1,
        y: (end.y + 1) * 5 - 1,
    };

    let dijkstra_res2 = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |f| {
            f.ortho_neighbors()
                .iter()
                .filter_map(|n| {
                    if n.x < start.x || n.y < start.y || n.x > end2.x || n.y > end2.y {
                        return None;
                    }
                    let nmod = Vec2 {
                        x: (n.x % width),
                        y: (n.y % height),
                    };
                    field.get(&nmod).map(|score| {
                        let inc = n.x / width + n.y / height; // 'increasing wave-front'
                        (*n, (*score + inc - 1) % 9 + 1)
                    })
                })
                .collect::<Vec<_>>()
        },
        |f| *f == end2,
    )
    .unwrap();

    // println!("{:?}", dijkstra_res2.0);
    (dijkstra_res.map(|(_, score)| score), Some(dijkstra_res2.1))
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
