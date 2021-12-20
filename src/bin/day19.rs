use std::collections::{HashMap, HashSet};

use aoc2021::{parser::scanner_list, Dir3, Vec3, ORIENTATIONS};
use itertools::Itertools;
use nom::branch::Permutation;

type Output1 = usize;
type Output2 = i64;

const INPUT_NAME: &str = "input/input19.txt";

#[derive(Debug)]
struct Scanner {
    pub id: i64,
    pub probes: Vec<Vec3>,
    // pub fingerprint: HashSet<i64>,
}

impl Scanner {
    pub fn new(id: i64, probes: Vec<Vec3>) -> Self {
        // let fingerprint = probes
        //     .iter()
        //     .permutations(2)
        //     .map(|p| p[0].manhattan_dist(p[1]))
        //     .collect();
        Scanner {
            id,
            probes,
            // fingerprint,
        }
    }
}

fn mean_square_dist(a: &[Vec3], b: &[Vec3], trans: Vec3) -> i64 {
    let num = (a.len() * b.len()) as i64;
    let mut msd = 0;
    for x in a.iter() {
        for y in b.iter() {
            let v = *x - (*y + trans);
            let d = v.x * v.x + v.y * v.y + v.z * v.z;
            msd += d / num;
        }
    }
    msd
}

fn optimize(a: &[Vec3], b: &[Vec3]) -> Option<Vec3> {
    let aset = a.iter().cloned().collect::<HashSet<_>>();

    for x in a.iter() {
        for y in b.iter() {
            let trans = *x - *y;
            // println!("trans: {:?}", trans);
            let bset = b.iter().map(|v| *v + trans).collect::<HashSet<_>>();
            let n = aset.intersection(&bset).count();
            if n >= 12 {
                println!("done: {:?}", trans);
                return Some(trans);
            }
        }
    }

    None
}

fn puzzle(s: &str) -> (Option<Output1>, Option<Output2>) {
    let (rest, scanners) = scanner_list(s).unwrap();
    assert!(rest.trim().is_empty());
    let scanners = scanners
        .iter()
        .cloned()
        .map(|(id, probes)| Scanner::new(id, probes))
        .collect::<Vec<_>>();
    for s in scanners.iter() {
        println!("{:?}", s);
    }

    let mut known = HashMap::new();
    known.insert(
        0,
        (0, (Dir3::XPos, Dir3::YPos, Dir3::ZPos), Vec3::new(0, 0, 0)),
    );

    // for i in 1..scanners.len() {
    //     'outer: for target_scanner in scanners.iter() {
    //         if target_scanner.id == scanners[i].id {
    //             continue;
    //         }
    //         for o in ORIENTATIONS.iter() {
    //             let src_probes = scanners[i]
    //                 .probes
    //                 .iter()
    //                 .map(|p| p.permute(o))
    //                 .collect::<Vec<_>>();

    //             let trans = optimize(&target_scanner.probes, &src_probes);
    //             if let Some(trans) = trans {
    //                 println!("{:?}", trans);
    //                 known.insert(scanners[i].id, (target_scanner.id, *o, trans));
    //                 break 'outer;
    //             }
    //         }
    //     }
    // }
    // assert!(known.len() == scanners.len());

    let mut checked = HashSet::new();
    while known.len() != scanners.len() {
        for scanner in scanners.iter() {
            if known.contains_key(&scanner.id) {
                continue;
            }

            'outer: for known_id in known.keys().cloned() {
                if checked.contains(&(scanner.id, known_id)) {
                    continue;
                }
                for o in ORIENTATIONS.iter() {
                    let probes1 = scanner
                        .probes
                        .iter()
                        .map(|p| p.permute(o))
                        .collect::<Vec<_>>();

                    let trans = optimize(&scanners[known_id as usize].probes, &probes1);
                    checked.insert((scanner.id, known_id));
                    if let Some(trans) = trans {
                        println!("{:?}", trans);
                        known.insert(scanner.id, (known_id, *o, trans));
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("known: {:?}", known);

    let mut all_probes = HashSet::<Vec3>::new();
    all_probes.extend(scanners[0].probes.iter());
    let mut scanner_pos = vec![Vec3::new(0, 0, 0)];
    for i in 1..scanners.len() {
        let mut probes = scanners[i].probes.clone();
        let (mut parent_id, mut parent_dir, mut trans) = known[&(i as i64)];
        let mut pos = Vec3::new(0, 0, 0);
        loop {
            probes.iter_mut().for_each(|v| {
                *v = v.permute(&parent_dir) + trans;
            });
            pos = pos.permute(&parent_dir) + trans;
            // println!("parent id: {}", parent_id);
            if parent_id == 0 {
                break;
            }
            let (new_parent_id, new_parent_dir, new_trans) = known[&parent_id];
            parent_id = new_parent_id;
            parent_dir = new_parent_dir;
            trans = new_trans;
        }
        all_probes.extend(probes.iter());
        scanner_pos.push(pos);
    }

    // for (id, (parent_id, dir, trans)) in known.iter() {}

    for Vec3 { x, y, z } in all_probes.iter().sorted() {
        println!("{} {} {}", x, y, z);
    }

    let max_dist = scanner_pos
        .iter()
        .permutations(2)
        .map(|v| v[0].manhattan_dist(v[1]))
        .max();
    (Some(all_probes.len()), max_dist)
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

pub fn example() -> &'static [(&'static str, Option<Output1>, Option<Output2>)] {
    &[(
        "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    ",
        Some(79),
        Some(3621),
    )]
}
