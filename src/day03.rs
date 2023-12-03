use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

type Position = (i32, i32);

type Input = (HashMap<Position, String>, HashMap<Position, char>);

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    let mut res = (HashMap::new(), HashMap::new());
    for (y, line) in input.lines().enumerate() {
        let mut t = String::new();
        for (x, c) in line.char_indices() {
            if c.is_digit(10) {
                t.push(c);
            } else {
                if t.len() > 0 {
                    res.0.insert(
                        (
                            i32::try_from(x - t.len()).unwrap(),
                            i32::try_from(y).unwrap(),
                        ),
                        t.clone(),
                    );
                    t.clear();
                }
                if c != '.' {
                    res.1
                        .insert((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()), c);
                }
            }
        }
        if t.len() > 0 {
            res.0.insert(
                (
                    i32::try_from(line.len() - t.len() - 1).unwrap(),
                    i32::try_from(y).unwrap(),
                ),
                t.clone(),
            );
            t.clear();
        }
    }
    res
}

fn solve1(input: &Input) -> Output1 {
    let mut res = 0;
    let ds: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    'outer: for (pos, part) in &input.0 {
        for i in 0..part.len() {
            let x = pos.0 + i32::try_from(i).unwrap();
            let y = pos.1;
            for d in &ds {
                if input.1.contains_key(&(x + d.0, y + d.1)) {
                    res += part.parse::<usize>().unwrap();
                    continue 'outer;
                }
            }
        }
    }
    res
}

/**
 * Not too proud of this one
 */
fn solve2(input: &Input) -> Output2 {
    let mut res = 0;
    let ds: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    for (cpos, c) in &input.1 {
        if c.ne(&'*') {
            continue;
        }

        let mut parts = vec![];
        'parts: for (ppos, part) in &input.0 {
            for d in &ds {
                for i in 0..part.len() {
                    if ppos.0 + i32::try_from(i).unwrap() == cpos.0 + d.0 && ppos.1 == cpos.1 + d.1
                    {
                        parts.push(part.clone());
                        continue 'parts;
                    }
                }
            }
        }
        if parts.len() == 2 {
            res += parts.get(0).unwrap().parse::<usize>().unwrap()
                * parts.get(1).unwrap().parse::<usize>().unwrap();
        }
    }
    res
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day03.txt"))?;
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
