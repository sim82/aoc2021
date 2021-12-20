use std::collections::{HashMap, HashSet, VecDeque};

use aoc2021::{bool_field_bounds, dump_bool_field, Vec2};

type Output1 = usize;
type Output2 = Output1;

const INPUT_NAME: &str = "input/input20.txt";
pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###", Some(35), Some(3351))]
}

fn check_pixel(
    pixel: &Vec2,
    base_image: &HashSet<Vec2>,
    level: usize,
    filter: &[bool],
    cache: &mut HashMap<(Vec2, usize), bool>,
) -> bool {
    if level == 0 {
        base_image.contains(pixel)
    } else {
        if let Some(e) = cache.get(&(*pixel, level)) {
            return *e;
        }

        let mut filter_index = 0b0;
        for n in pixel.self_and_neighbors() {
            filter_index <<= 1;
            if check_pixel(&n, base_image, level - 1, filter, cache) {
                filter_index |= 0b1;
            }
        }
        let r = filter[filter_index];
        cache.insert((*pixel, level), r);
        r
    }
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    let mut lines = s.trim().lines().map(|l| l.trim()).collect::<VecDeque<_>>();
    let filter = lines[0]
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("bad char in filter"),
        })
        .collect::<Vec<_>>();

    println!("filter: {:?}", filter);

    let mut y = 0;
    lines.pop_front();
    lines.pop_front();

    let mut image = HashSet::<Vec2>::new();
    while let Some(line) = lines.pop_front() {
        if line.is_empty() {
            continue;
        }
        image.extend(line.chars().enumerate().filter_map(|(x, c)| match c {
            '#' => Some(Vec2 { x: x as i64, y }),
            '.' => None,
            _ => panic!("bad char in image"),
        }));
        y += 1;
    }

    let (min, max) = bool_field_bounds(&image);
    let mut out_image = HashSet::<Vec2>::new();
    let mut cache = HashMap::new();
    for y in min.y - 10..max.y + 10 {
        for x in min.x - 10..max.x + 10 {
            let pixel = Vec2 { x, y };
            if check_pixel(&pixel, &image, 2, &filter, &mut cache) {
                out_image.insert(pixel);
            }
        }
    }
    dump_bool_field(&out_image);

    let (min, max) = bool_field_bounds(&image);
    let mut out_image2 = HashSet::<Vec2>::new();
    let mut cache = HashMap::new();
    let level = 50;
    let inc_bounds = level + 2;
    for y in min.y - inc_bounds..max.y + inc_bounds {
        for x in min.x - inc_bounds..max.x + inc_bounds {
            let pixel = Vec2 { x, y };
            if check_pixel(&pixel, &image, 50, &filter, &mut cache) {
                out_image2.insert(pixel);
            }
        }
    }
    dump_bool_field(&out_image2);
    (Some(out_image.len()), Some(out_image2.len()))
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
