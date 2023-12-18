use std::collections::{HashMap, HashSet};

use advent_of_code::{matrix_from_input, Position};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = matrix_from_input(input);

    let mut value = "".to_string();
    let mut found = false;

    let mut res = 0;
    for y in 0..matrix.rows {
        let mut x = 0;
        while x < matrix.columns {
            while x < matrix.columns && matrix[(y, x)].is_ascii_digit() {
                value.push(matrix[(y, x)]);
                if !found {
                    for (yy, xx) in matrix.neighbours((y, x), true) {
                        let c = matrix[(yy, xx)];
                        if c != '.' && !c.is_ascii_digit() {
                            found = true;
                        }
                    }
                }

                x += 1;
            }

            if found {
                res += value.parse::<u32>().unwrap();
            }

            x += 1;
            value.clear();
            found = false;
        }
    }

    Some(res)
}

// I'll need to optimize this one at some point
pub fn part_two(input: &str) -> Option<u32> {
    let matrix = matrix_from_input(input);

    let mut res = 0;

    let mut symbols: HashMap<Position, (char, Vec<u32>)> = HashMap::new();

    let mut value = 0;
    let mut nearby: HashSet<Position> = HashSet::new();

    for (y, line) in matrix.iter().enumerate() {
        let mut x = 0;
        while x < matrix.columns {
            while x < matrix.columns && line[x].is_ascii_digit() {
                value = value * 10 + line[x].to_digit(10).unwrap();
                for (yy, xx) in matrix.neighbours((y, x), true) {
                    let c = matrix[(yy, xx)];
                    if c != '.' && !c.is_ascii_digit() {
                        nearby.insert((yy, xx));
                    }
                }

                x += 1;
            }

            if !nearby.is_empty() {
                for position in &nearby {
                    symbols
                        .entry(*position)
                        .or_insert((matrix[(position.0, position.1)], vec![]))
                        .1
                        .push(value);
                }
            }

            x += 1;
            value = 0;
            nearby.clear();
        }
    }

    for (_position, (c, values)) in symbols {
        if c == '*' && values.len() == 2 {
            res += values[0] * values[1];
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
