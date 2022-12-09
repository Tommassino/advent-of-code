extern crate core;

use log::debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Move {
    fn is_winner(&self, other: &Move) -> bool {
        ((*other as u8) + 1) % 3 == (*self as u8)
    }
}

impl From<char> for Move {
    fn from(x: char) -> Self {
        match x {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Unknown move {}", x)
        }
    }
}

impl From<i8> for Move {
    fn from(x: i8) -> Self {
        match x {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            -1 => Move::Scissors,
            _ => panic!("Unknown move {}", x)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed: Vec<(Move, Move)> = input.lines()
        .map(
            |x|
                (Move::from(x.chars().next().unwrap()), Move::from(x.chars().last().unwrap()))
        ).collect();

    let score = parsed.iter()
        .map(
            |(first, second)| {
                if first == second {
                    3 + (*second as u32) + 1
                } else if second.is_winner(first) {
                    6 + (*second as u32) + 1
                } else {
                    (*second as u32) + 1
                }
            }
        ).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed: Vec<(Move, Move)> = input.lines()
        .map(
            |x| {
                let first_move = Move::from(x.chars().next().unwrap());
                let outcome = x.chars().last().unwrap();
                let second_move =
                    match outcome {
                        'X' => Move::from((first_move as i8 - 1) % 3),
                        'Y' => first_move,
                        'Z' => Move::from((first_move as i8 + 1) % 3),
                        _ => panic!("Unknown outcome {}", outcome)
                    };
                (first_move, second_move)
            }
        ).collect();

    debug!("Parsed input maybe {:?}", parsed);
    let score = parsed.iter()
        .map(
            |(first, second)| {
                if first == second {
                    3 + (*second as u32) + 1
                } else if second.is_winner(first) {
                    6 + (*second as u32) + 1
                } else {
                    (*second as u32) + 1
                }
            }
        ).sum();
    Some(score)
}

fn main() {
    env_logger::init();
    let input = &advent_of_code::read_file("inputs", 2, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2, None);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2, None);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_move_winner(){
        assert_eq!(Move::Rock.is_winner(&Move::Paper), false);
        assert_eq!(Move::Rock.is_winner(&Move::Scissors), true);
        assert_eq!(Move::Paper.is_winner(&Move::Scissors), false);
        assert_eq!(Move::Paper.is_winner(&Move::Rock), true);
        assert_eq!(Move::Scissors.is_winner(&Move::Rock), false);
        assert_eq!(Move::Scissors.is_winner(&Move::Paper), true);
    }
}
