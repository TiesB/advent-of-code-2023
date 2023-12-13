use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(12);

fn check(
    input: &[char],
    reqs: &[usize],
    i: usize,
    ri: usize,
    current_len: usize,
    map: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let key = (i, ri, current_len);

    if let Some(res) = map.get(&key) {
        return *res;
    }

    match (
        i == input.len(),
        ((ri == reqs.len() && current_len == 0)
            || (ri == reqs.len() - 1 && reqs[ri] == current_len)),
    ) {
        (true, true) => {
            return 1;
        }
        (true, false) => {
            return 0;
        }
        _ => (),
    }

    let res = match (input[i], *reqs.get(ri).unwrap_or(&0), current_len) {
        ('.', r, cur) if r > 0 && r == cur => check(input, reqs, i + 1, ri + 1, 0, map),
        ('.', _, 0) => check(input, reqs, i + 1, ri, 0, map),
        ('#', r, _) if r > 0 => check(input, reqs, i + 1, ri, current_len + 1, map),
        ('?', r, c) => {
            // This should be refactored
            (if r > 0 {
                check(input, reqs, i + 1, ri, current_len + 1, map)
            } else {
                0
            }) + (if r > 0 && r == c {
                check(input, reqs, i + 1, ri + 1, 0, map)
            } else {
                0
            }) + (if c == 0 {
                check(input, reqs, i + 1, ri, 0, map)
            } else {
                0
            })
        }
        _ => 0,
    };

    map.insert(key, res);

    res
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .par_lines()
            .map(|line| {
                let parts = line.split_whitespace().collect_vec();
                let pattern: Vec<char> = parts[0].chars().collect();
                let reqs: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

                check(&pattern, &reqs, 0, 0, 0, &mut HashMap::new())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .par_lines()
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
                check(
                    &repeated_pattern,
                    &repeated_reqs,
                    0,
                    0,
                    0,
                    &mut HashMap::new(),
                )
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
        assert_eq!(result, Some(525152));
    }
}
