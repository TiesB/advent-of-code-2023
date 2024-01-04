use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{matrix_from_input, Apply, Direction, Position};

advent_of_code::solution!(23);

trait FromChar {
    fn from_char(c: char) -> Self;
}

impl FromChar for Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Vertical(-1),
            '>' => Direction::Horizontal(1),
            'v' => Direction::Vertical(1),
            '<' => Direction::Horizontal(-1),
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, false)
}

fn solve(input: &str, slide: bool) -> Option<usize> {
    let map = matrix_from_input(input);

    let start: Position = (
        0,
        map.iter()
            .next()
            .and_then(|row| row.iter().position(|c| *c == '.'))
            .unwrap(),
    );
    let end: Position = (
        map.rows - 1,
        map.iter()
            .last()
            .and_then(|row| row.iter().position(|c| *c == '.'))
            .unwrap(),
    );

    let mut edges: HashMap<Position, HashMap<Position, usize>> = HashMap::new();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(usize, Position, Position, Position)> =
        VecDeque::from([(0, start, start, start)]);

    while let Some((dist, pos, prev, origin)) = queue.pop_front() {
        if pos == end {
            edges.entry(origin).or_default().insert(pos, dist);
            continue;
        }

        if visited.contains(&pos) {
            edges.entry(origin).or_default().insert(pos, dist);
            edges.entry(pos).or_default().insert(origin, dist);
            continue;
        }

        let mut neighbour_paths = HashSet::new();
        for n in map.neighbours(pos, false) {
            let c = map[n];
            match c {
                '#' => (),
                '<' | '>' | 'v' | '^'
                    if slide && Direction::from_char(c).reverse().apply(pos) == n => {}
                _ => {
                    neighbour_paths.insert(n);
                }
            }
        }

        if neighbour_paths.len() >= 3 {
            // Is junction
            visited.insert(pos);
            edges.entry(origin).or_default().insert(pos, dist);
            edges.entry(pos).or_default().insert(origin, dist);

            for n in neighbour_paths {
                if prev != n {
                    queue.push_back((1, n, pos, pos));
                }
            }
        } else {
            for n in neighbour_paths {
                if prev != n {
                    queue.push_back((dist + 1, n, pos, origin));
                }
            }
        }
    }

    dfs(&edges, &start, &end, &mut HashSet::new())
}

fn dfs(
    edges: &HashMap<Position, HashMap<Position, usize>>,
    current: &Position,
    end: &Position,
    visited_junctions: &mut HashSet<Position>,
) -> Option<usize> {
    let mut longest: Option<usize> = None;

    if *current == *end {
        return Some(0);
    }

    visited_junctions.insert(*current);
    if let Some(current_edges) = edges.get(current) {
        for (next, cost) in current_edges {
            if !visited_junctions.contains(next) {
                let r = dfs(edges, next, end, visited_junctions).map(|res| *cost + res);
                longest = longest.max(r);
            }
        }
    }
    visited_junctions.remove(current);

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
