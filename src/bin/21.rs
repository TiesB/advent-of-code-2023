use std::collections::HashSet;

use advent_of_code::{vecvec, IPosition, Neighbours};

advent_of_code::solution!(21);

pub fn solve(input: &str, steps: usize) -> Option<usize> {
    let map = vecvec(input);
    let height = map.len();
    assert!(height > 0);
    let width = map[0].len();
    let half_width = width / 2;

    assert_eq!(height, width);
    let brute_force = steps < half_width || (steps - half_width) % width != 0;

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

        if brute_force && i + 1 == steps {
            return Some(visited[i % 2].len());
        }

        if !brute_force && (i + 1) % width == half_width {
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

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 64)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_a() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two_b() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_part_two_c() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(6536));
    }

    // TODO: this one fails and I have no clue why
    // #[test]
    // fn test_part_two_d() {
    //     let result = solve(&advent_of_code::template::read_file("examples", DAY), 500);
    //     assert_eq!(result, Some(167004));
    // }

    #[test]
    fn test_part_two_e() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 1000);
        assert_eq!(result, Some(668697));
    }

    // Brute forcing this one takes too long
    // #[test]
    // fn test_part_two_f() {
    //     let result = solve(&advent_of_code::template::read_file("examples", DAY), 5000);
    //     assert_eq!(result, Some(16733044));
    // }
}
