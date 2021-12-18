use aoc2021::{parser::snailfish_number, SfNumber};
use itertools::Itertools;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input18.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        Some(4140),
        Some(3993),
    )]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());

    let mut numbers = s
        .trim()
        .lines()
        .map(|line| snailfish_number(line.trim()).unwrap().1)
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .cloned()
        .reduce(|a, b| {
            let mut a = SfNumber::Pair(Box::new(a), Box::new(b));
            a.reduce();
            a
        })
        .unwrap();
    println!("{:?}", sum);

    let max_pair = numbers
        .iter()
        .permutations(2)
        .map(|x| {
            let mut a = SfNumber::Pair(Box::new(x[0].clone()), Box::new(x[1].clone()));
            a.reduce();
            a.get_magnitude()
        })
        .max();
    (Some(sum.get_magnitude()), max_pair)
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
