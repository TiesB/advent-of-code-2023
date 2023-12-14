use indexmap::IndexMap;
use itertools::Itertools;

advent_of_code::solution!(15);

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut h, c| {
        let d = c as usize;
        h += d;
        h *= 17;
        h %= 256;
        h
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let line = input.lines().collect_vec()[0];

    Some(line.split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let line = input.lines().collect_vec()[0];

    let mut boxes: Vec<IndexMap<&str, usize>> = vec![IndexMap::new(); 256];

    for part in line.split(',') {
        if part.ends_with('-') {
            let label = &part[0..&part.len() - 1];

            boxes[hash(label)].shift_remove(&label);
        } else {
            let (label, lens) = part.split_once('=').unwrap();

            boxes[hash(label)].insert(label, lens.parse::<usize>().unwrap());
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.1)
                    .sum::<usize>()
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
