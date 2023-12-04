advent_of_code::solution!(1);

type Input = Vec<String>;

pub fn parse_input(input: String) -> Input {
    input.lines().map(|line| line.to_string()).collect()
}

fn line_to_n(line: Vec<char>) -> u32 {
    let digits: Vec<&char> = line.iter().filter(|c| c.is_ascii_digit()).collect();
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|line| line.chars().collect())
            .map(line_to_n)
            .sum(),
    )
}

pub fn part_two(input: &Input) -> Option<u32> {
    let dict = Vec::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    Some(
        input
            .iter()
            .map(|line| {
                let mut lc = line.clone();
                for ele in &dict {
                    lc = lc.replace(ele.0, ele.1)
                }
                lc.chars().collect()
            })
            .map(line_to_n)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&parse_input(advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        )));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse_input(advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        )));
        assert_eq!(result, Some(281));
    }
}
