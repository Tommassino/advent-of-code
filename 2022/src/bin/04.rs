use std::cmp::{max, min};
use regex::Regex;

pub fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    let pattern = Regex::new("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    let output: Vec<((u32, u32), (u32, u32))> = input.lines().map(|line| {
        let captures = pattern.captures(line).unwrap();
        let first_left = captures.get(1)
            .map(|x| x.as_str().parse::<u32>().unwrap()).unwrap();
        let first_right = captures.get(2)
            .map(|x| x.as_str().parse::<u32>().unwrap()).unwrap();
        let second_left = captures.get(3)
            .map(|x| x.as_str().parse::<u32>().unwrap()).unwrap();
        let second_right = captures.get(4)
            .map(|x| x.as_str().parse::<u32>().unwrap()).unwrap();
        ((first_left, first_right), (second_left, second_right))
    }).collect();
    output
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = parse(input);
    let result: u32 = output.iter().map(|((a_1, a_2), (b_1, b_2))| {
        let left_contained =
            max(a_1, b_1) == a_1 &&
                min(a_2, b_2) == a_2;
        let right_contained =
            max(a_1, b_1) == b_1 &&
                min(a_2, b_2) == b_2;
        (left_contained || right_contained) as u32
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = parse(input);
    let result: Vec<u32> = output.iter().map(|((a_1, a_2), (b_1, b_2))| {
        (!(a_2 < b_1 || b_2 < a_1)) as u32
    }).collect();
    println!("{:?}", result);
    Some(result.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4, None);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4, None);
        assert_eq!(part_two(&input), Some(4));
    }
}
