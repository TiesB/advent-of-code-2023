use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

// I'd like a solution with less `.clone()`s
advent_of_code::solution!(19);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn get(&self, c: &str) -> usize {
        match c {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!(),
        }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct Ranges {
    x: Vec<Range<usize>>,
    m: Vec<Range<usize>>,
    a: Vec<Range<usize>>,
    s: Vec<Range<usize>>,
}

impl Ranges {
    pub fn empty() -> Ranges {
        Ranges {
            x: vec![],
            m: vec![],
            a: vec![],
            s: vec![],
        }
    }

    pub fn get(&self, c: &str) -> &Vec<Range<usize>> {
        match c {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!(),
        }
    }

    pub fn get_mut(&mut self, c: &str) -> &mut Vec<Range<usize>> {
        match c {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => panic!(),
        }
    }

    pub fn total(&self) -> usize {
        self.x.iter().map(|r| r.len()).sum::<usize>()
            * self.m.iter().map(|r| r.len()).sum::<usize>()
            * self.a.iter().map(|r| r.len()).sum::<usize>()
            * self.s.iter().map(|r| r.len()).sum::<usize>()
    }
}

#[derive(Debug)]
enum Op {
    True,
    Gt,
    Lt,
}

#[derive(Debug)]
struct Operation<'a> {
    target: &'a str,
    att: Option<&'a str>,
    op: Op,
    val: Option<usize>,
}

impl Operation<'_> {
    pub fn apply(&self, part: &Part) -> bool {
        match self.op {
            Op::True => true,
            Op::Gt => part.get(self.att.unwrap()) > self.val.unwrap(),
            Op::Lt => part.get(self.att.unwrap()) < self.val.unwrap(),
        }
    }

    pub fn apply_ranges(&self, ranges: &Ranges) -> (Ranges, Ranges) {
        match self.op {
            Op::True => (ranges.clone(), Ranges::empty()),
            Op::Gt => {
                let v = self.val.unwrap();
                let mut t = ranges.clone();
                let t_att = t.get_mut(self.att.unwrap());
                t_att.clear();
                let mut f = ranges.clone();
                let f_att = f.get_mut(self.att.unwrap());
                f_att.clear();

                for r in ranges.get(self.att.unwrap()) {
                    if r.contains(&v) {
                        t_att.push(v + 1..r.end);
                        f_att.push(r.start..v + 1);
                    } else {
                        f_att.push(r.clone());
                    }
                }

                (t, f)
            }
            Op::Lt => {
                let v = self.val.unwrap();
                let mut t = ranges.clone();
                let t_att = t.get_mut(self.att.unwrap());
                t_att.clear();
                let mut f = ranges.clone();
                let f_att = f.get_mut(self.att.unwrap());
                f_att.clear();

                for r in ranges.get(self.att.unwrap()) {
                    if r.contains(&v) {
                        t_att.push(r.start..v);
                        f_att.push(v..r.end);
                    } else {
                        f_att.push(r.clone());
                    }
                }

                (t, f)
            }
        }
    }
}

type Workflow<'a> = Vec<Operation<'a>>;

fn parse_operation(input: &str) -> Operation {
    if let Some((op, target)) = input.split_once(':') {
        if let Some((cat, val_s)) = op.split_once('>') {
            Operation {
                target,
                att: Some(cat),
                op: Op::Gt,
                val: Some(val_s.parse::<usize>().unwrap()),
            }
        } else if let Some((cat, val_s)) = op.split_once('<') {
            Operation {
                target,
                att: Some(cat),
                op: Op::Lt,
                val: Some(val_s.parse::<usize>().unwrap()),
            }
        } else {
            panic!()
        }
    } else {
        Operation {
            target: input,
            att: None,
            op: Op::True,
            val: None,
        }
    }
}

fn parse_workflow(input: &str) -> (&str, Workflow) {
    let (name, rest) = input.split_once('{').unwrap();
    (
        name,
        rest.split_terminator(&[',', '}'])
            .map(parse_operation)
            .collect(),
    )
}

fn parse_input(input: &str, part_2: bool) -> (HashMap<&str, Workflow>, Option<HashSet<Part>>) {
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
                        let categories: Vec<&str> = line[1..line.len() - 1].split(',').collect();
                        Part {
                            x: categories[0][2..].parse().unwrap(),
                            m: categories[1][2..].parse().unwrap(),
                            a: categories[2][2..].parse().unwrap(),
                            s: categories[3][2..].parse().unwrap(),
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
                let mut target = "in";
                while target != "A" && target != "R" {
                    if let Some(workflow) = workflows.get(target) {
                        for operation in workflow {
                            if operation.apply(part) {
                                target = operation.target;
                                break;
                            }
                        }
                    } else {
                        panic!()
                    }
                }
                if target == "A" {
                    part.sum()
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn num_accepted(workflows: &HashMap<&str, Workflow>, target: &str, ranges: &Ranges) -> usize {
    let mut sum = 0;

    if let Some(workflow) = workflows.get(target) {
        let mut remaining = ranges.clone();
        for operation in workflow {
            let (t, f) = operation.apply_ranges(&remaining);
            remaining = f;
            match operation.target {
                "A" => {
                    sum += t.total();
                }
                "R" => {}
                op_target => {
                    sum += num_accepted(workflows, op_target, &t);
                }
            }
        }
    } else {
        panic!()
    }

    sum
}

#[allow(clippy::single_range_in_vec_init)]
pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _) = parse_input(input, true);

    let start_ranges = Ranges {
        x: vec![1..4001],
        m: vec![1..4001],
        a: vec![1..4001],
        s: vec![1..4001],
    };

    Some(num_accepted(&workflows, "in", &start_ranges))
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
