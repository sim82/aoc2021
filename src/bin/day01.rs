fn main() {
    let s = std::fs::read_to_string("input/input01.txt").unwrap();
    let input = s
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let window_sums: Vec<_> = input.windows(3).map(|w| w.iter().sum::<i64>()).collect();
    let res = window_sums.windows(2).filter(|w| w[1] > w[0]).count();

    println!("res2: {}", res);
}
