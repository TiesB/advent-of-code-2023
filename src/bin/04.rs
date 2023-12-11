use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code::solution!(4);

type Card = (HashSet<u8>, HashSet<u8>);

type Input = Vec<Card>;

pub fn parse_input(input: &str) -> Input {
    let r = Regex::new(r"(\d+)").unwrap();
    input
        .lines()
        .map(|line| line.split(&[':', '|']))
        .map(|mut parts| {
            parts.next();
            parts.map(|part| {
                r.find_iter(part)
                    .map(|cap| cap.as_str().parse::<u8>().unwrap())
                    .collect::<HashSet<u8>>()
            })
        })
        .map(|mut sets| (sets.next().unwrap(), sets.next().unwrap()))
        .collect()
}

fn number_of_wins(card: &Card) -> usize {
    card.0
        .iter()
        .filter(|winning_number| card.1.contains(winning_number))
        .collect_vec()
        .len()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(number_of_wins)
            .map(|wins| {
                if wins > 0 {
                    2_u32.pow((wins - 1).try_into().unwrap())
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let ns = parse_input(input).iter().map(number_of_wins).collect_vec();
    let len = ns.len();
    let mut s: HashMap<usize, u32> = HashMap::new();
    let mut res = 0;
    for (i, n) in ns.iter().rev().enumerate() {
        let id = len - i - 1;
        let mut sum = 1;
        for d in 1..=*n {
            let cid = id + d;
            sum += s.get(&cid).unwrap();
        }
        s.insert(id, sum);
        res += sum;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
