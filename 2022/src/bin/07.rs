use std::collections::HashMap;
use itertools::Itertools;

fn parse(input: &str, dir_sizes: &mut HashMap<String, u32>) {
    let mut cwd: Vec<&str> = Vec::new();
    input.lines().for_each(|line| {
        // println!("{:?}", line);
        let is_command = line.starts_with('$');

        if is_command && line.contains("cd") {
            let target = line.split(' ').nth(2).unwrap();
            if target == ".." {
                cwd.pop();
            } else if target.starts_with('/') {
                cwd.clear();
                target
                    .split('/')
                    .filter(|x| !x.is_empty())
                    .for_each(|x| {
                        cwd.push(x)
                    });
            } else {
                cwd.push(target);
            }
            // println!("Changed dir to {:?}", cwd);
        } else if !is_command {
            if line.starts_with("dir") {
                //do nothing
            } else {
                let size: u32 = line.split(' ')
                    .next().unwrap()
                    .parse().unwrap();
                for idx in 0..=cwd.len() {
                    let path: String = cwd.iter().take(idx).join("/");
                    let dir_size = dir_sizes.get(&path).unwrap_or(&0) + size;
                    dir_sizes.insert(path, dir_size);
                }
                // println!("Updated dir sizes: {:?}", dir_sizes);
            }
        }
    });
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    parse(input, &mut dir_sizes);
    let result = dir_sizes.iter()
        .map(|(_, size)| *size)
        .filter(|size| *size < 100000)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    parse(input, &mut dir_sizes);
    let disk_size: u32 = 70000000;
    let required_space: u32 = 30000000;
    let used_space: u32 = *dir_sizes.get("").unwrap_or(&0u32);
    let min_delete = required_space - (disk_size - used_space);
    let to_delete = dir_sizes.iter()
        .map(|(_, size)| *size)
        .filter(|size| *size >= min_delete)
        .min();
    to_delete
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
