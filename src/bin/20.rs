use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

trait Module {
    fn new(name: &str, outputs: &[String]) -> Self
    where
        Self: Sized;
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse>;
    fn add_input(&mut self, input: &str);
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    state: bool,
}
impl Module for FlipFlop {
    fn new(name: &str, outputs: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            outputs: outputs.to_vec(),
            state: false,
        }
    }

    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.state {
            return vec![];
        }

        self.state = !self.state;
        self.outputs
            .iter()
            .map(|dest| Pulse {
                source: self.name.to_owned(),
                target: dest.to_owned(),
                state: self.state,
            })
            .collect()
    }

    fn add_input(&mut self, _: &str) {}
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, bool>,
    outputs: Vec<String>,
}
impl Module for Conjunction {
    fn new(name: &str, outputs: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            inputs: HashMap::new(),
            outputs: outputs.to_vec(),
        }
    }

    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        *self.inputs.entry(pulse.source).or_insert(false) = pulse.state;

        let all_high = self.inputs.values().all(|state| *state);
        self.outputs
            .iter()
            .map(|dest| Pulse {
                source: self.name.to_owned(),
                target: dest.to_owned(),
                state: !all_high,
            })
            .collect()
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string(), false);
    }
}

#[derive(Debug)]
struct Broadcast {
    name: String,
    outputs: Vec<String>,
}
impl Module for Broadcast {
    fn new(name: &str, outputs: &[String]) -> Self {
        Self {
            name: name.to_owned(),
            outputs: outputs.to_vec(),
        }
    }

    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.outputs
            .iter()
            .map(|dest| Pulse {
                source: self.name.to_owned(),
                target: dest.to_owned(),
                state: pulse.state,
            })
            .collect()
    }

    fn add_input(&mut self, _: &str) {}
}

#[derive(Debug)]
struct Pulse {
    source: String,
    target: String,
    state: bool,
}

fn parse_outputs(outputs: &str) -> Vec<String> {
    outputs
        .split(',')
        .map(|output| output.trim().to_string())
        .collect()
}

fn parse_input(
    input: &str,
    output_module: Option<&str>,
) -> (HashMap<String, Box<dyn Module>>, Option<String>) {
    let mut gate = None;
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut reverse_mapping: HashMap<String, Vec<String>> = HashMap::new();
    let mut broadcaster: Broadcast;
    for line in input.lines() {
        let (module, outputs_s) = line.split_once(" ->").unwrap();
        if module == "broadcaster" {
            let outputs = parse_outputs(outputs_s);
            broadcaster = Broadcast::new(module, &outputs);
            modules.insert(module.to_string(), Box::new(broadcaster));
            for output in &outputs {
                if output_module.is_some_and(|g| g == output) {
                    gate = Some(module.to_string());
                }
                reverse_mapping
                    .entry(output.clone())
                    .or_default()
                    .push(module.to_string());
            }
        } else if let Some((_, name)) = module.split_once('&') {
            let outputs = parse_outputs(outputs_s);
            let conjunction = Conjunction::new(name, &outputs);
            modules.insert(name.to_string(), Box::new(conjunction));
            for output in &outputs {
                if output_module.is_some_and(|g| g == output) {
                    gate = Some(name.to_string());
                }
                reverse_mapping
                    .entry(output.clone())
                    .or_default()
                    .push(name.to_string());
            }
        } else if let Some((_, name)) = module.split_once('%') {
            let outputs = parse_outputs(outputs_s);
            let flipflop = FlipFlop::new(name, &outputs);
            modules.insert(name.to_string(), Box::new(flipflop));
            for output in &outputs {
                if output_module.is_some_and(|g| g == output) {
                    gate = Some(name.to_string());
                }
                reverse_mapping
                    .entry(output.clone())
                    .or_default()
                    .push(name.to_string());
            }
        }
    }

    for (id, module) in modules.iter_mut() {
        if let Some(inputs) = reverse_mapping.get(id) {
            for input in inputs {
                module.add_input(input);
            }
        }
    }

    (modules, gate)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut modules, _) = parse_input(input, None);

    let mut pulses: VecDeque<Pulse> = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        pulses.push_back(Pulse {
            source: "".to_string(),
            target: "broadcaster".to_string(),
            state: false,
        });
        low_count += 1;

        while let Some(pulse) = pulses.pop_front() {
            if let Some(target) = modules.get_mut(&pulse.target) {
                for new_pulse in target.receive(pulse) {
                    if new_pulse.state {
                        high_count += 1;
                    } else {
                        low_count += 1;
                    }
                    pulses.push_back(new_pulse);
                }
            }
        }
    }

    Some(low_count * high_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut modules, gate) = parse_input(input, Some("rx"));
    let mut pulses: VecDeque<Pulse> = VecDeque::new();

    let gate = gate.unwrap();
    let mut gate_sources: HashMap<String, Vec<usize>> = HashMap::new();

    let mut i = 0;

    // "More general" solution. Still would not work if the smallest cycle length is < (longest cycle length / 2)
    // while gate_sources.is_empty() || gate_sources.values().any(|pulses| pulses.len() < 2) {
    // Solution that works because we know specifics of the input
    while gate_sources.len() < 4 || gate_sources.values().any(|pulses| pulses.is_empty()) {
        i += 1;

        pulses.push_back(Pulse {
            source: "".to_string(),
            target: "broadcaster".to_string(),
            state: false,
        });

        while let Some(pulse) = pulses.pop_front() {
            if let Some(target) = modules.get_mut(&pulse.target) {
                for new_pulse in target.receive(pulse) {
                    if new_pulse.state && new_pulse.target == gate {
                        gate_sources
                            .entry(new_pulse.source.clone())
                            .or_default()
                            .push(i);
                    }

                    pulses.push_back(new_pulse);
                }
            }
        }
    }

    let mut res = 1;
    for source in gate_sources.values() {
        // Possible assertion for checking that cycle is consistent and has an offset of 0
        // assert_eq!(source[1] - 2 * source[0], 0);
        res = lcm(res, source[0]);
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }
}
