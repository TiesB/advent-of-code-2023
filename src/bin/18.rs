advent_of_code::solution!(18);

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve(input: &str, use_color: bool) -> isize {
    let mut pos = (0, 0);
    let mut prev_pos = (0, 0);
    let mut area = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (d, n) = if !use_color {
            (
                DIRS[match parts[0] {
                    "R" => 0,
                    "D" => 1,
                    "L" => 2,
                    "U" => 3,
                    _ => panic!(),
                }],
                parts[1].parse::<isize>().unwrap(),
            )
        } else {
            (
                DIRS[usize::try_from(parts[2].chars().nth(7).unwrap().to_digit(16).unwrap())
                    .unwrap()],
                isize::from_str_radix(&parts[2][2..7], 16).unwrap(),
            )
        };

        pos.0 += d.0 * n;
        pos.1 += d.1 * n;

        area += prev_pos.0 * pos.1 - pos.0 * prev_pos.1 + n;
        prev_pos = pos;
    }
    area / 2 + 1
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
