use pathfinding::matrix::Matrix;

pub mod template;

// Use this file to add helper functions and additional modules.

pub type Position = (usize, usize);

pub fn vecvec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn matrix_from_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap()
}
