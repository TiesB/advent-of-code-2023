advent_of_code::solution!(9);

type I = Vec<Vec<isize>>;

fn parse_input(input: &str) -> I {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

fn next(ns: &Vec<isize>) -> isize {
    let ds: Vec<isize> = (1..ns.len()).map(|i| ns[i] - ns[i - 1]).collect();
    if ds.iter().all(|n| *n == 0) {
        0
    } else {
        ds[ds.len() - 1] + next(&ds)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        parse_input(input)
            .iter()
            .fold(0, |res, line| res + line[line.len() - 1] + next(line)),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(
        parse_input(input)
            .iter()
            .map(|line| line.iter().rev().collect())
            .fold(0, |res, line: Vec<&isize>| {
                res + *line[line.len() - 1] + next(&line.iter().map(|n| **n).collect())
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
