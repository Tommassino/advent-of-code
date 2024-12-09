use itertools::Itertools;
advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();
            Equation { test_value, values }
        })
        .collect()
}

impl Equation {
    fn evaluate(&self, operations: &[Operator]) -> bool {
        let mut result = self.values[0];
        for (idx, value) in self.values.iter().skip(1).enumerate() {
            match operations[idx] {
                Operator::Add => result += value,
                Operator::Multiply => result *= value,
                Operator::Concatenate => {
                    result = result * 10_u64.pow(value.ilog10() + 1) + value;
                }
            }
        }
        result == self.test_value
    }

    fn is_possible(&self, with_concatenate: bool) -> bool {
        let operations = if with_concatenate {
            vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
        } else {
            vec![Operator::Add, Operator::Multiply]
        };
        std::iter::repeat(operations)
            .take(self.values.len() - 1)
            .multi_cartesian_product()
            .any(|operations| self.evaluate(&operations))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);
    let sum: u64 = equations
        .iter()
        .filter(|equation| equation.is_possible(false))
        .map(|equation| equation.test_value)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let sum: u64 = equations
        .iter()
        .filter(|equation| equation.is_possible(true))
        .map(|equation| equation.test_value)
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_156() {
        let equation = Equation {
            test_value: 156,
            values: vec![15, 6],
        };
        assert_eq!(equation.is_possible(true), true);
    }

    #[test]
    fn test_7290() {
        let equation = Equation {
            test_value: 7290,
            values: vec![6, 8, 6, 15],
        };
        assert_eq!(equation.is_possible(true), true);
    }

    #[test]
    fn test_192() {
        let equation = Equation {
            test_value: 192,
            values: vec![17, 8, 14],
        };
        assert_eq!(equation.is_possible(true), true);
    }
}
