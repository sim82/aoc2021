use itertools::Itertools;
use multimap::MultiMap;

type Output1 = i64;
type Output2 = Output1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AmphipodState {
    ForeignHigh(u8),
    ForeignLow(u8, u8),
    Storage(u8),
    FinalHigh(u8),
    FinalLow(u8, u8),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    amphipods: [AmphipodState; 8],
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
        s.amphipods.iter().all(|a| {
            matches!(
                a,
                AmphipodState::FinalHigh(_) | AmphipodState::FinalLow(_, _)
            )
        })
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
                            *o == AmphipodState::FinalHigh(tr)
                                || *o == AmphipodState::ForeignHigh(tr)
                                || matches!(*o, AmphipodState::ForeignLow(tr, _))
                        })
                    {
                        let mut s = s.clone();
                        s.amphipods[i] = AmphipodState::FinalHigh(tr);
                        new_states.push((s, dist_to_storage(*storage_slot, tr) * cost));
                    }
                }
                // in final high and
                // - no foreigners in lower
                // - low-0 free
                // -> move into low-0
                AmphipodState::FinalHigh(room)
                    if !s.amphipods.iter().any(|o| {
                        let room = *room;
                        matches!(
                            *o,
                            AmphipodState::FinalLow(room, 0) | AmphipodState::ForeignLow(room, _)
                        )
                    }) =>
                {
                    let mut s = s.clone();
                    s.amphipods[i] = AmphipodState::FinalLow(*room, 0);
                    new_states.push((s, 1 * cost));
                }
                // in final high and final low-0 occupies -> do nothing
                // TODO: not sure if this is correct
                AmphipodState::FinalHigh(room)
                    if s.amphipods
                        .iter()
                        .any(|o| *o == AmphipodState::FinalLow(*room, 0)) =>
                {
                    ()
                }
                // in final low and
                // - no foreigner in lower
                // - low n+1 is free
                // -> move down
                meeeeeeeep
                AmphipodState::FinalLow(room, level) => (),
                AmphipodState::ForeignLow(room) => {
                    if !s.amphipods.iter().any(|o| {
                        *o == AmphipodState::FinalHigh(*room)
                            || *o == AmphipodState::ForeignHigh(*room)
                    }) {
                        let mut s = s.clone();
                        s.amphipods[i] = AmphipodState::ForeignHigh(*room);
                        new_states.push((s, 1 * cost));
                    }
                }
                AmphipodState::ForeignHigh(room) | AmphipodState::FinalHigh(room) => {
                    for slot in storage_occ
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| !*b)
                        .map(|(i, _)| i as u8)
                    {
                        if can_reach_room(slot, *room, &storage_occ) {
                            let mut s = s.clone();
                            s.amphipods[i] = AmphipodState::Storage(slot);
                            new_states.push((s, dist_to_storage(slot, *room) * cost));
                        }
                    }
                }
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
    //   #C#A#A#B#
    //   #########
    let state = State {
        amphipods: [
            AmphipodState::ForeignLow(1),
            AmphipodState::ForeignLow(2),
            AmphipodState::ForeignHigh(3),
            AmphipodState::ForeignLow(3),
            AmphipodState::ForeignLow(0),
            AmphipodState::ForeignHigh(1),
            AmphipodState::ForeignHigh(0),
            AmphipodState::ForeignHigh(2),
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
        /*
         #############
         #01.2.3.4.56#
         ###7#9#b#d###
           #8#a#c#e#
           #########
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
                 */
        (
            State {
                amphipods: [
                    AmphipodState::FinalLow(0),
                    AmphipodState::ForeignLow(3),
                    AmphipodState::ForeignHigh(2),
                    AmphipodState::ForeignHigh(0),
                    AmphipodState::FinalLow(2),
                    AmphipodState::ForeignHigh(1),
                    AmphipodState::ForeignLow(1),
                    AmphipodState::FinalHigh(3),
                ],
            },
            None,
            None,
        ),
        /*
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
          */
        // #############
        // #01.2.3.4.56#
        // ###7#9#b#d###
        //   #8#a#c#e#
        //   #########
        (
            State {
                amphipods: [
                    AmphipodState::FinalLow(0),
                    AmphipodState::ForeignLow(3),
                    AmphipodState::ForeignHigh(0),
                    AmphipodState::ForeignHigh(2),
                    AmphipodState::ForeignHigh(1),
                    AmphipodState::ForeignLow(2),
                    AmphipodState::ForeignLow(1),
                    AmphipodState::FinalHigh(3),
                ],
            },
            None,
            None,
        ),
        // (
        //     State {
        //         pos: [0x9, 0xa, 0x7, 0x8, 0xd, 0xc, 0xb, 0xe],
        //         mover: -1,
        //     },
        //     None,
        //     None,
        // ),
        // (
        //     State {
        //         pos: [0x8, 0xe, 0x7, 0x2, 0x9, 0xc, 0xa, 0xd],
        //         mover: -1,
        //     },
        //     None,
        //     None,
        // ),
        /*
             #############
             #01.2.3.4.56#
             ###7#9#b#d###
               #8#a#c#e#
               #########
        #############
        #...B.......#
        ###B#C#.#D###
          #A#D#C#A#
          #########
                      */
        // (
        //     State {
        //         //       a      b       c        d
        //         pos: [8, 0xe, 7, 0xb, 9, 0xc, 0xa, 0xd],
        //         mover: -1,
        //     },
        //     None,
        //     None,
        // ),
    ]
}
