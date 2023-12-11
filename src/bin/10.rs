use std::collections::HashMap;

advent_of_code::solution!(10);

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn apply(&self, (x, y): Position) -> Position {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl Tile {
    pub fn can_be_reached(&self, by: Direction) -> Option<Direction> {
        if let Tile::Pipe(dir_a, dir_b) = self {
            if *dir_a == by.reverse() {
                return Some(*dir_b);
            } else if *dir_b == by.reverse() {
                return Some(*dir_a);
            }
        }
        None
    }
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn find_start(&self) -> Position {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }

    pub fn find_start_and_next(&self) -> ((Position, Tile), (Position, Direction)) {
        let start_pos = self.find_start();
        let mut one: Option<(Position, Direction)> = None;
        let mut two: Option<(Position, Direction)> = None;

        for (next_pos, next_dir, next_tile) in self.neighbors(start_pos) {
            if next_tile.can_be_reached(next_dir).is_some() {
                if one.is_none() {
                    one = Some((next_pos, next_dir));
                } else {
                    two = Some((next_pos, next_dir));
                }
            }
        }

        if two.is_none() {
            panic!("No nexts found for start");
        }

        (
            (start_pos, Tile::Pipe(two.unwrap().1, one.unwrap().1)),
            one.unwrap(),
        )
    }

    pub fn neighbors(&self, (x, y): Position) -> Vec<(Position, Direction, Tile)> {
        let mut res = Vec::new();

        if x > 0 {
            res.push(((x - 1, y), Direction::West, self.grid[y][x - 1]));
        }
        if x < self.width() - 1 {
            res.push(((x + 1, y), Direction::East, self.grid[y][x + 1]));
        }
        if y > 0 {
            res.push(((x, y - 1), Direction::North, self.grid[y - 1][x]));
        }
        if y < self.height() - 1 {
            res.push(((x, y + 1), Direction::South, self.grid[y + 1][x]));
        }

        res
    }

    pub fn find_loop(&self) -> HashMap<Position, Tile> {
        let (start, mut next) = self.find_start_and_next();
        let mut res = HashMap::from([start]);

        while next.0 != start.0 {
            let tile = self.grid[next.0 .1][next.0 .0];
            res.insert(next.0, tile);

            if let Some(new_dir) = tile.can_be_reached(next.1) {
                next = (new_dir.apply(next.0), new_dir)
            } else {
                panic!();
            }
        }

        res
    }

    pub fn max_distance(&self) -> usize {
        self.find_loop().len() / 2
    }

    pub fn inside_count(&self) -> usize {
        let l = self.find_loop();

        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for x in 0..row.len() {
                if l.contains_key(&(x, y)) {
                    continue;
                }

                count += (0..x)
                    .filter(|xx| {
                        l.get(&(*xx, y)).is_some_and(|tile| match tile {
                            Tile::Pipe(Direction::North, _) => true,
                            Tile::Start => panic!("aaa"),
                            _ => false,
                        })
                    })
                    .count()
                    % 2
            }
        }

        count
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        if input.is_empty() {
            panic!("Empty grid not supported");
        }
        Self {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '|' => Tile::Pipe(Direction::North, Direction::South),
                            '-' => Tile::Pipe(Direction::East, Direction::West),
                            'L' => Tile::Pipe(Direction::North, Direction::East),
                            'J' => Tile::Pipe(Direction::North, Direction::West),
                            '7' => Tile::Pipe(Direction::South, Direction::West),
                            'F' => Tile::Pipe(Direction::South, Direction::East),
                            '.' => Tile::Ground,
                            'S' => Tile::Start,
                            c => panic!("Unknown char {c}"),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Grid::from(input).max_distance())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Grid::from(input).inside_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_complex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_small_squeeze() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_larger() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
