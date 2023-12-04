use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

type Card = (HashSet<u8>, HashSet<u8>);

type Input = Vec<Card>;

type Output1 = usize;
type Output2 = Output1;

fn number_of_wins(card: &Card) -> usize {
    card.0
        .iter()
        .filter(|winning_number| card.1.contains(winning_number))
        .collect_vec()
        .len()
}

fn parse_input(input: &str) -> Input {
    let r = Regex::new(r"(\d+)").unwrap();
    input
        .lines()
        .map(|line| line.split(&[':', '|']))
        .map(|mut parts| {
            parts.next();
            parts.map(|part| {
                r.find_iter(part)
                    .map(|cap| cap.as_str().parse::<u8>().unwrap())
                    .collect::<HashSet<u8>>()
            })
        })
        .map(|mut sets| (sets.nth(0).unwrap(), sets.nth(0).unwrap()))
        .collect()
}

fn solve1(input: &Input) -> Output1 {
    input
        .iter()
        .map(number_of_wins)
        .map(|wins| {
            if wins > 0 {
                2_usize.pow((wins - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

fn solve2(input: &Input) -> Output2 {
    let ns = input.iter().map(number_of_wins).collect_vec();
    let len = ns.len();
    let mut s: HashMap<usize, usize> = HashMap::new();
    let mut res = 0;
    for (i, n) in ns.iter().rev().enumerate() {
        let id = len - i - 1;
        let mut sum = 1;
        for d in 1..=*n {
            let cid = id + d;
            sum += s.get(&cid).unwrap();
        }
        s.insert(id, sum);
        res += sum;
    }
    res
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day04.txt"))?;
    let mut input_s = String::new();
    file.read_to_string(&mut input_s)?;

    println!("Starting parsing");
    let p_start = Instant::now();
    let input = parse_input(&input_s);
    let p_elapsed = p_start.elapsed();

    println!("Starting part 1");
    let s1_start = Instant::now();
    let s1 = solve1(&input);
    let s1_elapsed = s1_start.elapsed();

    println!("Starting part 2");
    let s2_start = Instant::now();
    let s2 = solve2(&input);
    let s2_elapsed = s2_start.elapsed();

    println!("Parsing took {:.2?}", p_elapsed);
    println!("Part 1({:.2?}): {}", s1_elapsed, s1);
    println!("Part 2({:.2?}): {}", s2_elapsed, s2);
    Ok(())
}
