use std::collections::HashSet;

use aoc2021::{
    dump_bool_field,
    parser::{coords_and_fold, FoldInstruction},
    Vec2,
};
use itertools::Itertools;

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input13.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5",
        Some(17),
        Some(16),
    )]
}

fn apply_fold(field: &HashSet<Vec2>, fold: &FoldInstruction) -> HashSet<Vec2> {
    match fold {
        FoldInstruction::X(edge) => {
            let field1 = field.iter().filter(|c| c.x < *edge).cloned();
            let field2 = field.iter().filter(|c| c.x > *edge).map(|c| Vec2 {
                x: edge - (c.x - edge),
                y: c.y,
            });
            field1.chain(field2).collect()
        }
        FoldInstruction::Y(edge) => {
            let field1 = field.iter().filter(|c| c.y < *edge).cloned();
            let field2 = field.iter().filter(|c| c.y > *edge).map(|c| Vec2 {
                x: c.x,
                y: edge - (c.y - edge),
            });
            field1.chain(field2).collect()
        }
    }
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let (_, (coords, folds)) = coords_and_fold(s).unwrap();
    let field = coords.iter().cloned().collect::<HashSet<_>>();

    let field1 = apply_fold(&field, &folds[0]);

    let field2 = folds.iter().fold(field, |acc, fold| apply_fold(&acc, fold));
    dump_bool_field(&field2);
    (Some(field1.len()), Some(field2.len()))
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
