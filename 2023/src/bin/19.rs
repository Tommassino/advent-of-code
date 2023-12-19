use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(19);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(value.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    key: char,
    less_than: bool,
    value: u32,
    destination: Destination,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut parts = value.split(':');
        let (key_str, value_str) = parts.next().unwrap().split_at(1);
        let key = key_str.chars().next().unwrap();
        let less_than = value_str.starts_with('<');
        let value_str = value_str.trim_start_matches('<').trim_start_matches('>');
        let value = value_str.parse::<u32>().unwrap();
        let destination = parts.next().unwrap().into();
        Self {
            key,
            less_than,
            value,
            destination,
        }
    }
}

impl Rule {
    fn outcome(&self, part: &Part) -> Option<&Destination> {
        let value = match self.key {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid key"),
        };
        if self.less_than {
            if value < self.value {
                return Some(&self.destination);
            }
        } else if value > self.value {
            return Some(&self.destination);
        }
        None
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    final_destination: Destination,
}

impl Workflow {
    fn outcome(&self, part: &Part) -> &Destination {
        for rule in self.rules.iter() {
            if let Some(destination) = rule.outcome(part) {
                return destination;
            }
        }
        &self.final_destination
    }

    fn interval(&self, initial_part: &IntervalPart) -> Vec<(Destination, IntervalPart)> {
        let mut outcomes = Vec::new();
        let mut part = *initial_part;
        for rule in self.rules.iter() {
            let (passing_part, non_passing_part) = part.split(rule.key, rule.less_than, rule.value);
            if let Some(outcome_part) = passing_part {
                outcomes.push((rule.destination.clone(), outcome_part));
            }
            if let Some(next_part) = non_passing_part {
                part = next_part;
            }
        }
        outcomes.push((self.final_destination.clone(), part));
        outcomes
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, parts) = value.split_at(value.find('{').unwrap());
        let name = name.trim_end_matches('{');
        let parts: Vec<&str> = parts
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .collect();
        let final_destination = parts.last().cloned().unwrap().into();
        let mut rules: Vec<Rule> = Vec::new();
        for part in parts.iter().take(parts.len() - 1) {
            rules.push((*part).into());
        }
        Self {
            name: name.to_string(),
            rules,
            final_destination,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        let captures = re.captures(value).unwrap();
        Self {
            x: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            m: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            a: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            s: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let mut parts = value.split("\n\n");
        let workflows: HashMap<String, Workflow> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.into())
            .map(|workflow: Workflow| (workflow.name.clone(), workflow))
            .collect();
        let parts: Vec<Part> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.into())
            .collect();
        Self { workflows, parts }
    }
}

impl System {
    fn pass(&self, part: &Part) -> Destination {
        let mut location = "in";
        while let Some(workflow) = self.workflows.get(location) {
            let destination = workflow.outcome(part);
            match destination {
                Destination::Accept => {
                    return Destination::Accept;
                }
                Destination::Reject => {
                    return Destination::Reject;
                }
                Destination::Workflow(new_location) => {
                    location = &new_location;
                }
            }
        }
        Destination::Workflow(location.to_string())
    }

    fn accepted(&self) -> Vec<Part> {
        let mut accepted: Vec<Part> = Vec::new();
        for part in self.parts.iter() {
            if self.pass(part) == Destination::Accept {
                accepted.push(*part);
            }
        }
        accepted
    }

    fn accepted_intervals(&self, min: u32, max: u32) -> Vec<IntervalPart> {
        let mut accepted = Vec::new();
        let mut intervals = vec![("in".to_string(), IntervalPart::new(min, max))];
        while !intervals.is_empty() {
            let mut new_intervals = Vec::new();
            intervals.iter().for_each(|(workflow_id, part)| {
                let workflow = self.workflows.get(workflow_id).unwrap();
                workflow
                    .interval(part)
                    .iter()
                    .for_each(|(destination, part)| match destination {
                        Destination::Accept => {
                            accepted.push(*part);
                        }
                        Destination::Reject => {}
                        Destination::Workflow(workflow_id) => {
                            new_intervals.push((workflow_id.clone(), *part));
                        }
                    })
            });
            intervals = new_intervals;
        }
        accepted
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Interval {
    min: u32,
    max: u32,
}

impl Interval {
    fn clamp(&self, min: u32, max: u32) -> Option<Interval> {
        let n_min = self.min.max(min);
        let n_max = self.max.min(max);
        if n_min <= n_max {
            Some(Interval {
                min: n_min,
                max: n_max,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct IntervalPart {
    intervals: [Interval; 4],
}

impl IntervalPart {
    fn new(min: u32, max: u32) -> Self {
        Self {
            intervals: [
                Interval { min, max },
                Interval { min, max },
                Interval { min, max },
                Interval { min, max },
            ],
        }
    }

    fn clamp(&self, index: usize, less_than: bool, value: u32) -> Option<IntervalPart> {
        let mut part = *self;
        let clamped = match less_than {
            true => (value > 0)
                .then(|| part.intervals[index].clamp(part.intervals[index].min, value - 1))
                .flatten(),
            false => part.intervals[index].clamp(value + 1, part.intervals[index].max),
        };
        if let Some(clamped) = clamped {
            part.intervals[index] = clamped;
            Some(part)
        } else {
            None
        }
    }

    fn split(
        &self,
        key: char,
        less_than: bool,
        value: u32,
    ) -> (Option<IntervalPart>, Option<IntervalPart>) {
        let index = match key {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Invalid key"),
        };
        match less_than {
            true => (
                self.clamp(index, less_than, value),
                self.clamp(index, !less_than, value.saturating_sub(1)),
            ),
            false => (
                self.clamp(index, less_than, value),
                self.clamp(index, !less_than, value.saturating_add(1)),
            ),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let system: System = input.into();
    let accepted = system.accepted();
    accepted
        .iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let system: System = input.into();
    let accepted = system.accepted_intervals(1, 4000);
    // println!("{:?}", accepted);
    accepted
        .iter()
        .map(|x| {
            let mut total: u64 = 1;
            for interval in x.intervals.iter() {
                total *= (interval.max - interval.min + 1) as u64;
            }
            total
        })
        .sum::<u64>()
        .into()
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

    #[test]
    fn test_interval_clamp() {
        let interval = Interval { min: 0, max: 10 };
        let clamped = interval.clamp(5, 15).unwrap();
        assert_eq!(clamped.min, 5);
        assert_eq!(clamped.max, 10);
        let invalid_interval = clamped.clamp(15, 20);
        assert_eq!(invalid_interval, None);
    }

    #[test]
    fn test_interval_part_clamp() {
        let interval_part = IntervalPart::new(0, 10);
        let clamped = interval_part.clamp(0, true, 5).unwrap();
        assert_eq!(clamped.intervals[0].min, 0);
        assert_eq!(clamped.intervals[0].max, 4);
        let clamped = interval_part.clamp(0, false, 5).unwrap();
        assert_eq!(clamped.intervals[1].min, 6);
        assert_eq!(clamped.intervals[1].max, 10);
        let clamped = interval_part.clamp(1, true, 0);
        assert_eq!(clamped, None);
    }

    #[test]
    fn test_workflow_interval() {
        let workflow: Workflow = "test{x<5:A,x>9:R,A}".into();
        let interval_part = IntervalPart::new(0, 10);
        let interval = workflow.interval(&interval_part);
        assert_eq!(interval.len(), 3);
    }
}
