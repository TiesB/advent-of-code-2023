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

        let lower = 0;
        let upper = min(y, height - y);

        if lower >= upper {
            continue;
        }
        for i in lower..upper {
            let l = y.saturating_sub(i + 1);
            let r = y + i;
            for xx in 0..pattern[l].len() {
                if pattern[l][xx] != pattern[r][xx] {
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

        let lower = 0;
        let upper = min(x, width - x);

        if lower >= upper {
            continue;
        }
        for i in lower..upper {
            let l = x.saturating_sub(i + 1);
            let r = x + i;
            for row in pattern {
                if row[l] != row[r] {
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
