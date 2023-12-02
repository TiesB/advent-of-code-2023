use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

type Game = Vec<Draw>;

type Input = Vec<Game>;

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|game| {
                    let parts = game.split(", ");
                    let mut red: usize = 0;
                    let mut green: usize = 0;
                    let mut blue: usize = 0;
                    for part in parts {
                        let partparts = part.trim().split_once(' ').unwrap();
                        let d: usize = partparts.0.parse().unwrap();
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

fn solve1(input: &Input) -> Output1 {
    input
        .iter()
        .enumerate()
        .map(|game| {
            if game
                .1
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
            {
                game.0 + 1
            } else {
                0
            }
        })
        .sum()
}

fn solve2(input: &Input) -> Output2 {
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
        .sum()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day02.txt"))?;
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
