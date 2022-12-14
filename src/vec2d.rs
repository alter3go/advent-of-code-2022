// Mutably borrowed ;) from https://stackoverflow.com/questions/13102786/two-dimensional-vectors-in-rust
use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2d<T>
where
    T: Copy,
{
    vec: Vec<T>,
    pub row_count: usize,
    pub col_count: usize,
}

impl<T> Vec2d<T>
where
    T: Copy,
{
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

    pub fn col(&self, col: usize) -> Vec<T> {
        (col..self.vec.len())
            .step_by(self.col_count)
            .map(|i| self.vec[i])
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

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T>
where
    T: Copy,
{
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
    assert_eq!(a.col(0), vec![1, 6, 11, 16, 21, 26]);
    assert_eq!(a.row(0), &[1, 2, 3, 4, 5]);
}

pub fn input_from_file(filename: &str) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut input = io::BufReader::new(File::open(filename).unwrap());
    loop {
        let mut line = Vec::new();
        match input.read_until('\n' as u8, &mut line) {
            Err(_) => break,
            _ => {
                if line.len() < 1 {
                    break;
                } else if line.last() == Some(&('\n' as u8)) {
                    line.pop();
                }
            }
        }
        result.push(line);
    }
    result
}
