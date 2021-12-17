use aoc2021::Vec2;

type Output1 = i64;
type Output2 = Output1;

const INPUT_NAME: &str = "input/inputxx.txt";
pub fn example() -> &'static [((i64, i64, i64, i64), Option<Output1>, Option<Output2>)] {
    &[((20, 30, -10, -5), Some(45), Some(112))]
}

fn puzzle(minx: i64, maxx: i64, miny: i64, maxy: i64) -> (Option<Output1>, Option<Output2>) {
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());

    let mut uppery = 0;
    let mut num_success = 0;
    for velx in 0..400 {
        for mut vely in -400..400 {
            let velxi = velx;
            let velyi = vely;
            let mut velx = velx;

            let mut pos = Vec2::default();
            // println!("{} {}", velx, vely);
            let mut success = false;
            let mut local_uppery = 0;
            loop {
                pos.x += velx;
                pos.y += vely;
                // println!("pos: {:?}", pos);
                if pos.x > maxx || pos.y < miny {
                    // println!("failed");
                    break;
                }

                if pos.y > local_uppery {
                    local_uppery = pos.y;
                }
                if velx > 0 {
                    velx -= 1;
                } else if velx < 0 {
                    velx += 1;
                }
                vely -= 1;

                if pos.x >= minx && pos.x <= maxx && pos.y >= miny && pos.y <= maxy {
                    success = true;
                    println!("success: {} {:?} {} {}", local_uppery, pos, velxi, velyi);
                    break;
                }
            }
            if success {
                uppery = uppery.max(local_uppery);
                num_success += 1;
            }
        }
    }

    (Some(uppery), Some(num_success))
}

fn main() {
    let (res1, res2) = puzzle(230, 283, -107, -57);
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

#[test]
fn test() {
    for ((minx, maxx, miny, maxy), ref1, ref2) in example().iter().cloned() {
        let (res1, res2) = puzzle(minx, maxx, miny, maxy);
        assert_eq!(res1, ref1);
        assert_eq!(res2, ref2);
    }
}

/*/
...............#..#............
...........#........#..........
...............................
......#..............#.........
...............................
...............................
S....................#.........
...............................
...............................
...............................
.....................#.........
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................TTTTTTTTTTT
....................T#TTTTTTTTT
....................TTTTTTTTTTT
 */
