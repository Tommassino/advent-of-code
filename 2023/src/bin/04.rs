use std::collections::HashSet;
advent_of_code::solution!(4);

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: HashSet<u32>,
    scratch_numbers: HashSet<u32>,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (_, card_suffix) = value.split_once(": ").unwrap();
        let (winning_numbers_str, scratch_numbers_str) = card_suffix.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let scratch_numbers = scratch_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        ScratchCard {
            winning_numbers,
            scratch_numbers,
        }
    }
}

impl ScratchCard {
    pub fn win_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }

    pub fn value(&self) -> u32 {
        let win_count = self.win_count();
        if win_count > 0 {
            1 << (win_count - 1)
        } else {
            0
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let scratch_cards: Vec<ScratchCard> = input.lines().map(ScratchCard::from).collect();
    let result = scratch_cards.iter().map(|x| x.value()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let scratch_cards: Vec<ScratchCard> = input.lines().map(ScratchCard::from).collect();
    let mut card_counts = vec![1; scratch_cards.len()];
    scratch_cards.iter().enumerate().for_each(|(i, card)| {
        let value = card.win_count();
        let card_count = card_counts[i];
        for card_count_mut in card_counts.iter_mut().take(i + value + 1).skip(i + 1) {
            *card_count_mut += card_count;
        }
    });
    let result = card_counts.iter().sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
