use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let solution: u32 = input.lines().map(|x| {
        let mut chars = HashSet::new();
        let midpoint = x.len() / 2;
        x.chars().take(midpoint).for_each(|x| {
            chars.insert(x);
        });
        let misplaced: char = x.chars().skip(midpoint).flat_map(|x| {
            if chars.contains(&x) {
                Some(x)
            } else {
                None
            }
        }).next().unwrap();
        if misplaced.is_uppercase() {
            misplaced as u32 - 'A' as u32 + 27
        } else {
            misplaced as u32 - 'a' as u32 + 1
        }
    }).sum();
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.lines().collect();

    let badges: Vec<char> = rucksacks.chunks(3).map(|chunks| {
        let mut common: HashSet<char> = HashSet::new();
        for (i, item) in chunks.iter().enumerate().take(3) {
            if i == 0 {
                item.chars().for_each(|x| {common.insert(x);});
            } else {
                common.retain(|x| item.contains(*x));
            }
        }
        if common.len() != 1 {
            panic!("Got more than one badge {:?}", common);
        }
        *common.iter().next().unwrap()
    }).collect();

    let score = badges.iter().map(|x| {
        if x.is_uppercase() {
            *x as u32 - 'A' as u32 + 27
        } else {
            *x as u32 - 'a' as u32 + 1
        }
    }).sum();
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3, None);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3, None);
        assert_eq!(part_two(&input), Some(70));
    }
}
