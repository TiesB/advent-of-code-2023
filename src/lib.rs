use pathfinding::matrix::Matrix;

pub mod template;

// Use this file to add helper functions and additional modules.

pub type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Horizontal(isize),
    Vertical(isize),
}

impl Direction {
    // Should use generics and traits
    pub fn apply(&self, (y, x): Position) -> Position {
        match self {
            Direction::Horizontal(d) => (y, x.checked_add_signed(*d).unwrap()),
            Direction::Vertical(d) => (y.checked_add_signed(*d).unwrap(), x),
        }
    }

    pub fn get_d(&self) -> (isize, isize) {
        match self {
            Direction::Horizontal(d) => (0, *d),
            Direction::Vertical(d) => (*d, 0),
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Horizontal(d) => Direction::Horizontal(-*d),
            Direction::Vertical(d) => Direction::Vertical(-*d),
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Vertical(-1),
            Direction::Vertical(1),
            Direction::Horizontal(-1),
            Direction::Horizontal(1),
        ]
    }
}

pub fn vecvec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn matrix_from_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap()
}
