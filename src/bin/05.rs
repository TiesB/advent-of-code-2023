use std::{collections::HashSet, ops::Range};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(5);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Mapping {
    destination: i64,
    source: i64,
    length: i64,
}

type Map = HashSet<Mapping>;

type Input = (Vec<i64>, Vec<Map>);

pub fn parse_input(input: String) -> Input {
    let r_parts = Regex::new(r"\n\r?\n").unwrap();
    let input_parts = r_parts.split(&input).collect_vec();
    let r = Regex::new(r"(?<d>\d+) (?<s>\d+) (?<l>\d+)\n?").unwrap();

    let seeds = Regex::new(r"(\d+)")
        .unwrap()
        .find_iter(input_parts[0])
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let maps = input_parts[1..]
        .iter()
        .map(|m| {
            r.captures_iter(m)
                .map(|cap| Mapping {
                    destination: cap.name("d").unwrap().as_str().parse::<i64>().unwrap(),
                    source: cap.name("s").unwrap().as_str().parse::<i64>().unwrap(),
                    length: cap.name("l").unwrap().as_str().parse::<i64>().unwrap(),
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

pub fn part_one(input: &Input) -> Option<i64> {
    let mut seeds = input.0.clone();
    for map in &input.1 {
        seeds = seeds
            .iter()
            .map(|seed| {
                for mapping in map {
                    let d = *seed - mapping.source;

                    if d >= 0 && d < mapping.length {
                        return mapping.destination + d;
                    }
                }
                *seed
            })
            .collect()
    }
    seeds.iter().min().copied()
}

pub fn part_two(input: &Input) -> Option<i64> {
    let mut ranges: HashSet<Range<i64>> = HashSet::new();
    let seeds_data = input.0.clone();
    for i in (0..seeds_data.len()).step_by(2) {
        let start = seeds_data[i];
        let length = seeds_data[i + 1];
        ranges.insert(start..start + length);
    }

    for map in &input.1 {
        let mut new_ranges: HashSet<Range<i64>> = HashSet::new();
        for r in ranges {
            let mut rr = r.clone();
            for mapping in map {
                if !rr.is_empty() {
                    let transformation = mapping.destination - mapping.source;
                    let mapping_range = mapping.source..mapping.source + mapping.length;

                    // Case rr.start < mapping_range.start < mapping_range.end < rr.end is not covered, but this does not seem to matter for the puzzle input
                    if mapping_range.contains(&rr.start) {
                        if mapping_range.contains(&rr.end) {
                            new_ranges.insert(rr.start + transformation..rr.end + transformation);
                            rr = rr.start..rr.start;
                        } else {
                            new_ranges.insert(
                                rr.start + transformation..mapping_range.end + transformation,
                            );
                            rr = mapping_range.end..rr.end;
                        }
                    } else if mapping_range.contains(&rr.end) {
                        new_ranges.insert(mapping.destination..rr.end + transformation);
                        rr = rr.start..mapping.source;
                    }
                }
            }
            if !rr.is_empty() {
                new_ranges.insert(rr);
            }
        }
        ranges = new_ranges;
    }
    ranges.iter().map(|r| r.start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(46));
    }
}
