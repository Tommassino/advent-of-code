advent_of_code::solution!(1);
pub fn part_one(input: &str) -> Option<u32> {
    let calibration_value: u32 = input
        .lines()
        .map(|calibration_string| {
            let calibration_numbers: Vec<u32> = calibration_string
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            let calibration_number: u32 =
                calibration_numbers.first().unwrap() * 10 + calibration_numbers.last().unwrap();
            calibration_number
        })
        .sum::<u32>();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
