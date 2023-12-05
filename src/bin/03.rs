use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

type Position = (usize, usize);
type OldPosition = (i32, i32);

type Input = Vec<Vec<char>>;

pub fn parse_input(input: String) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn old_parse_input(
    input: &Input,
) -> (HashMap<OldPosition, String>, HashMap<OldPosition, char>) {
    let mut res = (HashMap::new(), HashMap::new());
    for (y, line) in input.iter().enumerate() {
        let mut t = String::new();
        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                t.push(*c);
            } else {
                if !t.is_empty() {
                    res.0.insert(
                        (
                            i32::try_from(x - t.len()).unwrap(),
                            i32::try_from(y).unwrap(),
                        ),
                        t.clone(),
                    );
                    t.clear();
                }
                if *c != '.' {
                    res.1
                        .insert((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()), *c);
                }
            }
        }
        if !t.is_empty() {
            res.0.insert(
                (
                    i32::try_from(line.len() - t.len() - 1).unwrap(),
                    i32::try_from(y).unwrap(),
                ),
                t.clone(),
            );
            t.clear();
        }
    }
    res
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

pub fn part_one(input: &Input) -> Option<u32> {
    let parsed_input = old_parse_input(input);
    let mut res = 0;
    let ds: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    'outer: for (pos, part) in &parsed_input.0 {
        for i in 0..part.len() {
            let x = pos.0 + i32::try_from(i).unwrap();
            let y = pos.1;
            for d in &ds {
                if parsed_input.1.contains_key(&(x + d.0, y + d.1)) {
                    res += part.parse::<u32>().unwrap();
                    continue 'outer;
                }
            }
        }
    }
    Some(res)
}

// I'll need to optimize this one at some point
pub fn part_two(input: &Input) -> Option<u32> {
    let width = input[0].len();
    let height = input.len();

    let mut res = 0;

    let mut symbols: HashMap<Position, (char, Vec<u32>)> = HashMap::new();

    let mut value = 0;
    let mut nearby: HashSet<Position> = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        let mut x = 0;
        while x < width {
            while x < width && line[x].is_ascii_digit() {
                value = value * 10 + line[x].to_digit(10).unwrap();
                for (xx, yy) in neighbors(&(x, y)) {
                    if xx < width && yy < height {
                        let c = input[yy][xx];
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
                        .or_insert((input[position.1][position.0], vec![]))
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
        let result = part_one(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(467835));
    }
}
