use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(8);

#[derive(Debug)]
struct Map {
    instructions: String,
    edges: HashMap<String, (String, String)>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let pattern = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
        let instructions = value.lines().next().unwrap().to_string();
        let edges = value
            .lines()
            .skip(2)
            .map(|line| {
                let captures = pattern.captures(line).unwrap();
                let key = captures[1].to_string();
                let left = captures[2].to_string();
                let right = captures[3].to_string();
                (key, (left, right))
            })
            .collect();
        Map {
            instructions,
            edges,
        }
    }
}

impl Map {
    fn walk(&self, start: &str) -> u32 {
        let mut position = start;
        let mut instruction_pointer = 0;
        let mut steps = 0;
        while !position.ends_with('Z') {
            steps += 1;
            let (left, right) = self.edges.get(position).unwrap();
            let instruction = self.instructions.chars().nth(instruction_pointer).unwrap();
            if instruction == 'L' {
                position = left;
            } else {
                position = right;
            }
            instruction_pointer = (instruction_pointer + 1) % self.instructions.len();
        }
        steps
    }

    fn walk_from(&self, starts: Vec<String>) -> u64 {
        let distances: Vec<u64> = starts.iter().map(|x| self.walk(x) as u64).collect();
        distances.iter().copied().reduce(lcm).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    Some(map.walk("AAA"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from(input);
    let starting_positions = map
        .edges
        .keys()
        .filter(|x| x.ends_with('A'))
        .cloned()
        .collect();
    Some(map.walk_from(starting_positions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_extended() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
