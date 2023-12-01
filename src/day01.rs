use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

type Input = Vec<String>;

type Output1 = u32;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.to_string()).collect()
}

fn line_to_n(line: Vec<char>) -> u32 {
    let digits: Vec<&char> = line.iter().filter(|c| c.is_digit(10)).collect();
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn solve1(input: &Input) -> Output1 {
    input
        .iter()
        .map(|line| line.chars().collect())
        .map(line_to_n)
        .sum()
}

fn solve2(input: &Input) -> Output2 {
    let dict = Vec::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    input
        .iter()
        .map(|line| {
            let mut lc = line.clone();
            for ele in &dict {
                lc = lc.replace(ele.0, ele.1)
            }
            lc.chars().collect()
        })
        .map(line_to_n)
        .sum()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day01.txt"))?;
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
