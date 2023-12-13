use std::cmp::min;

advent_of_code::solution!(13);

type Pattern = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn find_horizontal(pattern: &Pattern, num_error: usize) -> Option<usize> {
    let height = pattern.len();
    'outer: for y in 1..height {
        let mut error = 0;

        let max_d = min(y, height - y);

        for d in 0..max_d {
            let above = y.saturating_sub(d + 1);
            let below = y + d;

            for x in 0..pattern[above].len() {
                if pattern[above][x] != pattern[below][x] {
                    error += 1;

                    if error > num_error {
                        continue 'outer;
                    }
                }
            }
        }

        if error == num_error {
            return Some(y);
        }
    }

    None
}

fn find_vertical(pattern: &Pattern, num_error: usize) -> Option<usize> {
    let width = pattern[0].len();
    'outer: for x in 1..width {
        let mut error = 0;

        let max_d = min(x, width - x);

        for d in 0..max_d {
            let left = x.saturating_sub(d + 1);
            let right = x + d;

            for row in pattern {
                if row[left] != row[right] {
                    error += 1;

                    if error > num_error {
                        continue 'outer;
                    }
                }
            }
        }

        if error == num_error {
            return Some(x);
        }
    }

    None
}

fn solve(pattern: &Pattern, num_error: usize) -> usize {
    if let Some(horizontal) = find_horizontal(pattern, num_error) {
        return 100 * horizontal;
    }

    if let Some(vertical) = find_vertical(pattern, num_error) {
        return vertical;
    }

    panic!()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse(input).iter().map(|pattern| solve(pattern, 0)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(parse(input).iter().map(|pattern| solve(pattern, 1)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
