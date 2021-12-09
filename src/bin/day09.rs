use std::collections::HashSet;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input09.txt";
pub fn example() -> (&'static str, Option<Output1>, Option<Output2>) {
    (
        "2199943210
    3987894921
    9856789892
    8767896789
    9899965678",
        Some(15),
        Some(1134),
    )
}
fn char_to_int(c: char) -> i64 {
    assert!(c.is_digit(10));
    (c as u8 - b'0') as i64
}
fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let input = s
        .trim()
        .lines()
        .map(|s| s.trim().chars().map(|c| char_to_int(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = input.len();
    let width = input[0].len();

    let mut sum = 0;

    let mut basin_sizes = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut lowest_neighbor = i64::MAX;

            if y > 0 {
                lowest_neighbor = lowest_neighbor.min(input[y - 1][x]);
            }
            if y < height - 1 {
                lowest_neighbor = lowest_neighbor.min(input[y + 1][x]);
            }
            if x > 0 {
                lowest_neighbor = lowest_neighbor.min(input[y][x - 1]);
            }
            if x < width - 1 {
                lowest_neighbor = lowest_neighbor.min(input[y][x + 1]);
            }

            if input[y][x] < lowest_neighbor {
                sum += input[y][x] + 1;
                let mut front = HashSet::new();

                // let mut basin_size = 1;
                front.insert((x, y));
                let mut basin_points: HashSet<(usize, usize)> = HashSet::new();
                while !front.is_empty() {
                    // println!("front: {:?}", front);
                    let mut new_front = HashSet::new();
                    basin_points.extend(front.iter());
                    for (x, y) in front {
                        if y > 0
                            && input[y - 1][x] > input[y][x]
                            && input[y - 1][x] != 9
                            && !basin_points.contains(&(x, y - 1))
                        {
                            new_front.insert((x, y - 1));
                        }
                        if y < height - 1
                            && input[y + 1][x] > input[y][x]
                            && input[y + 1][x] != 9
                            && !basin_points.contains(&(x, y + 1))
                        {
                            new_front.insert((x, y + 1));
                        }
                        if x > 0
                            && input[y][x - 1] > input[y][x]
                            && input[y][x - 1] != 9
                            && !basin_points.contains(&(x - 1, y))
                        {
                            new_front.insert((x - 1, y));
                        }
                        if x < width - 1
                            && input[y][x + 1] > input[y][x]
                            && input[y][x + 1] != 9
                            && !basin_points.contains(&(x + 1, y))
                        {
                            new_front.insert((x + 1, y));
                        }
                    }
                    front = new_front;
                }
                println!("basin: {} {} {}", x, y, basin_points.len());
                basin_sizes.push(basin_points.len() as i64);
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    let res2 = basin_sizes[0..3].iter().cloned().reduce(|acc, v| acc * v);
    (Some(sum), res2)
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
