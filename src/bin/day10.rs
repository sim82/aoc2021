type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input10.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]",
        Some(26397),
        Some(288957),
    )
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '<' || c == '{'
}

fn check(open: char, close: char) -> Option<i64> {
    match close {
        ')' if open != '(' => Some(3),
        ']' if open != '[' => Some(57),
        '}' if open != '{' => Some(1197),
        '>' if open != '<' => Some(25137),
        _ => None,
    }
}
fn close_score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("bad char"),
    }
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let mut res1 = 0;
    let mut close_scores = Vec::new();
    'outer: for line in s.lines().map(|l| l.trim()) {
        let mut stack = Vec::new();
        for c in line.chars() {
            if is_opening(c) {
                stack.push(c);
            } else {
                let top = stack.pop().unwrap();
                if let Some(illegal_score) = check(top, c) {
                    println!("error: {}", illegal_score);
                    res1 += illegal_score;
                    continue 'outer;
                }
            }
        }

        let mut sum = 0;
        while !stack.is_empty() {
            let s = close_score(stack.pop().unwrap());
            sum *= 5;
            sum += s;
        }
        close_scores.push(sum);
    }
    close_scores.sort_unstable();

    (Some(res1), Some(close_scores[close_scores.len() / 2]))
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
