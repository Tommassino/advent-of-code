pub fn part_one(input: &str) -> Option<u32> {
    let elf_caloric_cache: u32 = input.split("\n\n").map(|elf_info| {
        elf_info.lines().map(|x| x.parse::<u32>().unwrap()).sum()
    }).max().unwrap();
    Some(elf_caloric_cache)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_caloric_caches: Vec<u32> = input.split("\n\n").map(|elf_info| {
        elf_info.lines().map(|x| x.parse::<u32>().unwrap()).sum()
    }).collect();
    elf_caloric_caches.sort();
    elf_caloric_caches.reverse();
    Some(elf_caloric_caches.iter().take(3).sum())
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1, None);
        assert_eq!(part_two(&input), Some(45000));
    }
}
