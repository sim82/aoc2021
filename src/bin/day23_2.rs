use itertools::Itertools;
use multimap::MultiMap;

type Output1 = i64;
type Output2 = Output1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AmphipodState {
    Foreign(u8, u8),
    Storage(u8),
    Final(u8, u8),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    amphipods: [AmphipodState; 16],
}

fn final_room(i: usize) -> usize {
    match i {
        0 | 1 | 2 | 3 => 0,
        4 | 5 | 6 | 7 => 1,
        8 | 9 | 10 | 11 => 2,
        12 | 13 | 14 | 15 => 3,
        _ => unreachable!(),
    }
}

fn puzzle(s: State) -> (Option<Output1>, Option<Output2>) {
    println!("start:");
    // print_state(&s);

    let cost = [
        1, 1, 1, 1, 10, 10, 10, 10, 100, 100, 100, 100, 1000, 1000, 1000, 1000,
    ];
    // println!("dist: {:x?}", dist);

    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########
    let goal = |s: &State| {
        s.amphipods
            .iter()
            .all(|a| matches!(a, AmphipodState::Final(_, _)))
    };

    let successors = |s: &State| -> Vec<(State, i64)> {
        //   assert!(s.amphipods.iter().permutations(2).all(|a| a[0] != a[1]));
        for a in s.amphipods.iter().permutations(2) {
            if a[0] == a[1] {
                panic!("{:?} == {:?}", a[0], a[1]);
            }
        }
        let mut new_states = Vec::new();

        let mut storage_occ = [false; 7];
        for a in s.amphipods.iter() {
            if let AmphipodState::Storage(s) = a {
                storage_occ[*s as usize] = true
            }
        }

        for (i, a) in s.amphipods.iter().enumerate() {
            let tr = final_room(i) as u8;
            let cost = cost[i];
            match a {
                // in storage and can reach high pos in final room (that is not occupied by a foreigner)
                AmphipodState::Storage(storage_slot) => {
                    if can_reach_room(*storage_slot, tr, &storage_occ)
                        && !s.amphipods.iter().any(|o| {
                            *o == AmphipodState::Final(tr, 0)
                                || matches!(*o, AmphipodState::Foreign(t, _) if t == tr)
                        })
                    {
                        let mut s = s.clone();
                        s.amphipods[i] = AmphipodState::Final(tr, 0);
                        new_states.push((s, dist_to_storage(*storage_slot, tr) * cost));
                    }
                }
                // in final and
                // - no foreign in final
                // - final + 1 is free
                // - higher than final +3
                // ==> move down to final + 1
                AmphipodState::Final(room, level)
                    if *level < 3
                        && !s.amphipods.iter().any(|o| {
                            matches!(*o, AmphipodState::Foreign(r, _) if r == *room)
                                || *o == AmphipodState::Final(*room, level + 1)
                        }) =>
                {
                    let mut s = s.clone();
                    s.amphipods[i] = AmphipodState::Final(*room, level + 1);
                    new_states.push((s, 1 * cost));
                }

                // in final or foreign + 0 and any foreign in room
                // => move to storage
                AmphipodState::Foreign(room, pos) | AmphipodState::Final(room, pos)
                    if !s.amphipods.iter().any(|o|{ 
                        matches!(*o, AmphipodState::Foreign(r,p) | AmphipodState::Final(r,p) if r == *room && p < *pos )
                    })
                    && s.amphipods
                        .iter()
                        .any(|o| matches!(*o, AmphipodState::Foreign(r, _) if r == *room)) =>
                {
                    for slot in storage_occ
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| !*b)
                        .map(|(i, _)| i as u8)
                    {
                        if can_reach_room(slot, *room, &storage_occ) {
                            let mut s = s.clone();
                            s.amphipods[i] = AmphipodState::Storage(slot);
                            new_states.push((s, (dist_to_storage(slot, *room) + *pos as i64) * cost));
                        }
                    }
                }
                // in foreign 1 or lower and n - 1 is free
                // => move up to n - 1
                AmphipodState::Foreign(room, level) | AmphipodState::Final(room, level)
                    if *level > 0
                        && !s.amphipods.iter().any(|o| {
                            *o == AmphipodState::Foreign(*room, level - 1)
                                || *o == AmphipodState::Final(*room, level - 1)
                        })
                        && s.amphipods
                            .iter()
                            .any(|o| matches!(*o, AmphipodState::Foreign(r, _) if r == *room)) =>
                {
                    let mut s = s.clone();
                    s.amphipods[i] = AmphipodState::Foreign(*room, level - 1);
                    new_states.push((s, 1 * cost));
                }
                _ => (),
            }
        }

        new_states
    };

    let res = pathfinding::directed::dijkstra::dijkstra(&s, successors, goal);

    if let Some((states, cost)) = res {
        println!("res: {}", cost);
        for s in states {
            println!("{:?}", s);
        }
    }
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    (None, None)
}

