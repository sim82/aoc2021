pub mod parser;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
