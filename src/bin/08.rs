use num::integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let ins: Vec<char> = input.lines().next().unwrap().chars().collect();
    let ins_len = ins.len();

    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| (&line[0..=2], (&line[7..=9], &line[12..=14])))
        .collect();

    let mut cur = "AAA";
    let mut i = 0;
    while cur != "ZZZ" {
        let ins = ins[i % ins_len];

        cur = if ins == 'L' {
            map.get(cur).unwrap().0
        } else {
            map.get(cur).unwrap().1
        };

        i += 1;
    }

    Some(i)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ins: Vec<char> = input.lines().next().unwrap().chars().collect();
    let ins_len = ins.len();

    let mut starts: Vec<&str> = Vec::new();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| {
            if line[0..=2].ends_with('A') {
                starts.push(&line[0..=2]);
            }

            (&line[0..=2], (&line[7..=9], &line[12..=14]))
        })
        .collect();
    Some(
        starts
            .par_iter()
            .map(|mut cur| {
                let mut i = 0;
                while !cur.ends_with('Z') {
                    let ins = ins[i % ins_len];

                    cur = if ins == 'L' {
                        &map.get(cur).unwrap().0
                    } else {
                        &map.get(cur).unwrap().1
                    };

                    i += 1;
                }
                i
            })
            .reduce(|| 1, lcm),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
