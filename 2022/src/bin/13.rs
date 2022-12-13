extern crate core;

use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone, Eq)]
pub enum Signal{
    Single(u8),
    Multiple(Vec<Signal>)
}


impl FromStr for Signal{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len()-1];
        let mut stack: Vec<Vec<Signal>> = vec![];
        let mut multiple: Vec<Signal> = vec![];
        let mut single: Option<u8> = None;

        s.bytes().for_each(|c| {
            match c {
                b'0'..=b'9' => {
                    // append to current single number
                    single = Some(single
                        .map(|curr| curr * 10 + c - b'0')
                        .unwrap_or(c - b'0')
                    );
                }
                b',' => {
                    // finalize a number if there is one in progress
                    single.iter()
                        .for_each(|x| multiple.push(Signal::Single(*x)));
                    single = None;
                }
                b'[' => {
                    // finalize a number if there is one in progress
                    single.iter()
                        .for_each(|x| multiple.push(Signal::Single(*x)));
                    single = None;
                    // push the current signal to the top of the stack
                    stack.push(multiple.clone());
                    // start a new nested signal
                    multiple = vec![];
                }
                b']' => {
                    // finalize a number if there is one in progress
                    single.iter()
                        .for_each(|x| multiple.push(Signal::Single(*x)));
                    single = None;
                    // push the current in-progress signal to the one at the top of the stack
                    let signal = Signal::Multiple(multiple.clone());
                    multiple = stack.pop().unwrap();
                    multiple.push(signal);
                }
                _ => panic!("Unexpected char {}", c)
            }
        });

        single.iter()
            .for_each(|x| multiple.push(Signal::Single(*x)));
        Ok(Signal::Multiple(multiple))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Single(left), Signal::Single(right)) => left.cmp(right),
            (Signal::Multiple(left), Signal::Multiple(right)) => left.cmp(right),
            (left @ Signal::Single(_), Signal::Multiple(right)) => {
                let left_list = std::slice::from_ref(left);
                left_list.cmp(right.as_slice())
            },
            (Signal::Multiple(left), right @ Signal::Single(_)) =>{
                let right_list = std::slice::from_ref(right);
                left.as_slice().cmp(right_list)
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data: Vec<(Signal, Signal)> = input.split("\n\n").map(|pair| {
        let signal_pair = pair.lines()
            .map(|line| Signal::from_str(line).expect(""))
            .next_tuple()
            .unwrap();
        signal_pair
    }).collect();

    let result: usize = data.iter().enumerate()
        .flat_map(|(idx, (first, second))| {
            if first <= second {
                Some(idx+1)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut data: Vec<Signal> = input.lines().flat_map(|line| {
        if line.is_empty() {
            None
        } else {
            Signal::from_str(line).ok()
        }
    }).collect();
    data.push(Signal::Single(2));
    data.push(Signal::Single(6));
    data.sort();
    let result = data.iter().enumerate()
        .filter(|(_, x)| {
            matches!(x, Signal::Single(2) | Signal::Single(6))
        })
        .map(|x| x.0 + 1)
        .product();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13, None);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13, None);
        assert_eq!(part_two(&input), Some(140));
    }
}
