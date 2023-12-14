use indexmap::IndexMap;

advent_of_code::solution!(15);

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut h, c| {
        h += c as usize;
        h *= 17;
        h %= 256;
        h
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .fold(vec![IndexMap::new(); 256], |mut boxes, step| {
                if step.ends_with('-') {
                    let label = &step[0..&step.len() - 1];

                    boxes[hash(label)].shift_remove(&label);
                } else {
                    let (label, lens) = step.split_once('=').unwrap();

                    boxes[hash(label)].insert(label, lens.parse::<usize>().unwrap());
                }

                boxes
            })
            .iter()
            .enumerate()
            .map(|(box_i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_i, lens)| (box_i + 1) * (lens_i + 1) * lens.1)
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
