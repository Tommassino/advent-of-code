use std::str::FromStr;

#[derive(Debug, Clone)]
struct Snafu {
    digits: Vec<i64>,
}

impl Snafu {
    pub fn value(&self) -> i64 {
        let mut total: i64 = 0;
        for (offset, digit) in self.digits.iter().rev().enumerate() {
            total += *digit * i64::pow(5, offset as u32);
        }
        total
    }

    pub fn from_decimal(decimal: i64) -> Snafu {
        let mut digits = Vec::new();
        let mut state = decimal;
        while state > 0 {
            let remainder = state % 5;
            if remainder >= 3 {
                digits.push( remainder - 5);
                state += 5 - remainder;
            } else {
                digits.push(remainder)
            }
            state /= 5;
        }
        digits.reverse();
        Snafu{
            digits
        }
    }

    fn encode(&self) -> String {
        self.digits.iter().map(|d| match d {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => unreachable!()
        }).collect()
    }
}

impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu {
            digits: s.chars().map(|c| {
                match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!()
                }
            }).collect()
        })
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let decimal = input
        .lines()
        .map(|line| {
            Snafu::from_str(line).expect("").value()
        })
        .sum();

    Some(Snafu::from_decimal(decimal).encode())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25, None);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25, None);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_examples() {
        assert_eq!(Snafu::from_str("1").expect("").value(), 1);
        assert_eq!(Snafu::from_str("1121-1110-1=0").expect("").value(), 314159265);
        assert_eq!(Snafu::from_str("1=11-2").expect("").value(), 2022);
        assert_eq!(Snafu::from_str("1-0---0").expect("").value(), 12345);
        assert_eq!(Snafu::from_str("2=-01").expect("").value(), 976);
        assert_eq!(Snafu::from_str("2=-01").expect("").value(), 976);
        assert_eq!(Snafu::from_str("1=-0-2").expect("").value(), 1747);
        assert_eq!(Snafu::from_str("12111").expect("").value(), 906);
        assert_eq!(Snafu::from_str("2=0=").expect("").value(), 198);
        assert_eq!(Snafu::from_str("21").expect("").value(), 11);
        assert_eq!(Snafu::from_str("2=01").expect("").value(), 201);
        assert_eq!(Snafu::from_str("111").expect("").value(), 31);
        assert_eq!(Snafu::from_str("20012").expect("").value(), 1257);
        assert_eq!(Snafu::from_str("112").expect("").value(), 32);
        assert_eq!(Snafu::from_str("1=-1=").expect("").value(), 353);
        assert_eq!(Snafu::from_str("1-12").expect("").value(), 107);
        assert_eq!(Snafu::from_str("12").expect("").value(), 7);
        assert_eq!(Snafu::from_str("1=").expect("").value(), 3);
        assert_eq!(Snafu::from_str("122").expect("").value(), 37);
    }
}
