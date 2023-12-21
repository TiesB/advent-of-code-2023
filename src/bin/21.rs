use std::collections::HashSet;

use advent_of_code::{vecvec, IPosition, Neighbours, Position};

advent_of_code::solution!(21);

fn solve_one(input: &str, steps: usize) -> Option<usize> {
    let map = vecvec(input);

    let start: Position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|c| *c == 'S').map(|x| (y, x)))
        .unwrap();

    let mut frontier: HashSet<Position> = HashSet::from([start]);
    let mut new_frontier: HashSet<Position> = HashSet::new();

    let mut visited = vec![HashSet::new(), HashSet::new()];

    for i in 0..steps {
        for pos in frontier.drain() {
            for (y, x) in pos.neighbours() {
                if map[y][x] != '#' && visited[(i + 1) % 2].insert((y, x)) {
                    new_frontier.insert((y, x));
                }
            }
        }
        std::mem::swap(&mut frontier, &mut new_frontier);
    }

    Some(visited[steps % 2].len())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_one(input, 64)
}

pub fn solve_two(input: &str, steps: usize) -> Option<usize> {
    let map = vecvec(input);
    let height = map.len();
    assert!(height > 0);
    let width = map[0].len();
    let half_width = width / 2;

    assert_eq!(height, width);
    assert!((steps - 65) % width == 0);

    let start: IPosition = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|c| *c == 'S')
                .map(|x| (y as i32, x as i32))
        })
        .unwrap();

    let mut frontier: HashSet<IPosition> = HashSet::from([start]);
    let mut new_frontier: HashSet<IPosition> = HashSet::new();

    let mut visited: [HashSet<IPosition>; 2] = [HashSet::new(), HashSet::new()];

    let mut cycles: Vec<usize> = Vec::new();

    for i in 0..steps {
        for pos in frontier.drain() {
            for (y, x) in pos.neighbours() {
                if map[y.rem_euclid(width as i32) as usize][x.rem_euclid(width as i32) as usize]
                    != '#'
                    && visited[i % 2].insert((y, x))
                {
                    new_frontier.insert((y, x));
                }
            }
        }

        if i % width == half_width - 1 {
            cycles.push(visited[i % 2].len());
            if cycles.len() == 3 {
                break;
            }
        }

        std::mem::swap(&mut frontier, &mut new_frontier);
    }

    let times = steps / width;

    Some(
        cycles[0]
            + times * (cycles[1] - cycles[0])
            + (cycles[0] + cycles[2] - 2 * cycles[1]) * (times * (times - 1)) / 2,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_two(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }
}
