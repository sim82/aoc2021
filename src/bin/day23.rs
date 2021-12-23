use multimap::MultiMap;

type Output1 = i64;
type Output2 = Output1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    // a1: u8,
    // a2: u8,
    // b1: u8,
    // b2: u8,
    // c1: u8,
    // c2: u8,
    // d1: u8,
    // d2: u8,
    pos: [u8; 8],
    mover: i8,
}

fn print_state(s: &State) {
    let mut field = "#############
#01.2.3.4.56#
###7#9#b#d###
  #8#a#c#e#
  #########"
        .to_string();

    let names = ["A", "A", "B", "B", "C", "C", "D", "D"];
    for (name, pos) in names.iter().zip(s.pos.iter()) {
        let rep = format!("{:x}", pos);
        field = field.replace(&rep, name);
    }
    for c in [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e",
    ] {
        field = field.replace(c, ".");
    }
    println!("{}", field);
}

fn puzzle(s: State) -> (Option<Output1>, Option<Output2>) {
    println!("start:");
    print_state(&s);

    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########
    let dist = vec![
        (0, 1, 1),
        (1, 7, 2),
        (1, 2, 2),
        (2, 9, 2),
        (2, 3, 2),
        (3, 4, 2),
        (3, 0xb, 2),
        (4, 0xd, 2),
        (4, 5, 2),
        (5, 6, 1),
        (7, 8, 1),
        (7, 2, 2),
        (9, 0xa, 1),
        (9, 3, 2),
        (0xb, 0xc, 1),
        (0xb, 4, 2),
        (0xd, 0xe, 1),
        (0xd, 5, 2),
    ];

    let dist = dist
        .iter()
        .map(|(s, g, c)| (*s, (*g, *c)))
        .chain(dist.iter().map(|(s, g, c)| (*g, (*s, *c))))
        .collect::<MultiMap<u8, (u8, i64)>>();

    let cost = [1, 1, 10, 10, 100, 100, 1000, 1000];
    println!("dist: {:x?}", dist);

    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########
    let goal = |s: &State| {
        ((s.pos[0] == 0x7 && s.pos[1] == 0x8) || (s.pos[0] == 0x8 && s.pos[1] == 0x7))
            && ((s.pos[2] == 0x9 && s.pos[3] == 0xa) || (s.pos[2] == 0xa && s.pos[3] == 0x9))
            && ((s.pos[4] == 0xb && s.pos[5] == 0xc) || (s.pos[4] == 0xc && s.pos[5] == 0xb))
            && ((s.pos[6] == 0xd && s.pos[7] == 0xe) || (s.pos[6] == 0xe && s.pos[7] == 0xd))
    };

    let successors = |s: &State| -> Vec<(State, i64)> {
        let mut new_states = Vec::new();

        for i in 0..8 {
            let in_hallway = (0..=6).contains(&s.pos[i]);

            if in_hallway && s.mover != -1 && s.mover != i as i8 {
                continue;
            }
            let pos = s.pos[i];

            for (g, c) in dist.get_vec(&pos).expect("missing dist entry") {
                if s.pos.iter().any(|other| other == g) {
                    // field occupied
                    continue;
                }

                if is_goal_for(pos, i) && !is_lower_goal(*g) && can_enter(i, &s.pos) {
                    continue;
                }
                if in_hallway && is_foreign_goal(i, *g) {
                    continue;
                }
                let move_to_goal = is_goal_for(*g, i);
                if in_hallway && move_to_goal && !can_enter(i, &s.pos) {
                    continue;
                }
                let mut n = s.clone();
                n.pos[i] = *g;

                n.mover = if move_to_goal { -1 } else { i as i8 };
                new_states.push((n, c * cost[i]));
            }
        }
        // if s.mover != -1 {
        //     let mut n = s.clone();
        //     n.mover = -1;
        //     new_states.push((n, 0));
        // }

        //  println!("new_states: {:?}", new_states);

        new_states
    };

    let res = pathfinding::directed::dijkstra::dijkstra(&s, successors, goal);
    println!("res: {:?}", res);

    if let Some((states, _cost)) = res {
        for s in states.iter() {
            print_state(s);
            println!("--------------------");
        }
    }
    // let input = s.trim().split(',').map(|s| s.parse::<i64>().unwrap());
    (None, None)
}

fn is_foreign_goal(i: usize, g: u8) -> bool {
    ((i == 0 || i == 1) && (g == 9 || g == 0xb || g == 0xd))
        || ((i == 2 || i == 3) && (g == 7 || g == 0xb || g == 0xd))
        || ((i == 4 || i == 5) && (g == 7 || g == 0x9 || g == 0xd))
        || ((i == 6 || i == 7) && (g == 7 || g == 0x9 || g == 0xb))
}

fn is_lower_goal(g: u8) -> bool {
    g == 8 || g == 0xa || g == 0xc || g == 0xe
}

fn can_enter(i: usize, pos: &[u8; 8]) -> bool {
    ((i == 0 || i == 1)
        && pos[2] != 8
        && pos[3] != 8
        && pos[4] != 8
        && pos[5] != 8
        && pos[6] != 8
        && pos[7] != 8)
        || ((i == 2 || i == 3)
            && pos[0] != 0xa
            && pos[1] != 0xa
            && pos[4] != 0xa
            && pos[5] != 0xa
            && pos[6] != 0xa
            && pos[7] != 0xa)
        || ((i == 4 || i == 5)
            && pos[0] != 0xc
            && pos[1] != 0xc
            && pos[2] != 0xc
            && pos[3] != 0xc
            && pos[6] != 0xc
            && pos[7] != 0xc)
        || ((i == 6 || i == 7)
            && pos[0] != 0xe
            && pos[1] != 0xe
            && pos[2] != 0xe
            && pos[3] != 0xe
            && pos[4] != 0xe
            && pos[5] != 0xe)
}

fn is_goal_for(g: u8, i: usize) -> bool {
    (i == 0 || i == 1) & (g == 7 || g == 8)
        || (i == 2 || i == 3) & (g == 9 || g == 0xa)
        || (i == 4 || i == 5) & (g == 0xb || g == 0xc)
        || (i == 6 || i == 7) & (g == 0xd || g == 0xe)
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
        pos: [0xa, 0xc, 0xd, 0xe, 0x8, 0x9, 0x7, 0xb],
        mover: -1,
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
    // #############
    // #...........#
    // ###B#C#B#D###
    //   #A#D#C#A#
    //   #########

    // #############
    // #01.2.3.4.56#
    // ###7#9#b#d###
    //   #8#a#c#e#
    //   #########
    &[
        // (
        //     State {
        //         pos: [0x8, 0x7, 0xa, 0x9, 0xc, 0xd, 0xe, 0xb],
        //         mover: -1,
        //     },
        //     None,
        //     None,
        // ),
        // (
        //     State {
        //         pos: [0x9, 0xa, 0x7, 0x8, 0xd, 0xc, 0xb, 0xe],
        //         mover: -1,
        //     },
        //     None,
        //     None,
        // ),
        (
            State {
                pos: [0x8, 0xe, 0x7, 0x2, 0x9, 0xc, 0xa, 0xd],
                mover: -1,
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
        #...B.......#
        ###B#C#.#D###
          #A#D#C#A#
          #########
                      */
        (
            State {
                //       a      b       c        d
                pos: [8, 0xe, 7, 0xb, 9, 0xc, 0xa, 0xd],
                mover: -1,
            },
            None,
            None,
        ),
    ]
}
