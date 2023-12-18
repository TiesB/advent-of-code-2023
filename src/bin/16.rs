use advent_of_code::Direction;
use rayon::prelude::*;

advent_of_code::solution!(16);

trait GetBd {
    fn get_bd(&self) -> u8;
}

impl GetBd for Direction {
    fn get_bd(&self) -> u8 {
        match self {
            Direction::Horizontal(1) => 0b1000,
            Direction::Vertical(1) => 0b0010,
            Direction::Horizontal(-1) => 0b0100,
            Direction::Vertical(-1) => 0b0001,
            _ => panic!(),
        }
    }
}

type Position = (isize, isize);

type Beam = (Position, Direction);

type Map = Vec<Vec<(char, u8)>>;

fn run(i_map: &Map, starting_beam: Beam) -> usize {
    let mut map = i_map.clone();
    let mut beams: Vec<Beam> = vec![starting_beam];
    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = vec![];
        for (pos, dir) in &mut beams {
            match dir {
                Direction::Horizontal(d) => {
                    pos.0 += *d;
                }
                Direction::Vertical(d) => {
                    pos.1 += *d;
                }
            }

            if pos.0 < 0
                || pos.0 >= map[0].len().try_into().unwrap()
                || pos.1 < 0
                || pos.1 >= map.len().try_into().unwrap()
            {
                continue;
            }

            let x = usize::try_from(pos.0).unwrap();
            let y = usize::try_from(pos.1).unwrap();

            let cur = map[y][x];

            if cur.1 & dir.get_bd() > 0 {
                continue;
            }

            map[y][x] = match cur {
                ('\\', b) => {
                    match dir {
                        Direction::Horizontal(d) => new_beams.push((*pos, Direction::Vertical(*d))),
                        Direction::Vertical(d) => new_beams.push((*pos, Direction::Horizontal(*d))),
                    }

                    ('\\', b | dir.get_bd())
                }
                ('/', b) => {
                    match dir {
                        Direction::Horizontal(d) => {
                            new_beams.push((*pos, Direction::Vertical(-*d)))
                        }
                        Direction::Vertical(d) => {
                            new_beams.push((*pos, Direction::Horizontal(-*d)))
                        }
                    }

                    ('/', b | dir.get_bd())
                }
                ('|', b) => {
                    if let Direction::Horizontal(_) = dir {
                        new_beams.push((*pos, Direction::Vertical(-1)));
                        new_beams.push((*pos, Direction::Vertical(1)));
                    } else {
                        new_beams.push((*pos, *dir));
                    }

                    ('|', b | dir.get_bd())
                }
                ('-', b) => {
                    if let Direction::Vertical(_) = dir {
                        new_beams.push((*pos, Direction::Horizontal(-1)));
                        new_beams.push((*pos, Direction::Horizontal(1)));
                    } else {
                        new_beams.push((*pos, *dir));
                    }

                    ('-', b | dir.get_bd())
                }
                (c, b) => {
                    new_beams.push((*pos, *dir));
                    (c, b | dir.get_bd())
                }
            }
        }
        beams = new_beams;
    }

    map.iter()
        .map(|row| row.iter().filter(|(_, energized)| *energized > 0).count())
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0b0000)).collect())
        .collect();

    Some(run(&map, ((-1, 0), Direction::Horizontal(1))))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0b0000)).collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let mut starting_beams: Vec<Beam> = vec![];

    for y in 0..height {
        starting_beams.push(((-1, y.try_into().unwrap()), Direction::Horizontal(1)));
        starting_beams.push((
            (width.try_into().unwrap(), y.try_into().unwrap()),
            Direction::Horizontal(-1),
        ));
    }
    for x in 0..width {
        starting_beams.push(((x.try_into().unwrap(), -1), Direction::Vertical(1)));
        starting_beams.push((
            (x.try_into().unwrap(), height.try_into().unwrap()),
            Direction::Vertical(-1),
        ));
    }

    starting_beams
        .par_iter()
        .map(|starting_beam| run(&map, *starting_beam))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
