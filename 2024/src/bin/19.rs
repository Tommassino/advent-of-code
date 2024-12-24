use std::collections::{HashMap, HashSet};
advent_of_code::solution!(19);

#[derive(Debug)]
struct Input {
    available_towels: Vec<String>,
    designs: Vec<String>,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let available_towels = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let designs = lines.skip(1).map(|s| s.to_string()).collect();
        Input {
            available_towels,
            designs,
        }
    }
}

impl Input {
    fn is_possible(&self, design: &str, possible_designs: &mut HashSet<String>) -> bool {
        if design.is_empty() {
            return true;
        }
        for towel in &self.available_towels {
            if design.starts_with(towel) {
                let new_design = design[towel.len()..].to_string();
                if self.is_possible(&new_design, possible_designs) {
                    possible_designs.insert(design.to_string());
                    return true;
                }
            }
        }
        false
    }

    fn possible_designs(&self) -> HashSet<String> {
        let mut original_designs = HashSet::new();
        let mut possible_designs = HashSet::new();
        for design in &self.designs {
            if self.is_possible(design, &mut possible_designs) {
                original_designs.insert(design.to_string());
            }
        }
        original_designs
    }

    fn count_possible(&self, design: &str, possible_count: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(count) = possible_count.get(design) {
            return *count;
        }
        let mut design_possibilities = 0;
        for towel in &self.available_towels {
            if design.starts_with(towel) {
                let new_design = design[towel.len()..].to_string();
                design_possibilities += self.count_possible(&new_design, possible_count);
            }
        }
        possible_count.insert(design.to_string(), design_possibilities);
        design_possibilities
    }

    fn possible_design_combinations(&self) -> HashMap<String, usize> {
        let mut original_designs = HashMap::new();
        let mut possible_designs = HashMap::new();
        for design in &self.designs {
            original_designs.insert(
                design.to_string(),
                self.count_possible(design, &mut possible_designs),
            );
        }
        original_designs
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let possible_designs = input.possible_designs();
    Some(possible_designs.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let possible_designs = input.possible_design_combinations();
    Some(possible_designs.values().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
