use std::cmp::Reverse;

use lazy_static::lazy_static;
use num::integer::lcm;
use queues::{IsQueue, Queue};
use regex::{RegexBuilder, Regex};

#[derive(Debug)]
struct Monkey {
    _idx: usize,
    items: Queue<usize>,
    operation_multiply: bool,
    operation_rhs: Option<usize>,
    test_mod: usize,
    monkey_success: usize,
    monkey_failure: usize,
}

impl Monkey {
    pub fn process(
        idx: usize,
        monkeys: &mut [Monkey],
        reduce_div: usize,
        field: Option<usize>,
    ) {
        let mut operations: Queue<(usize, usize)> = Queue::new();
        let monkey = monkeys.get_mut(idx).unwrap();
        while let Ok(item) = monkey.items.remove() {
            let mut worry_level = item;
            if monkey.operation_multiply {
                worry_level *= monkey.operation_rhs.unwrap_or(item);
            } else {
                worry_level += monkey.operation_rhs.unwrap_or(item);
            }
            worry_level /= reduce_div;
            worry_level = field.map(|x| worry_level % x).unwrap_or(worry_level);
            if worry_level % monkey.test_mod == 0 {
                operations.add((monkey.monkey_success, worry_level)).expect("");
            } else {
                operations.add((monkey.monkey_failure, worry_level)).expect("");
            }
        }
        while let Ok((monkey, item)) = operations.remove() {
            monkeys[monkey].items.add(item).expect("");
        }
    }
}

lazy_static! {
    pub static ref MONKEY_PATTERN: Regex = RegexBuilder::new(r"Monkey (\d+):
\s+Starting items: ([\d, ]+)
\s+Operation: new = old ([*+]) (\d+|old)
\s+Test: divisible by (\d+)
\s+If true: throw to monkey (\d+)
\s+If false: throw to monkey (\d+)")
            .case_insensitive(true)
            .multi_line(true)
            .build()
            .unwrap();
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let captures = MONKEY_PATTERN.captures(input).unwrap();
        let idx = captures.get(1).unwrap()
            .as_str().parse::<usize>().unwrap();
        let mut items: Queue<usize> = Queue::new();
        captures.get(2).unwrap().as_str()
            .split(", ").for_each(|x| {
            let item = x.parse::<usize>().unwrap();
            items.add(item).expect("Could not add item to queue");
        });
        let operation_multiply = captures.get(3).unwrap().as_str() == "*";
        let operation_rhs = captures.get(4).unwrap().as_str()
            .parse::<usize>().ok();
        let test_mod = captures.get(5).unwrap().as_str()
            .parse::<usize>().unwrap();
        let monkey_success = captures.get(6).unwrap().as_str()
            .parse::<usize>().unwrap();
        let monkey_failure = captures.get(7).unwrap().as_str()
            .parse::<usize>().unwrap();
        Monkey {
            _idx: idx,
            items,
            operation_multiply,
            operation_rhs,
            test_mod,
            monkey_success,
            monkey_failure,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n")
        .map(Monkey::from)
        .collect();
    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 1..=20 {
        for idx in 0..monkeys.len() {
            inspection_counts[idx] += monkeys[idx].items.size();
            Monkey::process(idx, &mut monkeys, 3, None);
        }
        // println!("Round {:?}", round);
        // monkeys.iter().for_each(|monkey| println!("Monkey {:?}: {:?}", monkey._idx, monkey.items));
    }
    // println!("{:?}", inspection_counts);
    inspection_counts.sort_by_key(|w| Reverse(*w));
    Some(inspection_counts[0] * inspection_counts[1])
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n")
        .map(Monkey::from)
        .collect();
    let field = monkeys.iter().fold(1, |state, monkey| {
        lcm(state, monkey.test_mod)
    });
    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            inspection_counts[idx] += monkeys[idx].items.size();
            Monkey::process(idx, &mut monkeys, 1, Some(field));
        }
    }
    // println!("{:?}", inspection_counts);
    inspection_counts.sort_by_key(|w| Reverse(*w));
    Some(inspection_counts[0] * inspection_counts[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11, None);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11, None);
        assert_eq!(part_two(&input), Some(2713310158));
    }

    #[test]
    fn test_monkey_parse() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let result = Monkey::from(input);
        assert_eq!(result._idx, 0);
        assert_eq!(result.test_mod, 23);
        assert_eq!(result.operation_multiply, true);
        assert_eq!(result.operation_rhs, Some(19));
    }
}
