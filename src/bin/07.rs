use std::collections::HashMap;

advent_of_code::solution!(7);

const CHARS_1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CHARS_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn parse_input(i: &str) -> Vec<(&str, HashMap<char, u8>, usize)> {
    i.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let hand_s = parts[0];

            let hand = hand_s.chars().fold(HashMap::new(), |mut map, c| {
                map.entry(c).and_modify(|n| *n += 1).or_insert(1);
                map
            });

            (hand_s, hand, parts[1].parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let hands = parse_input(input);

    let mut x: Vec<(&str, usize, usize)> = hands
        .iter()
        .map(|(s, hand, bid)| {
            let mut score: usize = 0;

            for (i, c) in s.chars().rev().enumerate() {
                score += 100usize.pow(u32::try_from(i).unwrap())
                    * (CHARS_1.iter().position(|&r| r == c).unwrap() + 2)
            }
            // println!("{} {}", s, score);

            let mut pairs = 0;
            let mut has_three = false;
            let mut has_four = false;
            let mut has_five = false;

            for c in CHARS_1 {
                if hand.contains_key(&c) {
                    let n = hand.get(&c).unwrap();

                    match n {
                        2 => {
                            pairs += 1;
                        }
                        3 => {
                            has_three = true;
                        }
                        4 => {
                            has_four = true;
                        }
                        5 => {
                            has_five = true;
                        }
                        _ => (),
                    }
                }
            }

            if pairs == 1 && !has_three {
                score += 10000000000;
            }
            if pairs == 2 {
                score += 2 * 10000000000;
            }
            if pairs == 0 && has_three {
                // Three of a kind
                score += 3 * 10000000000;
            }
            if pairs == 1 && has_three {
                // Full house
                score += 4 * 10000000000;
            }
            if has_four {
                score += 5 * 10000000000;
            }
            if has_five {
                score += 6 * 10000000000;
            }

            (*s, score, *bid)
        })
        .collect();
    x.sort_by(|a, b| a.1.cmp(&b.1));
    let res = x
        .iter()
        .enumerate()
        .fold(0usize, |total, (i, (_s, _score, bid))| {
            total + (i + 1) * bid
        }); // decreasing order
            // println!("{:?}", x);
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let hands = parse_input(input);

    let mut x: Vec<(&str, u64, usize)> = hands
        .iter()
        .map(|(s, hand, bid)| {
            let mut score: u64 = 0;

            for (i, c) in s.chars().rev().enumerate() {
                score += 100u64.pow(u32::try_from(i).unwrap())
                    * u64::try_from(CHARS_2.iter().position(|&r| r == c).unwrap() + 1).unwrap()
            }

            let mut jokers = 0;
            let mut pairs = 0;
            let mut has_three = false;
            let mut has_four = false;
            let mut has_five = false;

            for c in CHARS_2 {
                if hand.contains_key(&c) {
                    let n = hand.get(&c).unwrap();

                    if c == 'J' {
                        jokers = *n;
                    } else {
                        match n {
                            2 => {
                                pairs += 1;
                            }
                            3 => {
                                has_three = true;
                            }
                            4 => {
                                has_four = true;
                            }
                            5 => {
                                has_five = true;
                            }
                            _ => (),
                        }
                    }
                }
            }

            if has_five
                || (has_four && jokers >= 1)
                || (has_three && jokers >= 2)
                || (pairs >= 1 && jokers >= 3)
                || jokers >= 4
            {
                // Five of a kind
                score += 6 * 10000000000;
            } else if has_four
                || (has_three && jokers >= 1)
                || (pairs >= 1 && jokers == 2)
                || jokers >= 3
            {
                // Four of a kind
                score += 5 * 10000000000;
            } else if (pairs == 1 && has_three)
                || (pairs >= 2 && jokers >= 1)
                || (pairs == 1 && jokers >= 2)
            {
                // Full house
                score += 4 * 10000000000;
            } else if (pairs == 0 && has_three) || (pairs == 1 && jokers == 1) || jokers == 2 {
                // Three of a kind
                score += 3 * 10000000000;
            } else if pairs == 2 || (pairs == 1 && jokers == 1) {
                // Two pair
                score += 2 * 10000000000;
            } else if pairs == 1 && !has_three || jokers == 1 {
                // One pair
                score += 10000000000;
            }

            (*s, score, *bid)
        })
        .collect();

    x.sort_by(|a, b| a.1.cmp(&b.1));

    Some(
        x.iter()
            .enumerate()
            .fold(0usize, |total, (i, (_s, _score, bid))| {
                total + (i + 1) * bid
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
