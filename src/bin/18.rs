advent_of_code::solution!(18);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Horizontal(isize),
    Vertical(isize),
}

impl Direction {
    pub fn get_d(&self) -> (isize, isize) {
        match self {
            Direction::Horizontal(d) => (*d, 0),
            Direction::Vertical(d) => (0, *d),
        }
    }
}

type Instruction = (Direction, isize);

fn parse_line(line: &str, use_color: bool) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if !use_color {
        let i = parts[0].chars().collect::<Vec<char>>()[0];
        let n = parts[1].parse::<isize>().unwrap();
        let dir = match i {
            'U' => Direction::Vertical(-1),
            'D' => Direction::Vertical(1),
            'L' => Direction::Horizontal(-1),
            'R' => Direction::Horizontal(1),
            _ => panic!(),
        };
        (dir, n)
    } else {
        let cn = isize::from_str_radix(&parts[2][2..7], 16).unwrap();
        let cd = match parts[2].chars().nth(7).unwrap().to_digit(16).unwrap() {
            3 => Direction::Vertical(-1),
            1 => Direction::Vertical(1),
            2 => Direction::Horizontal(-1),
            0 => Direction::Horizontal(1),
            _ => panic!(),
        };

        (cd, cn)
    }
}

fn parse_lines(input: &str, use_color: bool) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| parse_line(line, use_color))
        .collect()
}

fn solve(instructions: &Vec<Instruction>) -> isize {
    let mut pos = (0, 0);
    let mut prev_pos = (0, 0);
    let mut area = 0;
    for (dir, n) in instructions {
        let d = dir.get_d();

        pos.0 += d.0 * n;
        pos.1 += d.1 * n;

        area += prev_pos.0 * pos.1 - pos.0 * prev_pos.1 + n;
        prev_pos = pos;
    }
    area / 2 + 1
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve(&parse_lines(input, false)))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve(&parse_lines(input, true)))
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
