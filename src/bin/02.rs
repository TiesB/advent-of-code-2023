advent_of_code::solution!(2);

#[derive(Clone, Debug)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

type Game = Vec<Draw>;

type Input = Vec<Game>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|game| {
                    let parts = game.split(", ");
                    let mut red: u32 = 0;
                    let mut green: u32 = 0;
                    let mut blue: u32 = 0;
                    for part in parts {
                        let partparts = part.trim().split_once(' ').unwrap();
                        let d: u32 = partparts.0.parse().unwrap();
                        match partparts.1 {
                            "red" => red = d,
                            "green" => green = d,
                            "blue" => blue = d,
                            _ => panic!(),
                        }
                    }
                    Draw { red, green, blue }
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .iter()
            .enumerate()
            .map(|(id, game)| {
                if game
                    .iter()
                    .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
                {
                    id + 1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|game| {
                game.iter().fold(
                    Draw {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |mut acc, draw| {
                        if draw.red > acc.red {
                            acc.red = draw.red;
                        }
                        if draw.green > acc.green {
                            acc.green = draw.green;
                        }
                        if draw.blue > acc.blue {
                            acc.blue = draw.blue;
                        }
                        acc
                    },
                )
            })
            .map(|d| d.red * d.green * d.blue)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
