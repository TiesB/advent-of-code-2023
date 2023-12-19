use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    pub fn get(&self, category: &Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Copy, Debug)]
struct Ranges {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl Ranges {
    pub fn get_mut(&mut self, category: &Category) -> &mut (usize, usize) {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    pub fn total(&self) -> usize {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Target<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}
impl<'a> From<&'a str> for Target<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Target::Accept,
            "R" => Target::Reject,
            workflow => Target::Workflow(workflow),
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    True(Target<'a>),
    Gt(Target<'a>, Category, usize),
    Lt(Target<'a>, Category, usize),
}

impl Operation<'_> {
    pub fn apply(&self, part: &Part) -> Option<&Target> {
        match self {
            Operation::True(target) => Some(target),
            Operation::Gt(target, category, value) => {
                if part.get(category) > *value {
                    Some(target)
                } else {
                    None
                }
            }
            Operation::Lt(target, category, value) => {
                if part.get(category) < *value {
                    Some(target)
                } else {
                    None
                }
            }
        }
    }
}

type Workflow<'a> = Vec<Operation<'a>>;

fn parse_operation(input: &str) -> Operation {
    if let Some((op, target)) = input.split_once(':') {
        if let Some((cat, val)) = op.split_once('>') {
            Operation::Gt(
                Target::from(target),
                Category::from(cat),
                val.parse().unwrap(),
            )
        } else if let Some((cat, val)) = op.split_once('<') {
            Operation::Lt(
                Target::from(target),
                Category::from(cat),
                val.parse().unwrap(),
            )
        } else {
            panic!()
        }
    } else {
        Operation::True(Target::from(input))
    }
}

fn parse_workflow(input: &str) -> (Target, Workflow) {
    let (name, rest) = input.split_once('{').unwrap();
    (
        Target::Workflow(name),
        rest.split_terminator(&[',', '}'])
            .map(parse_operation)
            .collect(),
    )
}

fn parse_input(input: &str, part_2: bool) -> (HashMap<Target, Workflow>, Option<HashSet<Part>>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    (
        workflows.lines().map(parse_workflow).collect(),
        if part_2 {
            None
        } else {
            Some(
                parts
                    .lines()
                    .map(|line| {
                        let categories: Vec<usize> = line
                            .split(|c: char| !c.is_numeric())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();
                        Part {
                            x: categories[0],
                            m: categories[1],
                            a: categories[2],
                            s: categories[3],
                        }
                    })
                    .collect(),
            )
        },
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows, parts) = parse_input(input, false);

    Some(
        parts
            .unwrap()
            .iter()
            .map(|part| {
                let mut target = &Target::Workflow("in");
                while let Some(workflow) = workflows.get(target) {
                    for operation in workflow {
                        if let Some(new_target) = operation.apply(part) {
                            target = new_target;
                            break;
                        }
                    }
                }
                if let Target::Accept = target {
                    part.sum()
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn num_accepted(
    workflows: &HashMap<Target, Workflow>,
    target: &Target,
    mut ranges: Ranges,
) -> usize {
    if let Target::Accept = target {
        return ranges.total();
    } else if let Target::Reject = target {
        return 0;
    }

    let mut sum = 0;

    if let Some(workflow) = workflows.get(target) {
        for operation in workflow {
            let mut new_ranges = ranges;
            match operation {
                Operation::True(new_target) => {
                    return sum + num_accepted(workflows, new_target, ranges)
                }
                Operation::Gt(new_target, category, value) => {
                    let range = ranges.get_mut(category);

                    if *value >= range.0 && *value < range.1 {
                        let new_range = new_ranges.get_mut(category);
                        new_range.0 = *value + 1;
                        sum += num_accepted(workflows, new_target, new_ranges);

                        range.1 = *value + 1;
                    }
                }
                Operation::Lt(new_target, category, value) => {
                    let range = ranges.get_mut(category);

                    if *value >= range.0 && *value < range.1 {
                        let new_range = new_ranges.get_mut(category);
                        new_range.1 = *value;
                        sum += num_accepted(workflows, new_target, new_ranges);

                        range.0 = *value;
                    }
                }
            }
        }
    }

    panic!()
}

#[allow(clippy::single_range_in_vec_init)]
pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _) = parse_input(input, true);

    let start_ranges = Ranges {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };

    Some(num_accepted(
        &workflows,
        &Target::Workflow("in"),
        start_ranges,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
