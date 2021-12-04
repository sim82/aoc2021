type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/inputxx.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    ("", None, None)
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    (None, None)
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
