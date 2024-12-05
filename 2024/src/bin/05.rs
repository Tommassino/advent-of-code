advent_of_code::solution!(5);

struct Puzzle {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        let (rule_section, update_section) = value.split_once("\n\n").unwrap();
        for line in rule_section.lines() {
            let (x, y) = line.split_once("|").unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();
            rules.push((x, y));
        }
        for line in update_section.lines() {
            let update = line.split(",").map(|x| x.parse().unwrap()).collect();
            updates.push(update);
        }
        Puzzle { rules, updates }
    }
}

impl Puzzle {
    fn is_correct(&self, update: &[u32]) -> bool {
        for rule in self.rules.iter() {
            let (x, y) = rule;
            if update.contains(x) && update.contains(y) {
                let x_index = update.iter().position(|&z| z == *x).unwrap();
                let y_index = update.iter().position(|&z| z == *y).unwrap();
                if x_index > y_index {
                    return false;
                }
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from(input);
    let middle_sum = puzzle
        .updates
        .iter()
        .filter(|update| puzzle.is_correct(update))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(middle_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from(input);
    let incorrect_update_sum = puzzle
        .updates
        .iter()
        .filter(|update| !puzzle.is_correct(update))
        .map(|update| {
            let mut update = update.clone();
            let mut correct_index = 0;
            while correct_index < update.len() - 1 {
                let current_value = update[correct_index];
                let swap_values = puzzle
                    .rules
                    .iter()
                    .filter(|(_, z)| *z == current_value)
                    .map(|(z, _)| z);
                let mut found = false;
                for idx in (correct_index + 1)..update.len() {
                    let value = update[idx];
                    if swap_values.clone().any(|rule| *rule == value) {
                        update[idx] = current_value;
                        update[correct_index] = value;
                        found = true;
                        break;
                    }
                }
                if !found {
                    correct_index += 1;
                }
            }
            update[update.len() / 2]
        })
        .sum();
    Some(incorrect_update_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
