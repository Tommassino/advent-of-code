use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let all_decreasing = numbers.clone().tuple_windows().all(|(a, b)| a > b);
        let all_increasing = numbers.clone().tuple_windows().all(|(a, b)| a < b);
        let max_diff = numbers
            .clone()
            .tuple_windows()
            .map(|(a, b)| a.abs_diff(b))
            .max()
            .unwrap();
        if (all_decreasing || all_increasing) && max_diff <= 3 {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        for i in 0..numbers.len() {
            let mut numbers = numbers.clone();
            numbers.remove(i);
            let all_decreasing = numbers
                .clone()
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| a > b);
            let all_increasing = numbers
                .clone()
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| a < b);
            let max_diff = numbers
                .clone()
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| a.abs_diff(b))
                .max()
                .unwrap();
            if (all_decreasing || all_increasing) && max_diff <= 3 {
                safe_reports += 1;
                break;
            }
        }
    }
    Some(safe_reports)
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
