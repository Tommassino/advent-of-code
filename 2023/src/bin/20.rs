use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
advent_of_code::solution!(20);

#[derive(PartialEq, Copy, Clone, Default, Debug, Hash, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Pulse {
    fn neg(&self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct FlipFlop {
    state: Pulse,
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    state: Pulse,
}

#[derive(Default, Clone, Debug)]
struct Broadcaster {
    state: Pulse,
}

trait Module: Debug {
    fn accepts(&self, _: Pulse) -> bool {
        true
    }
    fn visit(&mut self, pulse: Pulse, name: &str);
    fn get_state(&self) -> Pulse;
    fn register_input(&mut self, _: &str) {}

    fn reset(&mut self);
}

impl Module for FlipFlop {
    fn accepts(&self, pulse: Pulse) -> bool {
        match pulse {
            Pulse::Low => true,
            Pulse::High => false,
        }
    }
    fn visit(&mut self, pulse: Pulse, _: &str) {
        if pulse == Pulse::Low {
            self.state = self.state.neg();
        }
    }

    fn get_state(&self) -> Pulse {
        self.state
    }

    fn reset(&mut self) {
        self.state = Pulse::Low;
    }
}

impl Module for Conjunction {
    fn visit(&mut self, pulse: Pulse, name: &str) {
        self.inputs.insert(name.to_string(), pulse);
        if self.inputs.values().all(|&p| p == Pulse::High) {
            self.state = Pulse::Low;
        } else {
            self.state = Pulse::High;
        }
    }

    fn get_state(&self) -> Pulse {
        self.state
    }

    fn register_input(&mut self, name: &str) {
        self.inputs.insert(name.to_string(), Pulse::Low);
    }

    fn reset(&mut self) {
        self.inputs.iter_mut().for_each(|(_, pulse)| {
            *pulse = Pulse::Low;
        });
        self.state = Pulse::Low;
    }
}

impl Module for Broadcaster {
    fn visit(&mut self, pulse: Pulse, _: &str) {
        self.state = pulse;
    }

    fn get_state(&self) -> Pulse {
        self.state
    }

    fn reset(&mut self) {
        self.state = Pulse::Low;
    }
}

struct State {
    module_states: HashMap<String, Box<dyn Module>>,
    neighbors: HashMap<String, Vec<String>>,
    low_pulses: u64,
    high_pulses: u64,
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let mut module_states: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut neighbors = HashMap::<String, Vec<String>>::new();
        value.lines().for_each(|line| {
            let (source, destination) = line.split_once("->").unwrap();
            let (name, module): (String, Box<dyn Module>) = match source.split_at(1) {
                ("%", name) => {
                    let module = FlipFlop::default();
                    (name.trim().to_string(), Box::new(module))
                }
                ("&", name) => {
                    let module = Conjunction::default();
                    (name.trim().to_string(), Box::new(module))
                }
                (_, _) => {
                    let module = Broadcaster::default();
                    ("broadcaster".to_string(), Box::new(module))
                }
            };
            let destinations = destination
                .split(',')
                .map(|s| s.trim().to_string())
                .collect_vec();
            neighbors.insert(name.clone(), destinations.clone());
            module_states.insert(name.clone(), module);
        });
        neighbors.iter().for_each(|(name, destinations)| {
            destinations.iter().for_each(|destination| {
                module_states
                    .get_mut(destination)
                    .iter_mut()
                    .for_each(|module| {
                        module.register_input(name);
                    });
            });
        });
        State {
            module_states,
            neighbors,
            low_pulses: 0,
            high_pulses: 0,
        }
    }
}

struct TerminationCondition {
    source: String,
    pulse: Pulse,
}

impl State {
    fn reset(&mut self) {
        self.module_states.iter_mut().for_each(|(_, module)| {
            module.reset();
        });
        self.low_pulses = 0;
        self.high_pulses = 0;
    }
    fn button(&mut self, termination_condition: Option<TerminationCondition>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while let Some((source, pulse, destination)) = queue.pop_front() {
            match pulse {
                Pulse::Low => self.low_pulses += 1,
                Pulse::High => self.high_pulses += 1,
            }
            if let Some(termination_condition) = &termination_condition {
                if termination_condition.source == source && termination_condition.pulse == pulse {
                    return true;
                }
            }
            if destination == "output" || destination == "rx" {
                continue;
            }
            let module: &mut Box<dyn Module> = self.module_states.get_mut(&destination).unwrap();
            if !module.accepts(pulse) {
                continue;
            }
            module.visit(pulse, &source);
            for neighbor in self.neighbors.get(&destination).unwrap() {
                queue.push_back((destination.clone(), module.get_state(), neighbor.clone()));
            }
        }
        false
    }

    fn inputs_for(&self, name: &str) -> Vec<String> {
        self.neighbors
            .iter()
            .filter_map(|(source, neighbors)| {
                if neighbors.contains(&name.to_string()) {
                    Some(source.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut state = State::from(input);
    for _ in 0..1000 {
        state.button(None);
    }
    Some(state.high_pulses * state.low_pulses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut state = State::from(input);
    let rx_input = state.inputs_for("rx").first().cloned().unwrap();
    let cycles: Vec<u64> = state
        .inputs_for(&rx_input)
        .iter()
        .map(|conj_input| {
            state.reset();
            let mut iter = 1;
            while !state.button(Some(TerminationCondition {
                source: conj_input.clone(),
                pulse: Pulse::High,
            })) {
                iter += 1;
            }
            iter
        })
        .collect();
    let result = cycles.iter().fold(cycles[0], |acc, &cycle| lcm(acc, cycle));
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_part_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
