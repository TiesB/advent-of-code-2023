use std::collections::HashMap;

advent_of_code::solution!(3);

type Position = (i32, i32);

type Input = (HashMap<Position, String>, HashMap<Position, char>);

pub fn parse_input(input: String) -> Input {
    let mut res = (HashMap::new(), HashMap::new());
    for (y, line) in input.lines().enumerate() {
        let mut t = String::new();
        for (x, c) in line.char_indices() {
            if c.is_ascii_digit() {
                t.push(c);
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
                if c != '.' {
                    res.1
                        .insert((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()), c);
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

pub fn part_one(input: &Input) -> Option<u32> {
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
    'outer: for (pos, part) in &input.0 {
        for i in 0..part.len() {
            let x = pos.0 + i32::try_from(i).unwrap();
            let y = pos.1;
            for d in &ds {
                if input.1.contains_key(&(x + d.0, y + d.1)) {
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
    for (cpos, c) in &input.1 {
        if c.ne(&'*') {
            continue;
        }

        let mut parts = vec![];
        'parts: for (ppos, part) in &input.0 {
            for d in &ds {
                for i in 0..part.len() {
                    if ppos.0 + i32::try_from(i).unwrap() == cpos.0 + d.0 && ppos.1 == cpos.1 + d.1
                    {
                        parts.push(part.clone());
                        continue 'parts;
                    }
                }
            }
        }
        if parts.len() == 2 {
            res += parts.first().unwrap().parse::<u32>().unwrap()
                * parts.get(1).unwrap().parse::<u32>().unwrap();
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
