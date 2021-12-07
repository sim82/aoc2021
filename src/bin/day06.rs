type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input06.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    ("3,4,3,1,2", Some(5934), Some(26984457539))
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    let init_day_count = input.fold([0; 9], |mut acc, n| {
        acc[n as usize] += 1;
        acc
    });

    let mut day_count = init_day_count;
    println!("{:?}", day_count);
    let mut res1 = 0;
    for i in 0..256 {
        day_count.rotate_left(1);
        day_count[6] += day_count[8];
        if i == 79 {
            res1 = day_count.iter().sum::<_>();
        }
    }

    let res2 = day_count.iter().sum::<_>();
    (Some(res1), Some(res2))
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
