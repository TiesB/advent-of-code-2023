use std::{
    cmp::{max, min, Ordering},
    collections::{HashSet, VecDeque},
};

advent_of_code::solution!(22);

const DIM: usize = 10;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Block {
    ys: (usize, usize),
    xs: (usize, usize),
    zs: (usize, usize),
    dependants: Vec<usize>,
    depends_on: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
fn settle(input: &str, dim: usize) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    for line in input.lines() {
        let parts: Vec<usize> = line.split([',', '~']).map(|s| s.parse().unwrap()).collect();
        let (x1, y1, z1, x2, y2, z2) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
        blocks.push(Block {
            ys: (min(y1, y2), max(y1, y2)),
            xs: (min(x1, x2), max(x1, x2)),
            zs: (min(z1, z2), max(z1, z2)),
            dependants: Vec::new(),
            depends_on: Vec::new(),
        });
    }

    blocks.sort_by_key(|block| block.zs.0);

    let mut heights: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; dim]; dim];
    for id in 0..blocks.len() {
        let block = &blocks[id];

        let mut highest = 0;
        let mut supporting_blocks: HashSet<usize> = HashSet::new();

        for y in block.ys.0..=block.ys.1 {
            for x in block.xs.0..=block.xs.1 {
                if let Some((height, higher_block_id)) = heights[y][x] {
                    match (height).cmp(&highest) {
                        Ordering::Greater => {
                            supporting_blocks.clear();
                            highest = height;
                            supporting_blocks.insert(higher_block_id);
                        }
                        Ordering::Equal => {
                            supporting_blocks.insert(higher_block_id);
                        }
                        _ => (),
                    }
                }
            }
        }

        let z = highest + block.zs.0.abs_diff(block.zs.1) + 1;

        for y in block.ys.0..=block.ys.1 {
            for x in block.xs.0..=block.xs.1 {
                heights[y][x] = Some((z, id));
            }
        }

        for supporting_block_id in supporting_blocks {
            blocks[id].depends_on.push(supporting_block_id);
            blocks[supporting_block_id].dependants.push(id);
        }
    }

    blocks
}

fn solve_one(input: &str, dim: usize) -> usize {
    let blocks = settle(input, dim);

    blocks
        .iter()
        .filter(|block| {
            block
                .dependants
                .iter()
                .all(|d_id| blocks[*d_id].depends_on.len() > 1)
        })
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve_one(input, DIM))
}

fn total_dependants(blocks: &[Block], id: usize) -> usize {
    let mut removed = HashSet::new();
    let mut queue = VecDeque::new();

    removed.insert(id);
    queue.push_back(id);

    while let Some(id) = queue.pop_front() {
        for d_id in &blocks[id].dependants {
            let dependant = &blocks[*d_id];
            if dependant
                .depends_on
                .iter()
                .all(|do_id| removed.contains(do_id))
            {
                removed.insert(*d_id);
                queue.push_back(*d_id);
            }
        }
    }

    removed.len() - 1
}

fn solve_two(input: &str, dim: usize) -> usize {
    let blocks = settle(input, dim);
    (0..blocks.len())
        .map(|id| total_dependants(&blocks, id))
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_two(input, DIM))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two() {
        let result = solve_two(&advent_of_code::template::read_file("examples", DAY), 3);
        assert_eq!(result, 7);
    }
}
