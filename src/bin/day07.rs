type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input07.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    ("16,1,2,0,4,2,7,1,2,14", Some(37), Some(168))
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let input = s
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let max_column = *input.iter().max().unwrap();
    let res1 = (0..=max_column)
        .map(|i| input.iter().map(|c| (*c - i).abs()).sum::<i64>())
        .min();

    let res2 = (0..=max_column)
        .map(|i| {
            input
                .iter()
                .map(|c| {
                    let n = (*c - i).abs();
                    (n * (n + 1)) / 2
                })
                .sum::<i64>()
        })
        .min();

    (res1, res2)
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