fn dist_to_storage(slot: u8, tr: u8) -> i64 {
    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########

    let slot_pos = [0, 1, 3, 5, 7, 9, 10];
    let room_pos = [2, 4, 6, 8];

    (slot_pos[slot as usize] as i64 - room_pos[tr as usize] as i64).abs() + 1
}
#[test]
fn test_dist_to_storage() {
    assert_eq!(dist_to_storage(5, 1), 6);
    assert_eq!(dist_to_storage(5, 3), 2);
}
fn can_reach_room(storage_slot: u8, tr: u8, occ: &[bool; 7]) -> bool {
    let b = [(1, 2), (2, 3), (3, 4), (4, 5)];

    let b = b[tr as usize];
    if storage_slot == b.0 || storage_slot == b.1 {
        true
    } else if storage_slot < b.0 {
        !occ[(storage_slot + 1) as usize..=b.0 as usize]
            .iter()
            .any(|b| *b)
    } else {
        !occ[b.1 as usize..storage_slot as usize].iter().any(|b| *b)
    }
}

#[test]
fn test_can_reach_room() {
    assert!(can_reach_room(
        0,
        1,
        &[true, false, false, true, true, true, false]
    ));

    assert!(!can_reach_room(
        0,
        2,
        &[true, false, false, true, true, true, false]
    ));

    assert!(!can_reach_room(
        5,
        0,
        &[true, false, false, true, true, true, false]
    ));
    assert!(can_reach_room(
        5,
        1,
        &[true, true, true, false, false, true, false]
    ));
}

// #############
// #...........#
// ###D#C#D#B###
//   #C#A#A#B#
//   #########

// #############
// #01.2.3.4.56#
// ###7#9#b#d###
//   #8#a#c#e#
//   #########

