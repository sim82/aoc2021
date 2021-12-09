use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
