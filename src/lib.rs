use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Sub},
};

use itertools::Itertools;

pub mod parser;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}
impl Vec2 {
    pub fn ortho_neighbors(&self) -> [Vec2; 4] {
        [
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
    pub fn neighbors(&self) -> [Vec2; 8] {
        [
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Dir3 {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

pub const ORIENTATIONS: [(Dir3, Dir3, Dir3); 24] = [
    (Dir3::XPos, Dir3::YPos, Dir3::ZPos),
    (Dir3::XPos, Dir3::YNeg, Dir3::ZNeg),
    (Dir3::XPos, Dir3::ZPos, Dir3::YNeg),
    (Dir3::XPos, Dir3::ZNeg, Dir3::YPos),
    (Dir3::XNeg, Dir3::YPos, Dir3::ZNeg),
    (Dir3::XNeg, Dir3::YNeg, Dir3::ZPos),
    (Dir3::XNeg, Dir3::ZPos, Dir3::YPos),
    (Dir3::XNeg, Dir3::ZNeg, Dir3::YNeg),
    (Dir3::YPos, Dir3::XPos, Dir3::ZNeg),
    (Dir3::YPos, Dir3::XNeg, Dir3::ZPos),
    (Dir3::YPos, Dir3::ZPos, Dir3::XPos),
    (Dir3::YPos, Dir3::ZNeg, Dir3::XNeg),
    (Dir3::YNeg, Dir3::XPos, Dir3::ZPos),
    (Dir3::YNeg, Dir3::XNeg, Dir3::ZNeg),
    (Dir3::YNeg, Dir3::ZPos, Dir3::XNeg),
    (Dir3::YNeg, Dir3::ZNeg, Dir3::XPos),
    (Dir3::ZPos, Dir3::XPos, Dir3::YPos),
    (Dir3::ZPos, Dir3::XNeg, Dir3::YNeg),
    (Dir3::ZPos, Dir3::YPos, Dir3::XNeg),
    (Dir3::ZPos, Dir3::YNeg, Dir3::XPos),
    (Dir3::ZNeg, Dir3::XPos, Dir3::YNeg),
    (Dir3::ZNeg, Dir3::XNeg, Dir3::YPos),
    (Dir3::ZNeg, Dir3::YPos, Dir3::XPos),
    (Dir3::ZNeg, Dir3::YNeg, Dir3::XNeg),
];

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn manhattan_dist(&self, other: &Vec3) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn get_component(&self, d: Dir3) -> i64 {
        match d {
            Dir3::XPos => self.x,
            Dir3::XNeg => -self.x,
            Dir3::YPos => self.y,
            Dir3::YNeg => -self.y,
            Dir3::ZPos => self.z,
            Dir3::ZNeg => -self.z,
        }
    }

    pub fn permute(&self, p: &(Dir3, Dir3, Dir3)) -> Self {
        Vec3 {
            x: self.get_component(p.0),
            y: self.get_component(p.1),
            z: self.get_component(p.2),
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[test]
fn test_vec3() {
    assert_eq!(
        Vec3 { x: 1, y: 1, z: 1 }.manhattan_dist(&Vec3 { x: 2, y: 3, z: 4 }),
        6
    );
}

#[derive(Debug)]
pub struct BingoBoard {
    pub column_count: [usize; 5],
    pub row_count: [usize; 5],
    pub field_map: HashMap<i64, (usize, usize)>,
}

impl BingoBoard {
    pub fn new(numbers: Vec<Vec<i64>>) -> Self {
        let mut field_map = HashMap::new();
        for row in 0..5 {
            for column in 0..5 {
                field_map.insert(numbers[row][column], (column, row));
            }
        }

        Self {
            field_map,
            row_count: [0; 5],
            column_count: [0; 5],
        }
    }

    pub fn apply(&mut self, number: i64) -> bool {
        match self.field_map.entry(number) {
            std::collections::hash_map::Entry::Occupied(e) => {
                let (column, row) = *e.get();
                self.column_count[column] += 1;
                self.row_count[row] += 1;

                e.remove_entry();
                self.column_count.iter().any(|c| *c >= 5) || self.row_count.iter().any(|c| *c >= 5)
            }
            std::collections::hash_map::Entry::Vacant(_) => false,
        }
    }
    pub fn count_remaining(&self) -> i64 {
        self.field_map.keys().sum::<i64>()
    }
}

pub fn lowercase_char_to_index(c: char) -> usize {
    assert!(c.is_ascii_lowercase());
    (c as u8 - b'a') as usize
}

fn char_to_int(c: char) -> i64 {
    assert!(c.is_digit(10));
    (c as u8 - b'0') as i64
}

pub fn read_i64_field(s: &str) -> HashMap<Vec2, i64> {
    s.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, c)| {
                (
                    Vec2 {
                        x: x as i64,
                        y: y as i64,
                    },
                    char_to_int(c),
                )
            })
        })
        .collect()
}

/// Compute the number of possible paths from 'start' through to a node for which 'success' returns
/// 'true'.
///
/// - 'start' is the starting node.
/// - 'successors' returns the list of successors for a given node.
/// - 'success' checks whether the goal has been reached

pub fn bfs_count_paths<S, FN, FS, IN>(start: S, mut successors: FN, mut success: FS) -> usize
where
    S: Clone,
    FN: FnMut(&S) -> IN,
    IN: IntoIterator<Item = S>,
    FS: FnMut(&S) -> bool,
{
    let mut queue = VecDeque::<S>::new();
    queue.push_back(start);
    let mut count = 0;
    while let Some(s) = queue.pop_front() {
        if success(&s) {
            count += 1;
            continue;
        }
        queue.extend(successors(&s).into_iter());
    }

    count
}

pub fn count_occurrences<T: Eq + Clone + std::hash::Hash, IN: IntoIterator<Item = T>>(
    i: IN,
) -> HashMap<T, i64> {
    let mut counts = HashMap::new();
    for e in i.into_iter() {
        match counts.entry(e.clone()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn i64_field_bounds(field: &HashMap<Vec2, i64>) -> (Vec2, Vec2) {
    let (minx, maxx) = field.keys().map(|c| c.x).minmax().into_option().unwrap();
    let (miny, maxy) = field.keys().map(|c| c.y).minmax().into_option().unwrap();

    (Vec2 { x: minx, y: miny }, Vec2 { x: maxx, y: maxy })
}

pub fn dump_bool_field(field: &HashSet<Vec2>) {
    if field.is_empty() {
        return;
    }
    let (minx, maxx) = field.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (miny, maxy) = field.iter().map(|c| c.y).minmax().into_option().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if field.contains(&Vec2 { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// pub enum SnDir {
//     None,
//     Left,
//     Right,
// }

// pub enum SnLink {
//     Number(i64),
//     Node(usize),
// }

// pub struct SnNode {
//     from_dir: SnDir,
//     parent: usize,
//     id: usize,
//     left: SnLink,
//     right: SnLink,
// }

// #[derive(Default)]
// pub struct SnNodes {
//     next: usize,
//     nodes: HashMap<usize, SnNode>,
// }

// impl SnNodes {
//     pub fn put(&mut self, mut node: SnNode) {
//         node.id = self.next;
//         self.next += 1;
//         self.nodes.insert(node.id, node);
//     }

//     pub fn get(&self, id: usize) -> &SnNode {
//         self.nodes.get(&id).unwrap()
//     }
//     pub fn get_mut(&mut self, id: usize) -> &SnNode {
//         self.nodes.get_mut(&id).unwrap()
//     }
// }

#[derive(Debug, Clone)]
pub enum SfNumber {
    Number(i64),
    Pair(Box<SfNumber>, Box<SfNumber>),
    Exploded,
}

impl SfNumber {
    pub fn traverse_left_to_right(&self) {
        match self {
            SfNumber::Number(v) => println!("v: {}", v),
            SfNumber::Pair(l, r) => {
                l.traverse_left_to_right();
                r.traverse_left_to_right();
            }
            _ => panic!("bad node type"),
        }
    }

    pub fn traverse_left_to_right_vec(&mut self, level: usize) -> Vec<(&mut SfNumber, usize)> {
        match self {
            SfNumber::Number(v) => vec![(self, level)],
            SfNumber::Pair(l, r) => {
                let mut vl = l.traverse_left_to_right_vec(level + 1);
                let mut vr = r.traverse_left_to_right_vec(level + 1);
                vl.append(&mut vr);
                vl
            }
            _ => panic!("bad node"),
        }
    }
    pub fn is_exploded(&self) -> bool {
        matches!(self, SfNumber::Exploded)
    }
    pub fn prune_exploded(&mut self) {
        match self {
            SfNumber::Pair(a, b) if a.is_exploded() && b.is_exploded() => {
                *self = SfNumber::Number(0)
            }
            SfNumber::Pair(a, b) => {
                a.prune_exploded();
                b.prune_exploded();
            }
            _ => {}
        }
    }

    pub fn split(&mut self) -> bool {
        match self {
            SfNumber::Number(n) if *n > 9 => {
                *self = SfNumber::Pair(
                    Box::new(SfNumber::Number(*n / 2)),
                    Box::new(SfNumber::Number(*n / 2 + *n % 2)),
                );
                true
            }
            SfNumber::Pair(a, b) => a.split() || b.split(),
            _ => false,
        }
    }
    // pub fn traverse_pairs_left_to_right_vec(
    //     &mut self,
    //     level: usize,
    // ) -> Vec<((&mut i64, &mut i64), usize)> {
    //     match self {
    //         SfNumber::Number(v) => vec![(self, level)],
    //         SfNumber::Pair(l, r) => {
    //             let mut vl = l.traverse_left_to_right_vec(level + 1);
    //             let mut vr = r.traverse_left_to_right_vec(level + 1);
    //             vl.append(&mut vr);
    //             vl
    //         }
    //     }
    // }

    pub fn reduce(&mut self) {
        loop {
            {
                let mut v = self.traverse_left_to_right_vec(0);
                println!("{:?}", v);

                if let Some((explode_pos, _)) = v
                    .windows(2)
                    .find_position(|x| x[0].1 == x[1].1 && x[0].1 >= 5)
                {
                    // println!("explode: {:?}", explode_pos);
                    // *v[explode_pos].0 = SfNumber::Exploded;
                    // *v[explode_pos + 1].0 = SfNumber::Exploded;

                    if explode_pos > 0 {
                        let n = match (&v[explode_pos - 1].0, &v[explode_pos].0) {
                            (SfNumber::Number(a), SfNumber::Number(b)) => a + b,
                            _ => panic!("bad nodes"),
                        };
                        *v[explode_pos - 1].0 = SfNumber::Number(n);
                    }
                    if explode_pos < v.len() - 2 {
                        let n = match (&v[explode_pos + 1].0, &v[explode_pos + 2].0) {
                            (SfNumber::Number(a), SfNumber::Number(b)) => a + b,
                            _ => panic!("bad nodes"),
                        };
                        *v[explode_pos + 2].0 = SfNumber::Number(n);
                    }
                    *v[explode_pos].0 = SfNumber::Exploded;
                    *v[explode_pos + 1].0 = SfNumber::Exploded;
                    self.prune_exploded();
                    continue;
                } else if self.split() {
                    // println!("split");
                    continue;
                }
            }
            break;
        }
    }
    pub fn get_magnitude(&self) -> i64 {
        match self {
            SfNumber::Number(n) => *n,
            SfNumber::Pair(a, b) => 3 * a.get_magnitude() + 2 * b.get_magnitude(),
            _ => panic!("bad node"),
        }
    }
}
