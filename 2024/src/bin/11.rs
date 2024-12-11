use std::collections::HashMap;
advent_of_code::solution!(11);

#[derive(Debug)]
struct Input {
    stones: Vec<u64>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let stones = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { stones }
    }
}

impl Input {
    fn blink_many(&self, count: u64) -> u64 {
        let mut memory = HashMap::new();
        self.stones
            .iter()
            .map(|s| Self::blink_stone(*s, count, &mut memory))
            .sum()
    }

    fn blink_stone(stone: u64, count: u64, memory: &mut HashMap<(u64, u64), u64>) -> u64 {
        if count == 0 {
            return 1;
        }
        if memory.contains_key(&(stone, count)) {
            return *memory.get(&(stone, count)).unwrap();
        }
        let mut next = Vec::new();
        if stone == 0 {
            next.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let s = stone.to_string();
            let half = s.len() / 2;
            let left = s[..half].parse().unwrap();
            let right = s[half..].parse().unwrap();
            next.push(left);
            next.push(right);
        } else {
            next.push(stone * 2024);
        }
        let result = next
            .iter()
            .map(|s| Self::blink_stone(*s, count - 1, memory))
            .sum();
        memory.insert((stone, count), result);
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = Input::from(input);
    Some(input.blink_many(25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = Input::from(input);
    Some(input.blink_many(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }
}
