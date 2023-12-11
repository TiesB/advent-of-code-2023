use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

type Position = (usize, usize);

type Input = Vec<Vec<char>>;

pub fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn neighbors((x, y): &Position) -> HashSet<Position> {
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .iter()
    .map(|&(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);

    let width = parsed[0].len();
    let height = parsed.len();

    let mut value = "".to_string();
    let mut found = false;

    let mut res = 0;
    for y in 0..height {
        let mut x = 0;
        while x < width {
            while x < width && parsed[y][x].is_ascii_digit() {
                value.push(parsed[y][x]);
                if !found {
                    for (xx, yy) in neighbors(&(x, y)) {
                        if xx < width && yy < height {
                            let c = parsed[yy][xx];
                            if c != '.' && !c.is_ascii_digit() {
                                found = true;
                            }
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
    let parsed = parse_input(input);

    let width = parsed[0].len();
    let height = parsed.len();

    let mut res = 0;

    let mut symbols: HashMap<Position, (char, Vec<u32>)> = HashMap::new();

    let mut value = 0;
    let mut nearby: HashSet<Position> = HashSet::new();

    for (y, line) in parsed.iter().enumerate() {
        let mut x = 0;
        while x < width {
            while x < width && line[x].is_ascii_digit() {
                value = value * 10 + line[x].to_digit(10).unwrap();
                for (xx, yy) in neighbors(&(x, y)) {
                    if xx < width && yy < height {
                        let c = parsed[yy][xx];
                        if c != '.' && !c.is_ascii_digit() {
                            nearby.insert((xx, yy));
                        }
                    }
                }

                x += 1;
            }

            if !nearby.is_empty() {
                for position in &nearby {
                    symbols
                        .entry(*position)
                        .or_insert((parsed[position.1][position.0], vec![]))
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
