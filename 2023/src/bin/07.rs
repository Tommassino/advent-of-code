use std::collections::HashMap;
use std::marker::PhantomData;
advent_of_code::solution!(7);

struct PartOne;
struct PartTwo;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand<Part> {
    cards: [char; 5],
    bid: u32,
    phantom: PhantomData<Part>,
}

impl<Part> From<&str> for Hand<Part> {
    fn from(value: &str) -> Self {
        let (cards_str, bid_str) = value.split_once(' ').unwrap();
        let bid = bid_str.parse::<u32>().unwrap();
        let cards = cards_str.chars().collect::<Vec<char>>().try_into().unwrap();
        Self {
            cards,
            bid,
            phantom: PhantomData,
        }
    }
}
impl<Part> Hand<Part>
where
    Hand<Part>: CardValue,
{
    fn strength(&self) -> HandType {
        match self.suit_count() {
            (0, 5) => HandType::FiveOfAKind,
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPairs,
            (4, 2) => HandType::OnePair,
            (5, 1) => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }

    pub fn suits(&self) -> HashMap<char, u32> {
        let mut suits = HashMap::<char, u32>::new();
        self.cards.iter().for_each(|card| {
            if suits.contains_key(card) {
                *suits.get_mut(card).unwrap() += 1;
            } else {
                suits.insert(*card, 1);
            }
        });
        suits
    }
}

pub trait CardValue {
    fn card_values(&self) -> u32;
    fn suit_count(&self) -> (u32, u32);
}

impl CardValue for Hand<PartOne> {
    fn card_values(&self) -> u32 {
        let card_values = self.cards.map(|char| match char {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => char.to_digit(10).unwrap(),
        });
        card_values.iter().enumerate().map(|(idx, val)| val << (20 - idx * 4)).sum()
    }

    fn suit_count(&self) -> (u32, u32) {
        let suits = self.suits();
        let max_suit = *suits.values().max().unwrap();
        let suit_len = suits.len();
        (suit_len as u32, max_suit)
    }
}

impl CardValue for Hand<PartTwo> {
    fn card_values(&self) -> u32 {
        let card_values = self.cards.map(|char| match char {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => char.to_digit(10).unwrap(),
        });
        card_values.iter().enumerate().map(|(idx, val)| val << (20 - idx * 4)).sum()
    }

    fn suit_count(&self) -> (u32, u32) {
        let mut suits = self.suits();
        let jokers = suits.get(&'J').copied().unwrap_or(0);
        suits.remove(&'J');
        let max_suit = suits.values().max().copied().unwrap_or(0);
        let suit_len = suits.len();
        (suit_len as u32, max_suit + jokers)
    }
}

#[derive(Debug)]
struct CamelCards<Part> {
    hands: Vec<Hand<Part>>,
}

impl<Part> From<&str> for CamelCards<Part> {
    fn from(value: &str) -> Self {
        let hands = value.lines().map(Hand::from).collect();
        Self { hands }
    }
}

impl<Part> CamelCards<Part>
where
    Hand<Part>: CardValue,
{
    pub fn solve(&mut self) -> u32 {
        self.hands.sort_by_cached_key(|hand| {
            let card_values = hand.card_values();
            let strength = hand.strength();
            (strength, card_values)
        });
        self.hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
            .sum::<u32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = CamelCards::<PartOne>::from(input);
    Some(game.solve())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game = CamelCards::<PartTwo>::from(input);
    Some(game.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_5_of_a_kind_hands() {
        let hands = [
            Hand::<PartTwo>::from("AAAAA 1"),
            Hand::<PartTwo>::from("JJJJJ 1"),
            Hand::<PartTwo>::from("AAAAJ 1"),
            Hand::<PartTwo>::from("AAAJJ 1"),
            Hand::<PartTwo>::from("AAJJJ 1"),
            Hand::<PartTwo>::from("AJJJJ 1"),
        ];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::FiveOfAKind);
        })
    }

    #[test]
    fn test_4_of_a_kind_hands() {
        let hands = [
            Hand::<PartTwo>::from("AAAA1 1"),
            Hand::<PartTwo>::from("AAAJ1 1"),
            Hand::<PartTwo>::from("AAJJ1 1"),
            Hand::<PartTwo>::from("AJJJ1 1"),
        ];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::FourOfAKind);
        })
    }

    #[test]
    fn test_full_house_hands() {
        let hands = [
            Hand::<PartTwo>::from("AAAKK 1"),
            Hand::<PartTwo>::from("AAKKJ 1"),
        ];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::FullHouse);
        })
    }

    #[test]
    fn test_3_of_a_kind_hands() {
        let hands = [
            Hand::<PartTwo>::from("AAAKQ 1"),
            Hand::<PartTwo>::from("AAJ12 1"),
            Hand::<PartTwo>::from("AJJ12 1"),
        ];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::ThreeOfAKind);
        })
    }

    #[test]
    fn test_2_pairs_hands() {
        let hands = [Hand::<PartTwo>::from("AAKK1 1")];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::TwoPairs);
        })
    }

    #[test]
    fn test_pair_hands() {
        let hands = [
            Hand::<PartTwo>::from("AA123 1"),
            Hand::<PartTwo>::from("AJ123 1"),
        ];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::OnePair);
        })
    }

    #[test]
    fn test_high_card_hands() {
        let hands = [Hand::<PartTwo>::from("A1234 1")];
        hands.iter().for_each(|hand| {
            assert_eq!(hand.strength(), HandType::HighCard);
        })
    }

    #[test]
    fn test_compare() {
        let weaker = Hand::<PartTwo>::from("JKKK2 1").card_values();
        let stronger = Hand::<PartTwo>::from("QQQQ2 1").card_values();
        assert!(weaker < stronger);
    }
}
