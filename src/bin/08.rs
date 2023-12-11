use num::integer::lcm;
use petgraph::graphmap::DiGraphMap;
use regex::Regex;

advent_of_code::solution!(8);

type D<'a> = DiGraphMap<&'a str, ()>;
type I<'a> = (Vec<char>, D<'a>);

fn parse_input(input: &str) -> I {
    let ins: Vec<char> = input.lines().next().unwrap().chars().collect();

    let re = Regex::new(r"(?<s>\w\w\w) = \((?<d1>\w\w\w), (?<d2>\w\w\w)\)").unwrap();

    let edges: Vec<(&str, &str)> = re.captures_iter(input).fold(Vec::new(), |mut e, cap| {
        let s = cap.name("s").unwrap().as_str();
        let d1 = cap.name("d1").unwrap().as_str();
        let d2 = cap.name("d2").unwrap().as_str();
        e.push((s, d1));
        e.push((s, d2));

        e
    });
    let g = DiGraphMap::<&str, ()>::from_edges(edges);
    (ins, g)
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse_input(input);

    let ins_len = parsed.0.len();
    let mut cur = "AAA";
    let mut i = 0;
    while cur != "ZZZ" {
        let ins = &parsed.0[i % ins_len];
        // let n = input.1.neighbors(cur);
        // println!("{:?} {:?}", cur, n.count());
        cur = if *ins == 'L' || parsed.1.neighbors(cur).count() == 1 {
            parsed.1.neighbors(cur).next()
        } else {
            parsed.1.neighbors(cur).nth(1)
        }
        .unwrap();
        // println!("{:?}", cur);
        i += 1;
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse_input(input);

    let ins_len = parsed.0.len();
    Some(
        parsed
            .1
            .nodes()
            .filter(|n| n.ends_with('A'))
            .map(|mut cur| {
                let mut i = 0;
                while !cur.ends_with('Z') {
                    let ins = &parsed.0[i % ins_len];
                    // let n = input.1.neighbors(cur);
                    // println!("{:?} {:?}", cur, n.count());
                    cur = if *ins == 'L' || parsed.1.neighbors(cur).count() == 1 {
                        parsed.1.neighbors(cur).next()
                    } else {
                        parsed.1.neighbors(cur).nth(1)
                    }
                    .unwrap();
                    // println!("{:?}", cur);
                    i += 1;
                }
                i
            })
            .fold(1, lcm),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
