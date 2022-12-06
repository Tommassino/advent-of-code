use itertools::Itertools;

pub fn is_unique(data: &[char]) -> bool {
    data
        .iter()
        .all_unique()
}

pub fn part_one(input: &str) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .windows(4)
        .enumerate()
        .find(|&(_, window)| is_unique(window))
        .map(|(index, window)| {
            println!("found unique at pos {:?} {:?}", index, window);
            index + 4
        })
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .windows(14)
        .enumerate()
        .find(|&(_, window)| is_unique(window))
        .map(|(index, window)| {
            println!("found unique at pos {:?} {:?}", index, window);
            index + 14
        })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
