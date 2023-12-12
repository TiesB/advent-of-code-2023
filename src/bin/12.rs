use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

fn check(
    input: &[char],
    mut reqs: &[usize],
    in_seq: bool,
    map: &mut HashMap<(Vec<char>, Vec<usize>, bool), usize>,
) -> usize {
    let key = (input.to_owned(), reqs.to_owned(), in_seq);

    if map.contains_key(&key) {
        return *map.get(&key).unwrap();
    }

    match (
        input.is_empty(),
        (reqs.is_empty() || (reqs.len() == 1 && reqs[0] == 0)),
    ) {
        (true, true) => {
            return 1;
        }
        (true, false) => return 0,
        _ => (),
    }
    if reqs.is_empty() {
        reqs = &[0];
    }
    let res = match (input[0], reqs.first().unwrap_or(&0), in_seq) {
        ('.', &0, false) => check(&input[1..], &reqs[1..], false, map),
        ('.', _, false) => check(&input[1..], reqs, false, map),
        ('?', _, _) => {
            check(&[&['#'], &input[1..]].concat(), reqs, in_seq, map)
                + check(&[&['.'], &input[1..]].concat(), reqs, in_seq, map)
        }
        ('#', &len, _) if len > 0 => check(
            &input[1..],
            &[&[len - 1], &reqs[1..]].concat(),
            len > 1,
            map,
        ),
        _ => 0,
    };
    map.insert(key, res);
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: &'static mut HashMap<(Vec<char>, Vec<usize>, bool), usize> = Box::leak(Box::default());
    Some(
        input
            .lines()
            .map(|line| {
                let parts = line.split_whitespace().collect_vec();
                let pattern: Vec<char> = parts[0].chars().collect();
                let reqs: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

                check(&pattern, &reqs, false, map)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: &'static mut HashMap<(Vec<char>, Vec<usize>, bool), usize> = Box::leak(Box::default());
    Some(
        input
            .lines()
            .map(|line| {
                let parts = line.split_whitespace().collect_vec();
                let pattern: Vec<char> = parts[0].chars().collect_vec();
                let repeated_pattern = [
                    &pattern[..],
                    &['?'],
                    &pattern[..],
                    &['?'],
                    &pattern[..],
                    &['?'],
                    &pattern[..],
                    &['?'],
                    &pattern[..],
                ]
                .concat();
                let reqs: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
                let repeated_reqs =
                    [&reqs[..], &reqs[..], &reqs[..], &reqs[..], &reqs[..]].concat();
                check(&repeated_pattern, &repeated_reqs, false, map)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
