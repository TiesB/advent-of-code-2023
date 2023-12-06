use roots::{find_roots_quadratic, Roots};

// This solution will be cleaned up and improved later
advent_of_code::solution!(6);

type Race = (usize, usize);

type Input = String;

pub fn parse_input(input: String) -> String {
    input
}

pub fn parse_input_part1(input: &str) -> Vec<Race> {
    let mut res = vec![];
    let mut lines = input.lines();
    for t in lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|p| p.parse::<usize>().unwrap())
    {
        res.push((t, 0));
    }
    for (i, d) in lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|p| p.parse::<usize>().unwrap())
        .enumerate()
    {
        res[i].1 = d
    }

    res
}

type Part2Data = f64;
pub fn parse_input_part2(input: &str) -> (Part2Data, Part2Data) {
    let mut lines = input.lines();
    let t_line = lines.next().unwrap();
    let d_line = lines.next().unwrap();
    let t = t_line.replace(' ', "")[5..].parse::<Part2Data>().unwrap();
    let d = d_line.replace(' ', "")[9..].parse::<Part2Data>().unwrap();
    (t, d)
}

fn find_a(max: usize, record: usize) -> usize {
    (max / record..max)
        .find(|t| (t * (max - t)) > record)
        .unwrap()
}

pub fn part_one(input: &Input) -> Option<usize> {
    Some(
        parse_input_part1(input)
            .iter()
            .map(|&(time, record)| time - 2 * find_a(time, record) + 1)
            .product(),
    )
}

pub fn part_two(input: &Input) -> Option<Part2Data> {
    let (time, record) = parse_input_part2(input);

    if let Roots::Two([a, b]) = find_roots_quadratic(1f64, -time, record) {
        return Some(b.ceil() - a.ceil());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(71503f64));
    }
}
