use itertools::Itertools;
use regex::Regex;

pub fn parse(input: &str, stacks: &mut Vec<Vec<char>>) -> Vec<(usize, usize, usize)> {
    let (stacks_str, instructions_str): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let instruction_pattern = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let instructions: Vec<(usize, usize, usize)> = instructions_str.lines().map(|x| {
        let captures = instruction_pattern.captures(x).unwrap();
        let count = captures.get(1)
            .map(|x| x.as_str().parse::<usize>().unwrap()).unwrap();
        let from = captures.get(2)
            .map(|x| x.as_str().parse::<usize>().unwrap()).unwrap();
        let to = captures.get(3)
            .map(|x| x.as_str().parse::<usize>().unwrap()).unwrap();
        (count, from-1, to-1)
    }).collect();

    let stack_count = (stacks_str.lines().last().unwrap().len() + 1) / 4;
    for _ in 0..=stack_count {
        stacks.push(Vec::new());
    }

    stacks_str.lines().rev()
        .filter(|x| !x.chars().nth(1).unwrap().is_ascii_digit())
        .for_each(|line| {
            line.chars().enumerate().for_each(|(pos, c)| {
                if c.is_alphanumeric() {
                    let stack_index = (pos - 1) / 4;
                    stacks.get_mut(stack_index).unwrap().push(c);
                }
            })
        });

    instructions
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let instructions = parse(input, &mut stacks);
    println!("{:?}\n{:?}", stacks, instructions);
    instructions.iter().for_each(|(count, from, to)| {
        for _ in 0..*count {
            let thing = stacks[*from].pop().unwrap();
            println!("popping {:?} from {:?} to {:?}", thing, from, to);
            stacks[*to].push(thing);
        }
        println!("{:?}", stacks);
    });
    let result = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let instructions = parse(input, &mut stacks);
    println!("{:?}\n{:?}", stacks, instructions);
    instructions.iter().for_each(|(count, from, to)| {
        let mut crane_black_hole: Vec<char> = Vec::new();
        for _ in 0..*count {
            let thing = stacks[*from].pop().unwrap();
            crane_black_hole.push(thing);
        }
        crane_black_hole.iter().rev().for_each(|thing| {
            stacks[*to].push(*thing);
        });
        println!("{:?}", stacks);
    });
    let result = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5, None);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5, None);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