fn main() {
    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########
    // #############
    // #...........#
    // ###D#C#D#B###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#A#A#B#
    //   #########

    let state = State {
        amphipods: [
            // A
            AmphipodState::Foreign(1, 3),
            AmphipodState::Foreign(2, 2),
            AmphipodState::Foreign(2, 3),
            AmphipodState::Foreign(3, 1),
            // B
            AmphipodState::Final(1, 2),
            AmphipodState::Foreign(2, 1),
            AmphipodState::Foreign(3, 0),
            AmphipodState::Foreign(3, 3),
            // C
            AmphipodState::Foreign(0, 3),
            AmphipodState::Foreign(1, 0),
            AmphipodState::Foreign(1, 1),
            AmphipodState::Foreign(3, 2),
            // D
            AmphipodState::Foreign(0, 0),
            AmphipodState::Foreign(0, 1),
            AmphipodState::Foreign(0, 2),
            AmphipodState::Foreign(2, 0),
        ],
    };

    let statex =  State {
        amphipods: [
            // A
            AmphipodState::Final(0, 3),
            AmphipodState::Foreign(2, 2),
            AmphipodState::Foreign(3, 3),
            AmphipodState::Foreign(3, 1),
            // B
            AmphipodState::Foreign(0, 0),
            AmphipodState::Final(1, 2),
            AmphipodState::Foreign(2, 0),
            AmphipodState::Foreign(2, 1),
            // C
            AmphipodState::Foreign(1, 0),
            AmphipodState::Foreign(1, 1),
            AmphipodState::Final(2, 3),
            AmphipodState::Foreign(3, 2),
            // D
            AmphipodState::Foreign(0, 1),
            AmphipodState::Foreign(0, 2),
            AmphipodState::Foreign(1, 3),
            AmphipodState::Final(3, 0),
        ],
    };
    let (res1, res2) = puzzle(state);
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
const INPUT_NAME: &str = "input/inputxx.txt";
pub fn example() -> &'static [(State, Option<Output1>, Option<Output2>)] {
    &[
        // #############
        // #AA.....B.BD#
        // ###B#.#.#.###
        //   #D#.#C#.#
        //   #D#B#C#C#
        //   #A#D#C#A#
        //   #########
        // #############
        // #AA.....B.BD#
        // ###B#.#.#.###
        //   #D#C#.#.#
        //   #D#B#C#C#
        //   #A#D#C#A#
        //   #########
        // #############
        // #AA.....B.BD#
        // ###B#C#.#.###
        //   #D#C#.#.#
        //   #D#B#.#C#
        //   #A#D#C#A#
        //   #########
        // #############
        // #A......B.BD#
        // ###B#C#.#.###
        //   #D#C#.#.#
        //   #D#B#A#C#
        //   #A#D#C#A#
        //   #########
        // #############
        // #A........BD#
        // ###B#C#.#.###
        //   #D#C#B#.#
        //   #D#B#A#C#
        //   #A#D#C#A#
        //   #########
        // #############
        // #A.........D#
        // ###B#C#B#.###
        //   #D#C#B#.#
        //   #D#B#A#C#
        //   #A#D#C#A#
        //   #########
        (
            State {
                amphipods: [
                    // A
                    AmphipodState::Storage(0),
                    AmphipodState::Foreign(2, 2),
                    AmphipodState::Foreign(3, 3),
                    AmphipodState::Final(0, 3),
                    // B
                    AmphipodState::Foreign(0, 0),
                    AmphipodState::Final(1, 2),
                    AmphipodState::Foreign(2, 1),
                    AmphipodState::Storage(5),
                    // C
                    AmphipodState::Foreign(3, 2),
                    AmphipodState::Foreign(1, 1),
                    AmphipodState::Foreign(1, 0),
                    AmphipodState::Final(2, 3),
                    // D
                    AmphipodState::Storage(6),
                    AmphipodState::Foreign(0, 1),
                    AmphipodState::Foreign(0, 2),
                    AmphipodState::Foreign(1, 3),
                ],
            },
            None,
            None,
        ),
        /*
         #############
         #01.2.3.4.56#
         ###7#9#b#d###
           #8#a#c#e#
           #########


        #############
        #...........#
        ###B#C#B#D###
          #D#C#B#A#
          #D#B#A#C#
          #A#D#C#A#
          #########
                 */
        (
            State {
                amphipods: [
                    // A
                    AmphipodState::Final(0, 3),
                    AmphipodState::Foreign(2, 2),
                    AmphipodState::Foreign(3, 3),
                    AmphipodState::Foreign(3, 1),
                    // B
                    AmphipodState::Foreign(0, 0),
                    AmphipodState::Final(1, 2),
                    AmphipodState::Foreign(2, 0),
                    AmphipodState::Foreign(2, 1),
                    // C
                    AmphipodState::Foreign(1, 0),
                    AmphipodState::Foreign(1, 1),
                    AmphipodState::Final(2, 3),
                    AmphipodState::Foreign(3, 2),
                    // D
                    AmphipodState::Foreign(0, 1),
                    AmphipodState::Foreign(0, 2),
                    AmphipodState::Foreign(1, 3),
                    AmphipodState::Final(3, 0),
                ],
            },
            None,
            None,
        ),
    ]
}
