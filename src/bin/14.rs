use std::{
    collections::HashMap,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

advent_of_code::solution!(14);

type GridT = Vec<Vec<char>>;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Grid {
    grid: GridT,
}

impl Grid {
    pub fn rotated_cw_clone(&self) -> Self {
        let mut rotated: GridT = (0..self.grid[0].len()).map(|_| vec![]).collect();

        self.grid.iter().rev().for_each(|original_row| {
            for (item, transposed_row) in original_row.iter().zip(&mut rotated) {
                transposed_row.push(*item);
            }
        });

        Self { grid: rotated }
    }

    pub fn tilted_clone(&self) -> Self {
        let width = self.grid[0].len();
        let mut rocks: Vec<usize> = vec![0; width];
        let mut new_grid = self.clone();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match *c {
                    'O' => {
                        new_grid.grid[y][x] = '.';
                        new_grid.grid[rocks[x]][x] = 'O';
                        rocks[x] += 1;
                    }
                    '#' => {
                        rocks[x] = y + 1;
                    }
                    _ => (),
                }
            }
        }

        new_grid
    }

    pub fn load(&self) -> usize {
        let height = self.grid.len();

        let mut res = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for c in row {
                if *c == 'O' {
                    res += height - y;
                }
            }
        }

        res
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Self {
            grid: s.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<String> = self.grid.iter().map(|line| line.iter().collect()).collect();
        write!(f, "{}", strings.join("\n"))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid = Grid::from(input);
    let height = grid.grid.len();
    let width = grid.grid[0].len();

    let mut rocks: Vec<usize> = vec![0; width];

    let mut res = 0;
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
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
    let mut grid: Grid = Grid::from(input);

    let mut m: HashMap<u64, usize> = HashMap::new();

    for cycle in 1..=n {
        for _ in 0..4 {
            grid = grid.tilted_clone().rotated_cw_clone()
        }

        let hash = calculate_hash(&grid);
        if let Some(prev) = m.get(&hash) {
            let cycle_length = cycle - prev;

            if (n - prev) % cycle_length == 0 {
                return Some(grid.load());
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
