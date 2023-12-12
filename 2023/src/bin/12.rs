use itertools::Itertools;
advent_of_code::solution!(12);

#[derive(Debug)]
struct SpringCondition {
    record: Vec<char>,
    groups: Vec<usize>,
}

impl From<&str> for SpringCondition {
    fn from(value: &str) -> Self {
        let (record_s, checksum_s) = value.split_whitespace().collect_tuple().unwrap();
        let record = record_s.chars().collect_vec();
        let checksum = checksum_s
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_vec();
        Self {
            record,
            groups: checksum,
        }
    }
}

impl SpringCondition {
    fn expand(&self, factor: usize) -> SpringCondition {
        let mut record = self.record.clone();
        record.push('?');
        SpringCondition {
            record: record
                .iter()
                .cycle()
                .take(record.len() * factor - 1)
                .cloned()
                .collect_vec(),
            groups: self
                .groups
                .iter()
                .cycle()
                .take(self.groups.len() * factor)
                .cloned()
                .collect_vec(),
        }
    }
    fn combinations(&self) -> u64 {
        let mut memory = vec![vec![None; self.record.len() + 1]; self.groups.len() + 1];
        SpringCondition::_combinations(&self.record, &self.groups, &mut memory).unwrap_or(0)
    }

    fn _combinations(
        record: &[char],
        groups: &[usize],
        memory: &mut Vec<Vec<Option<u64>>>,
    ) -> Option<u64> {
        if let result @ Some(_) = memory[groups.len()][record.len()] {
            return result;
        }
        let outcome = match (record.iter().next(), groups.iter().next()) {
            (Some('.'), _) => SpringCondition::_combinations(&record[1..], groups, memory),
            (Some('#'), None) => Some(0),
            (Some('#'), Some(group_len)) => {
                SpringCondition::_try_place(*group_len, record, &groups[1..], memory)
            }
            (Some('?'), None) => SpringCondition::_combinations(&record[1..], groups, memory),
            (Some('?'), Some(group_len)) => {
                let broken_outcome =
                    SpringCondition::_try_place(*group_len, record, &groups[1..], memory)
                        .unwrap_or(0);
                let unbroken_outcome =
                    SpringCondition::_combinations(&record[1..], groups, memory).unwrap_or(0);
                Some(broken_outcome + unbroken_outcome)
            }
            (None, Some(_)) => Some(0),
            (None, None) => Some(1),
            _ => panic!("Unexpected condition"),
        };
        memory[groups.len()][record.len()] = outcome;
        outcome
    }

    fn _try_place(
        group_len: usize,
        record: &[char],
        groups: &[usize],
        memory: &mut Vec<Vec<Option<u64>>>,
    ) -> Option<u64> {
        if record.len() < group_len // cannot place as no space left
            || record.iter().take(group_len).any(|c| *c == '.') //cannot place as blocked by unbroken tiles
            || (record.len() > group_len && record[group_len] == '#')
        //cannot place next tile cannot be broken
        {
            Some(0)
        } else {
            //place the tiles
            let offset = (group_len + 1).min(record.len());
            SpringCondition::_combinations(&record[offset..], groups, memory)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let combinations = input
        .lines()
        .map(|line| SpringCondition::from(line).combinations())
        .sum();
    Some(combinations)
}

pub fn part_two(input: &str) -> Option<u64> {
    let combinations = input
        .lines()
        .map(|line| SpringCondition::from(line).expand(5).combinations())
        .sum();
    Some(combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_simple_schematic() {
        let schematic = SpringCondition::from("???.### 1,1,3");
        assert_eq!(schematic.combinations(), 1);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_part_two_simple_schematic() {
        let schematic = SpringCondition::from(".??..??...?##. 1,1,3").expand(5);
        assert_eq!(schematic.combinations(), 16384);
    }
}
