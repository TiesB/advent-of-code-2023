use pathfinding::matrix::Matrix;

pub mod template;

// Use this file to add helper functions and additional modules.

pub type Position = (usize, usize);
pub type IPosition = (i32, i32);

pub trait Neighbours {
    fn neighbours(&self) -> Vec<Self>
    where
        Self: std::marker::Sized;
}

impl Neighbours for Position {
    fn neighbours(&self) -> Vec<Position> {
        // TODO might be faster to just return vec![(self.0+1) ...]
        ALL_DIRECTIONS.iter().map(|d| d.apply(*self)).collect()
    }
}

impl Neighbours for IPosition {
    fn neighbours(&self) -> Vec<IPosition> {
        // TODO might be faster to just return vec![(self.0+1) ...]
        vec![
            (self.0, self.1 + 1),
            (self.0 + 1, self.1),
            (self.0, self.1 - 1),
            (self.0 - 1, self.1),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Horizontal(i8),
    Vertical(i8),
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Vertical(-1),
    Direction::Vertical(1),
    Direction::Horizontal(-1),
    Direction::Horizontal(1),
];

pub trait Apply<T> {
    fn apply(&self, x: T) -> T;
}

impl Direction {
    // Should use generics and traits

    pub fn get_d(&self) -> (i8, i8) {
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
}

impl Apply<Position> for Direction {
    fn apply(&self, (y, x): Position) -> Position {
        match self {
            Direction::Horizontal(d) => (y, x.checked_add_signed(*d as isize).unwrap()),
            Direction::Vertical(d) => (y.checked_add_signed(*d as isize).unwrap(), x),
        }
    }
}

impl Apply<IPosition> for Direction {
    fn apply(&self, (y, x): IPosition) -> IPosition {
        match self {
            Direction::Horizontal(d) => (y, x + *d as i32),
            Direction::Vertical(d) => (y + *d as i32, x),
        }
    }
}

pub fn vecvec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn matrix_from_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap()
}
