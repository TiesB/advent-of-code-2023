use advent_of_code::{vecvec, Position};
use std::{
    cmp::{max, min},
    collections::HashSet,
};

advent_of_code::solution!(11);

type Row = Vec<char>;
type Universe = Vec<Row>;

fn expand(universe: &Universe, expansion: usize) -> Universe {
    let height = universe.len();
    let width = universe[0].len();
    let mut expanded = universe.clone();

    let empty_row: Row = (0..width).map(|_| '.').collect();

    let mut y_inserted = 0;
    for (y, row) in universe.iter().enumerate() {
        if (0..width).all(|x| row[x] == '.') {
            for _i in 0..expansion - 1 {
                expanded.insert(y + y_inserted, empty_row.clone());
                y_inserted += 1;
            }
        }
    }
    let mut x_inserted = 0;
    for x in 0..width {
        if (0..height).all(|y| universe[y][x] == '.') {
            for _i in 0..expansion - 1 {
                for row in &mut expanded {
                    row.insert(x + x_inserted, '.');
                }
                x_inserted += 1;
            }
        }
    }

    expanded
}

fn solve(universe: &Universe, expansion: usize) -> usize {
    let expanded = expand(universe, expansion);

    let mut galaxies: Vec<Position> = vec![];
    let mut sum = 0;
    for (y, row) in expanded.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                for g in &galaxies {
                    sum += g.0.abs_diff(x) + g.1.abs_diff(y);
                }
                galaxies.push((x, y));
            }
        }
    }

    sum
}

fn solve2(universe: &Universe, expansion: usize) -> usize {
    let mut galaxies: Vec<Position> = vec![];
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut empty_columns: HashSet<usize> = HashSet::new();

    for x in 0..universe[0].len() {
        let mut is_empty = true;
        for row in universe {
            if row[x] != '.' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_columns.insert(x);
        }
    }

    let mut sum = 0;
    for (y, row) in universe.iter().enumerate() {
        let mut is_empty = true;
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                is_empty = false;

                for g in &galaxies {
                    let num_empty_columns = empty_columns
                        .iter()
                        .filter(|&&xx| xx > min(g.0, x) && xx < max(g.0, x))
                        .count();
                    let num_empty_rows = empty_rows
                        .iter()
                        .filter(|&&yy| yy > min(g.1, y) && yy < max(g.1, y))
                        .count();
                    sum += g.0.abs_diff(x)
                        + g.1.abs_diff(y)
                        + (num_empty_columns * (expansion - 1))
                        + (num_empty_rows * (expansion - 1));
                }
                galaxies.push((x, y));
            }
        }
        if is_empty {
            empty_rows.insert(y);
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(&vecvec(input), 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve2(&vecvec(input), 1000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let universe = vecvec(&advent_of_code::template::read_file("examples", DAY));
        let result = solve(&universe, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_solve2() {
        let universe = vecvec(&advent_of_code::template::read_file("examples", DAY));
        let result = solve(&universe, 100);
        assert_eq!(result, 8410);
    }
}
