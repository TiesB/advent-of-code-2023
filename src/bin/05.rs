use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let mut parts = input.split("\n\n");
    let mut seeds: Vec<i64> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap())
        .collect();

    let maps: Vec<Vec<(i64, i64, i64)>> = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let ds: Vec<i64> = line
                        .split_whitespace()
                        .map(|d| d.parse().unwrap())
                        .collect();
                    (ds[0], ds[1], ds[2])
                })
                .collect()
        })
        .collect();

    for map in maps {
        seeds = seeds
            .iter()
            .map(|seed| {
                for mapping in &map {
                    let d = seed - mapping.1;

                    if d >= 0 && d < mapping.2 {
                        return mapping.0 + d;
                    }
                }
                *seed
            })
            .collect()
    }
    seeds.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut parts = input.split("\n\n");
    let mut seeds_ranges: Vec<Range<i64>> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .tuples()
        .map(|(start, length)| {
            let start = start.parse::<i64>().unwrap();
            let length = length.parse::<i64>().unwrap();
            start..start + length
        })
        .collect();

    let maps: Vec<Vec<(i64, i64, i64)>> = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let ds: Vec<i64> = line
                        .split_whitespace()
                        .map(|d| d.parse().unwrap())
                        .collect();
                    (ds[0], ds[1], ds[2])
                })
                .collect()
        })
        .collect();

    for map in &maps {
        let mut new_ranges: Vec<Range<i64>> = Vec::new();
        for mut r in seeds_ranges {
            for mapping in map {
                if !r.is_empty() {
                    let transformation = mapping.0 - mapping.1;
                    let mapping_range = mapping.1..mapping.1 + mapping.2;

                    // Case rr.start < mapping_range.start < mapping_range.end < rr.end is not covered, but this does not seem to matter for the puzzle input
                    if mapping_range.contains(&r.start) {
                        if mapping_range.contains(&r.end) {
                            new_ranges.push(r.start + transformation..r.end + transformation);
                            r = r.start..r.start;
                        } else {
                            new_ranges
                                .push(r.start + transformation..mapping_range.end + transformation);
                            r = mapping_range.end..r.end;
                        }
                    } else if mapping_range.contains(&r.end) {
                        new_ranges.push(mapping.0..r.end + transformation);
                        r = r.start..mapping.1;
                    }
                }
            }
            if !r.is_empty() {
                new_ranges.push(r);
            }
        }
        seeds_ranges = new_ranges;
    }

    seeds_ranges.iter().map(|r| r.start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
