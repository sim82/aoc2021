type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/inputxx.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[("", None, None)]
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    (None, None)
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
