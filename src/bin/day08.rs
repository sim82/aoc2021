use std::collections::HashSet;

use aoc2021::{lowercase_char_to_index, parser};
use itertools::Itertools;

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input08.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
    fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
    cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
    efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
    gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
    gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
    cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
    ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
    gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
    fgae cfgab fg bagce
    ",
        Some(26),
        Some(61229),
    )
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let input = parser::seven_segment_sample_list(s).unwrap().1;
    println!("{:?}", input);
    let res1 = input
        .iter()
        .flat_map(|(_, b)| b.iter())
        .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
        .count();

    // //a0, b1, c2, d3, e4, f5, g6
    // let len_to_mapping = HashMap::new();
    // len_to_mapping[2] = [2, 5]; // 1 -> cf
    // len_to_mapping[3] = [0, 2, 5]; // 7 -> acf
    // len_to_mapping[4] = [1, 2, 3, 5]; // 4 -> bcdf
    // len_to_mapping[5] = [0, 1, 2, 3, 4, 5, 6]; // 2, 3, 5 -> abcdefg

    // 0: abcefg
    // 1: cf
    // 2: acdeg
    // 3: acdfg
    // 4: bcdf
    // 5: abdfg
    // 6: abdefg
    // 7: acf
    // 8: abcdefg
    // 9: abcdfg
    let reference = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let reference_set = reference
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    let permutations = ('a'..='g')
        .permutations(7)
        .map(|perm| perm.to_vec())
        .collect::<Vec<_>>();
    let mut res2 = 0;
    for (a, b) in input.iter() {
        let mut num_perms = 0;
        let mut perm = Vec::new();
        for p in permutations.iter() {
            let perm_set = a
                .iter()
                .map(|s| apply_permutation(s, p))
                .collect::<HashSet<_>>();

            // println!("{:?} {:?}", perm_set, reference_set);
            if perm_set == reference_set {
                num_perms += 1;
                perm = p.clone();
            }
        }
        println!("num perms: {} {:?}", num_perms, perm);
        let mut v = 0;
        for d in b.iter() {
            let o = apply_permutation(d, &perm);
            let digit = reference.iter().find_position(|x| **x == o).unwrap().0;
            v *= 10;
            v += digit;
            //            println!("{}", digit);
        }
        println!("{}", v);
        res2 += v;
    }

    (Some(res1), Some(res2))
}

fn apply_permutation(i: &str, permutation: &Vec<char>) -> String {
    i.chars()
        .map(|c| {
            let i = lowercase_char_to_index(c);
            permutation[i]
        })
        .sorted()
        .collect()
}

fn main() {
    let (res1, res2) = puzzle(&std::fs::read_to_string(INPUT_NAME).unwrap());
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

#[test]
fn test() {
    assert_eq!(
        apply_permutation("acf", &"deafgbc".chars().collect()),
        "abd"
    );

    let (example, ref1, ref2) = example();
    let (res1, res2) = puzzle(example);
    assert_eq!(res1, ref1);
    assert_eq!(res2, ref2);
}
