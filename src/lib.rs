use std::collections::{HashMap, HashSet, VecDeque};

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
