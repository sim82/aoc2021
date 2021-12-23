type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/inputxx.txt";
pub fn example() -> &'static [((i64, i64), Option<Output1>, Option<Output2>)] {
    &[((4, 8), Some(739785), None)]
}

fn puzzle((mut pos1, mut pos2): (i64, i64)) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());

    let mut pos = [pos1, pos2];
    let mut score = [0, 0];

    let mut die = (0..).map(|v| (v % 100) + 1);
    let mut num_throws = 0;
    let mut last_i = 0;
    for i in 0.. {
        for _ in 0..3 {
            pos[i % 2] += die.next().unwrap();
            num_throws += 1;
        }

        pos[i % 2] = 1 + (pos[i % 2] - 1) % 10;
        score[i % 2] += pos[i % 2];

        if score[i % 2] >= 1000 {
            last_i = i;
            break;
        }
    }
    (Some(score[(last_i + 1) % 2] * num_throws), None)
}

fn main() {
    let (res1, res2) = puzzle((3, 4));
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
