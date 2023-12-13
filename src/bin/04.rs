use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

type Card = (HashSet<u8>, HashSet<u8>);

type Input = Vec<Card>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split(&[':', '|']))
        .map(|parts| {
            parts.skip(1).map(|part| {
                part.split_whitespace()
                    .map(|d| d.parse().unwrap())
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
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .par_lines()
            .map(|line| line.split(&[':', '|']))
            .map(|parts| {
                parts.skip(1).map(|part| {
                    part.split_whitespace()
                        .map(|d| d.parse().unwrap())
                        .collect::<HashSet<u8>>()
                })
            })
            .map(|mut sets| (sets.next().unwrap(), sets.next().unwrap()))
            .map(|card| number_of_wins(&card))
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

    Some(
        ns.iter()
            .rev()
            .enumerate()
            .map(|(i, n)| {
                let id = len - i - 1;
                let mut sum = 1;
                for d in 1..=*n {
                    let cid = id + d;
                    sum += s.get(&cid).unwrap();
                }
                s.insert(id, sum);
                sum
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
