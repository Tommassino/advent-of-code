pub fn part_one(input: &str) -> Option<u32> {
    let calibration_value: u32 = input.lines().map(|calibration_string| {
        let calibration_numbers: Vec<u32> = calibration_string.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap()).collect();
        let calibration_number: u32 = calibration_numbers.first().unwrap() * 10 + calibration_numbers.last().unwrap();
        calibration_number
    }).sum::<u32>();
    Some(calibration_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let formatted_str = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    part_one(formatted_str.as_str())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1, None);
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1, None);
        assert_eq!(part_two(&input), Some(281));
    }
}
