advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_numbers, mut right_numbers): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip();
    left_numbers.sort();
    right_numbers.sort();
    let mut total_distance = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        total_distance += left.abs_diff(*right);
    }
    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_numbers, right_numbers): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip();
    let right_number_counts =
        right_numbers
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &n| {
                *map.entry(n).or_insert(0) += 1;
                map
            });
    let mut similarity_score = 0;
    for left in left_numbers.iter() {
        if let Some(count) = right_number_counts.get(left) {
            similarity_score += left * count;
        }
    }
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
