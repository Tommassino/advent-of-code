use std::collections::HashMap;
advent_of_code::solution!(22);

#[derive(Debug)]
struct Input {
    secret_numbers: Vec<SecretNumber>,
}

impl Input {
    fn best_sequence(&self, num: usize) -> i32 {
        let mut scores = HashMap::new();
        for number in &self.secret_numbers {
            let sequence_scores = number.sequence_scores(num);
            for (sequence, score) in sequence_scores {
                let count = scores.entry(sequence).or_insert(0);
                *count += score;
            }
        }
        *scores.values().max().unwrap()
    }
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let secret_numbers = input
            .lines()
            .map(|line| SecretNumber::new(line.parse().unwrap()))
            .collect();
        Self { secret_numbers }
    }
}

#[derive(Debug, Copy, Clone)]
struct SecretNumber {
    value: u64,
}

impl SecretNumber {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn prune(&mut self) {
        self.value %= 16777216;
    }

    fn step(&mut self) {
        self.value = (self.value << 6) ^ self.value;
        self.prune();
        self.value = (self.value >> 5) ^ self.value;
        self.prune();
        self.value = (self.value << 11) ^ self.value;
        self.prune();
    }

    fn nth(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn secret_price(&self) -> i32 {
        (self.value % 10) as i32
    }

    fn price_changes(&self, num: usize) -> Vec<(i32, i32)> {
        let mut current_number = *self;
        let mut changes = Vec::new();
        for _ in 0..num {
            let last_price = current_number.secret_price();
            current_number.step();
            let current_price = current_number.secret_price();
            changes.push((current_price, current_price - last_price));
        }
        changes
    }

    fn sequence_scores(&self, num: usize) -> HashMap<(i32, i32, i32, i32), i32> {
        let mut scores = HashMap::new();
        let changes = self.price_changes(num);
        for i in 0..changes.len() - 3 {
            let sequence = (
                changes[i].1,
                changes[i + 1].1,
                changes[i + 2].1,
                changes[i + 3].1,
            );
            let score = changes[i + 3].0;
            // if the sequence is already in the hashmap, we can skip it
            if scores.contains_key(&sequence) {
                continue;
            }
            scores.insert(sequence, score);
        }
        scores
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = Input::from(input);
    let result = input
        .secret_numbers
        .iter_mut()
        .map(|secret_number| {
            secret_number.nth(2000);
            secret_number.value
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::from(input);
    let result = input.best_sequence(2000);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_secret_step() {
        let mut number = SecretNumber::new(123);
        println!("{:?}", number.price_changes(10));
    }

    #[test]
    fn test_1_2_3_2024() {
        let input = Input::from("1\n2\n3\n2024");
        assert_eq!(input.best_sequence(2000), 23);
    }
}
