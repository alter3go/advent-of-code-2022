// Mutably borrowed ;) from https://stackoverflow.com/questions/13102786/two-dimensional-vectors-in-rust
use std::fmt;

#[derive(Clone, Debug)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    pub row_count: usize,
    pub col_count: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(vec.len() == row * col);
        Self {
            vec,
            row_count: row,
            col_count: col,
        }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col_count * row;
        &self.vec[i..(i + self.col_count)]
    }

    pub fn col(&self, col: usize) -> Vec<&T> {
        (col..self.vec.len())
            .step_by(self.col_count)
            .map(|i| &self.vec[i])
            .collect()
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = self.col_count * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.col_count * row;
        &mut self.vec[i + col]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.row_count {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}

#[test]
fn test_vec2d_row_col() {
    let a: Vec2d<u8> = Vec2d::new((1..=30).collect(), 6, 5);
    assert_eq!(a.col(0), vec![&1, &6, &11, &16, &21, &26]);
    assert_eq!(a.row(0), &[1, 2, 3, 4, 5]);
}
