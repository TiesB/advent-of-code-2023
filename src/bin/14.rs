use pathfinding::matrix::Matrix;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

advent_of_code::solution!(14);

trait Platform {
    fn tilt(&mut self);
    fn load(&self) -> usize;
}

impl Platform for Matrix<char> {
    fn tilt(&mut self) {
        let mut rocks: Vec<usize> = vec![0; self.columns];

        for y in 0..self.rows {
            for x in 0..self.columns {
                let c = self[(y, x)];
                match c {
                    'O' => {
                        self[(y, x)] = '.';
                        self[(rocks[x], x)] = 'O';
                        rocks[x] += 1;
                    }
                    '#' => {
                        rocks[x] = y + 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn load(&self) -> usize {
        let height = self.rows;

        let mut res = 0;
        for (y, row) in self.iter().enumerate() {
            for c in row {
                if *c == 'O' {
                    res += height - y;
                }
            }
        }

        res
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let mut rocks: Vec<usize> = vec![0; lines[0].len()];

    let mut res = 0;
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                'O' => {
                    res += height - rocks[x];
                    rocks[x] += 1;
                }
                '#' => {
                    rocks[x] = y + 1;
                }
                _ => (),
            }
        }
    }

    Some(res)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn part_two(input: &str) -> Option<usize> {
    let n = 1000000000;

    let mut matrix: Matrix<char> =
        Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap();

    let mut m: HashMap<u64, usize> = HashMap::new();

    for cycle in 1..=n {
        for _ in 0..4 {
            matrix.tilt();
            matrix.rotate_cw(1);
        }

        let hash = calculate_hash(&matrix);
        if let Some(prev) = m.get(&hash) {
            let cycle_length = cycle - prev;

            if (n - prev) % cycle_length == 0 {
                return Some(matrix.load());
            }
        } else {
            m.insert(hash, cycle);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
