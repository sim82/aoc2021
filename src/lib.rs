use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Deref, RangeInclusive, Sub},
};

use itertools::Itertools;

pub mod parser;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}
impl Vec2 {
    pub fn new(x: i64, y: i64) -> Vec2 {
        Vec2 { x, y }
    }
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
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
    pub fn self_and_neighbors(&self) -> [Vec2; 9] {
        [
            Vec2 {
                x: self.x - 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            *self,
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x + 1,
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

pub fn bool_field_bounds(field: &HashSet<Vec2>) -> (Vec2, Vec2) {
    let (minx, maxx) = field.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (miny, maxy) = field.iter().map(|c| c.y).minmax().into_option().unwrap();
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cube {
    pub xrange: RangeInclusive<i64>,
    pub yrange: RangeInclusive<i64>,
    pub zrange: RangeInclusive<i64>,
}

impl Cube {
    pub fn new(
        xrange: RangeInclusive<i64>,
        yrange: RangeInclusive<i64>,
        zrange: RangeInclusive<i64>,
    ) -> Self {
        Cube {
            xrange,
            yrange,
            zrange,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Overlap {
    Equals,
    Contains,
    Contained,
    Start,
    End,
    None,
}

fn overlap_type(a: RangeInclusive<i64>, b: RangeInclusive<i64>) -> Overlap {
    let astart = a.start();
    let aend = a.end();
    let bstart = b.start();
    let bend = b.end();

    //              0 1 2 3 4 5 6 7 8 9
    //  Contains:   . . . . . . . . . .
    //                        . . . . .
    //              . . . .
    //                  . . . . . . .

    // Contained:       . . . . . . .
    //              . . . . . . . . . .

    //  Start:      . . . .
    //                  . .
    //                  . . . . . .

    // End:             . . . . . .
    //                  . .
    //              . . . .

    if a == b {
        Overlap::Equals
    } else if astart <= bstart && aend >= bend {
        Overlap::Contains
    } else if astart > bstart && aend < bend {
        Overlap::Contained
    } else if astart <= bstart && aend >= bstart {
        Overlap::Start
    } else if astart <= bend && aend >= bend {
        Overlap::End
    } else {
        Overlap::None
        // panic!("overlap {}..{} {}..{}", astart, aend, bstart, bend);
        // unreachable!()
    }
}

#[test]
fn test_overlap() {
    assert_eq!(overlap_type(-10..=10, -10..=10), Overlap::Equals);
    assert_eq!(overlap_type(0..=11, 1..=10), Overlap::Contains);
    assert_eq!(overlap_type(-9..=9, -10..=10), Overlap::Contained);
    assert_eq!(overlap_type(-10..=9, -9..=10), Overlap::Start);
    assert_eq!(overlap_type(-10..=9, -10..=10), Overlap::Start);
    assert_eq!(overlap_type(-9..=11, -10..=10), Overlap::End);
    assert_eq!(overlap_type(-9..=10, -10..=10), Overlap::End);
    assert_eq!(overlap_type(-10..=-5, -4..=10), Overlap::None);
}

impl Cube {
    pub fn overlaps(&self, other: &Cube) -> bool {
        overlap_type(self.xrange.clone(), other.xrange.clone()) != Overlap::None
            && overlap_type(self.yrange.clone(), other.yrange.clone()) != Overlap::None
            && overlap_type(self.zrange.clone(), other.zrange.clone()) != Overlap::None
    }

    pub fn split_side(
        s: RangeInclusive<i64>,
        r: RangeInclusive<i64>,
    ) -> (Vec<RangeInclusive<i64>>, Vec<RangeInclusive<i64>>) {
        let sstart = *s.start();
        let send = *s.end();
        let rstart = *r.start();
        let rend = *r.end();

        let overlap_type = overlap_type(r, s.clone());

        match overlap_type {
            Overlap::Equals | Overlap::Contains => (vec![s], vec![]),
            Overlap::Contained => {
                // Contained:       . . . . . . .
                //              . . . . . . . . . .
                assert!(rstart <= rend);
                assert!(sstart < rstart);
                assert!(rend < send);
                (
                    vec![rstart..=rend],
                    vec![sstart..=rstart - 1, rend + 1..=send],
                )
            }
            Overlap::Start => {
                //  Start:      . . . .
                //                  . .
                //                  . . . . . .
                assert!(sstart <= rend);
                assert!(rend < send);
                (vec![sstart..=rend], vec![rend + 1..=send])
            }
            Overlap::End => {
                // End:             . . . . . .
                //                  . .
                //              . . . .
                assert!(rstart <= send);
                assert!(sstart < rstart);
                (vec![rstart..=send], vec![sstart..=rstart - 1])
            }
            Overlap::None => (vec![], vec![s]),
        }
    }

    pub fn split(&self, rcube: &Cube) -> Option<(Vec<Cube>, Vec<Cube>)> {
        let mut new_cubes_out = Vec::new();
        let (in_range, out_range) = Cube::split_side(self.xrange.clone(), rcube.xrange.clone());

        let new_cubes_in_x = in_range
            .iter()
            .cloned()
            .map(|xrange| Cube {
                xrange,
                yrange: self.yrange.clone(),
                zrange: self.zrange.clone(),
            })
            .collect::<Vec<_>>();
        new_cubes_out.extend(out_range.iter().cloned().map(|xrange| Cube {
            xrange,
            yrange: self.yrange.clone(),
            zrange: self.zrange.clone(),
        }));

        if new_cubes_in_x.is_empty() {
            return None;
        }

        let mut new_cubes_in_y = Vec::new();
        for cube in new_cubes_in_x {
            let (in_range, out_range) = Cube::split_side(cube.yrange.clone(), rcube.yrange.clone());

            new_cubes_in_y.extend(in_range.iter().cloned().map(|yrange| Cube {
                xrange: cube.xrange.clone(),
                yrange,
                zrange: cube.zrange.clone(),
            }));

            new_cubes_out.extend(out_range.iter().cloned().map(|yrange| Cube {
                xrange: cube.xrange.clone(),
                yrange,
                zrange: cube.zrange.clone(),
            }));
        }

        if new_cubes_in_y.is_empty() {
            return None;
        }

        let mut new_cubes_in_z = Vec::new();
        for cube in new_cubes_in_y {
            let (in_range, out_range) = Cube::split_side(cube.zrange.clone(), rcube.zrange.clone());

            new_cubes_in_z.extend(in_range.iter().cloned().map(|zrange| Cube {
                xrange: cube.xrange.clone(),
                yrange: cube.yrange.clone(),
                zrange,
            }));

            new_cubes_out.extend(out_range.iter().cloned().map(|zrange| Cube {
                xrange: cube.xrange.clone(),
                yrange: cube.yrange.clone(),
                zrange,
            }));
        }

        assert_eq!(
            new_cubes_in_z.iter().fold(0, |a, c| a + c.volume())
                + new_cubes_out.iter().fold(0, |a, c| a + c.volume()),
            self.volume()
        );

        Some((new_cubes_in_z, new_cubes_out))
    }

    pub fn volume(&self) -> i64 {
        let xstart = self.xrange.start();
        let xend = self.xrange.end();

        let ystart = self.yrange.start();
        let yend = self.yrange.end();

        let zstart = self.zrange.start();
        let zend = self.zrange.end();

        (xend - xstart + 1) * (yend - ystart + 1) * (zend - zstart + 1)
    }
}

#[test]
fn test_cube_split() {
    let a = Cube {
        xrange: 10..=12,
        yrange: 10..=12,
        zrange: 10..=12,
    };
    let b = Cube {
        xrange: 11..=13,
        yrange: 10..=12,
        zrange: 10..=12,
    };

    let (inside, outside) = a.split(&b).unwrap();
    assert_eq!(
        inside,
        vec![Cube {
            xrange: 11..=12,
            yrange: 10..=12,
            zrange: 10..=12,
        }]
    );
    assert_eq!(
        outside,
        vec![Cube {
            xrange: 10..=10,
            yrange: 10..=12,
            zrange: 10..=12,
        }]
    );
    println!(
        "{:?} {:?} {} {}",
        inside[0],
        outside[0],
        inside[0].volume(),
        outside[0].volume()
    );
    assert_eq!(inside[0].volume() + outside[0].volume(), a.volume());
}

#[test]
fn test_cube() {
    let s = Cube {
        xrange: 10..=12,
        yrange: 10..=12,
        zrange: 10..=12,
    };
    assert_eq!(s.volume(), 27);

    let s = Cube {
        xrange: -10..=10,
        yrange: -15..=15,
        zrange: -20..=20,
    };

    let r = Cube {
        xrange: -2..=3,
        yrange: -4..=5,
        zrange: -6..=7,
    };

    let (in_cubes, out_cubes) = s.split(&r).unwrap();
    assert_eq!(in_cubes.len(), 1);
    assert_eq!(in_cubes[0], r);

    assert_eq!(out_cubes.len(), 6);

    println!("in: {:?}", in_cubes);
    println!("out: {:?}", out_cubes);

    assert_eq!(
        s.volume(),
        in_cubes
            .iter()
            .chain(out_cubes.iter())
            .fold(0, |a, c| a + c.volume())
    );

    assert_eq!(
        Cube {
            xrange: -5..=5,
            yrange: -5..=5,
            zrange: -5..=5
        }
        .volume(),
        11 * 11 * 11
    );
}

#[derive(Default)]
pub struct CubeSet {
    cubes: Vec<Cube>,
}

impl CubeSet {
    pub fn add(&mut self, add_cube: Cube, on_off: bool) {
        // println!("\nadd {:?} {:?}", add_cube, on_off);
        let mut new_cubes = Vec::new();
        for cube in self.cubes.drain(..) {
            // println!("{:?}", cube);

            if let Some((_in_cubes, out_cubes)) = cube.split(&add_cube) {
                // println!("-> {:?} {:?}", _in_cubes, out_cubes);
                new_cubes.extend(out_cubes);
            } else {
                // println!("no overlap");
                new_cubes.push(cube);
            }
        }

        self.cubes = new_cubes;
        if on_off {
            self.cubes.push(add_cube);
        }

        // let vol = self.cubes.iter().fold(0, |a, c| a + c.volume());
    }

    pub fn volume(&self) -> i64 {
        self.cubes.iter().fold(0, |a, c| a + c.volume())
    }
    pub fn volume_intersect(&self, add_cube: &Cube) -> i64 {
        let mut new_cubes = Vec::new();
        for cube in self.cubes.iter() {
            // println!("{:?}", cube);

            if let Some((in_cubes, _out_cubes)) = cube.split(add_cube) {
                // println!("-> {:?} {:?}", _in_cubes, out_cubes);
                new_cubes.extend(in_cubes);
            }
        }
        new_cubes.iter().fold(0, |a, c| a + c.volume())
    }
}

#[test]
pub fn test_cube_set() {
    let mut cube_set = CubeSet::default();

    cube_set.add(Cube::new(0..=0, 0..=1, 0..=2), true);
    assert_eq!(cube_set.volume(), 6);
    cube_set.add(Cube::new(0..=0, 0..=0, 0..=0), false);
    assert_eq!(cube_set.volume(), 5);
}
#[test]
pub fn test_cube_set_vol() {
    let mut cube_set = CubeSet::default();
    cube_set.add(Cube::new(-100..=0, -100..=0, -100..=0), true);
    cube_set.add(Cube::new(0..=100, 0..=100, 0..=100), true);

    println!(
        "{}",
        cube_set.volume_intersect(&Cube::new(-50..=50, -50..=50, -50..=50))
    );
}
