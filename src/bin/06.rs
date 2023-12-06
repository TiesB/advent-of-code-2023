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

pub fn parse_input_part2(input: &str) -> (f64, f64) {
    let mut lines = input.lines();
    let t_line = lines.next().unwrap();
    let d_line = lines.next().unwrap();
    let t = t_line.replace(' ', "")[5..].parse::<f64>().unwrap();
    let d = d_line.replace(' ', "")[9..].parse::<f64>().unwrap();
    (t, d)
}

fn gen_ds(max: usize) -> Vec<usize> {
    (0..max).map(|t| t * max - t * t).collect()
}

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        parse_input_part1(input)
            .iter()
            .map(|&(time, record)| {
                // s = v * (T-t)
                // v = a * t
                // s = a * t * (T - t)
                // s = a * t * T - a * t * t

                // record = t * (T - t)
                // record = t * T - t * t
                // 0 = t^2 - T*t + record
                // record / (T-t) = t
                let mut n = 0;
                for d in gen_ds(time) {
                    if d > record {
                        n += 1;
                    }
                }
                n
            })
            .product(),
    )
}

pub fn part_two(input: &Input) -> Option<f64> {
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
