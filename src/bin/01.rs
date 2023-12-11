advent_of_code::solution!(1);

fn line_to_n(line: &str) -> u32 {
    let mut digits = line.chars().filter(|c| c.is_ascii_digit());
    let first = digits.next().unwrap();
    first.to_digit(10).unwrap() * 10 + digits.last().unwrap_or(first).to_digit(10).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| acc + line_to_n(line)))
}

const DICT: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

pub fn part_two(input: &String) -> Option<u32> {
    let mut i = input.to_owned();

    for ele in &DICT {
        i = i.replace(ele.0, ele.1)
    }

    Some(input.lines().fold(0, |acc, line| acc + line_to_n(line)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
