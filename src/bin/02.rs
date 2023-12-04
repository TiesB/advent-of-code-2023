advent_of_code::solution!(2);

#[derive(Clone, Debug)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

type Game = Vec<Draw>;

type Input = Vec<Game>;

pub fn parse_input(input: String) -> Input {
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

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        input
            .iter()
            .enumerate()
            .map(|game| {
                if game
                    .1
                    .iter()
                    .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
                {
                    u32::try_from(game.0).unwrap() + 1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &Input) -> Option<u32> {
    Some(
        input
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
        let result = part_one(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse_input(advent_of_code::template::read_file(
            "examples", DAY,
        )));
        assert_eq!(result, Some(2286));
    }
}
