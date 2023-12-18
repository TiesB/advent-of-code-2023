use advent_of_code::Position;
use pathfinding::directed::dijkstra::dijkstra;
use std::cmp::{max, min};

advent_of_code::solution!(17);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Horizontal(isize),
    Vertical(isize),
}

impl Direction {
    pub fn get_next(&self) -> Vec<Self> {
        match self {
            Direction::Horizontal(d) => vec![
                Direction::Vertical(-1),
                Direction::Vertical(1),
                Direction::Horizontal(*d),
            ],
            Direction::Vertical(d) => vec![
                Direction::Horizontal(-1),
                Direction::Horizontal(1),
                Direction::Vertical(*d),
            ],
        }
    }

    pub fn get_d(&self) -> (isize, isize) {
        match self {
            Direction::Horizontal(d) => (0, *d),
            Direction::Vertical(d) => (*d, 0),
        }
    }
}

type State = (Position, Direction, usize); // (position, dir, num_of_steps_in_dir)

type GridT = Vec<Vec<usize>>;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Grid {
    grid: GridT,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn successors_part_1(
        &self,
        ((y, x), dir, num_of_steps_in_dir): &State,
    ) -> Vec<(State, usize)> {
        let mut res = vec![];

        for next in dir.get_next() {
            let is_straight_on = next == *dir;
            if is_straight_on && num_of_steps_in_dir + 1 > 3 {
                continue;
            }

            let d = next.get_d();
            match (y.checked_add_signed(d.0), x.checked_add_signed(d.1)) {
                (Some(new_y), Some(new_x)) if new_x < self.width && new_y < self.height => res
                    .push((
                        (
                            (new_y, new_x),
                            next,
                            if is_straight_on {
                                num_of_steps_in_dir + 1
                            } else {
                                1
                            },
                        ),
                        self.grid[new_y][new_x],
                    )),
                _ => (),
            }
        }

        res
    }

    pub fn successors_part_2(
        &self,
        ((y, x), dir, num_of_steps_in_dir): &State,
    ) -> Vec<(State, usize)> {
        let mut res = vec![];

        for next in dir.get_next() {
            let is_straight_on = next == *dir;
            if is_straight_on && num_of_steps_in_dir + 1 > 10 {
                continue;
            }

            let d = next.get_d();
            if !is_straight_on || *num_of_steps_in_dir == 0 {
                match (y.checked_add_signed(d.0 * 4), x.checked_add_signed(d.1 * 4)) {
                    (Some(new_y), Some(new_x)) if new_x < self.width && new_y < self.height => {
                        let mut cost = 0;
                        for cy in min(*y, new_y)..=max(*y, new_y) {
                            for cx in min(*x, new_x)..=max(*x, new_x) {
                                if cy != *y || cx != *x {
                                    cost += self.grid[cy][cx];
                                }
                            }
                        }
                        res.push((((new_y, new_x), next, 4), cost))
                    }
                    _ => (),
                }
            } else {
                match (y.checked_add_signed(d.0), x.checked_add_signed(d.1)) {
                    (Some(new_y), Some(new_x)) if new_x < self.width && new_y < self.height => res
                        .push((
                            (
                                (new_y, new_x),
                                next,
                                if is_straight_on {
                                    num_of_steps_in_dir + 1
                                } else {
                                    1
                                },
                            ),
                            self.grid[new_y][new_x],
                        )),
                    _ => (),
                }
            }
        }

        res
    }

    pub fn success_part_1(&self, ((y, x), _, _): &State) -> bool {
        *y == self.grid.len() - 1 && *x == self.grid[0].len() - 1
    }

    pub fn success_part_2(&self, ((y, x), _, num_of_steps_in_dir): &State) -> bool {
        *num_of_steps_in_dir >= 4 && *y == self.grid.len() - 1 && *x == self.grid[0].len() - 1
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid: GridT = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect();
        let height = grid.len();
        assert!(height > 0);
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);

    Some(
        dijkstra(
            &((0, 0), Direction::Horizontal(1), 0),
            |state| grid.successors_part_1(state),
            |state| grid.success_part_1(state),
        )
        .unwrap()
        .1,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from(input);

    Some(
        dijkstra(
            &((0, 0), Direction::Horizontal(1), 0),
            |state| grid.successors_part_2(state),
            |state| grid.success_part_2(state),
        )
        .unwrap()
        .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
