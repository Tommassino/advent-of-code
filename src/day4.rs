use log::{debug, info};
use std::fs;

#[derive(Copy, Clone, Debug)]
struct NumberUtils(usize);

impl NumberUtils {
    fn digits(&self) -> Vec<usize> {
        fn x_inner(n: usize, xs: &mut Vec<usize>) {
            if n >= 10 {
                x_inner(n / 10, xs);
            }
            xs.push(n % 10);
        }
        let mut xs = Vec::new();
        x_inner(self.0, &mut xs);
        xs
    }

    fn non_decreasing(&self) -> bool {
        self.digits().windows(2).all(|w| w[0] <= w[1])
    }

    fn repeated_digit(&self) -> bool {
        self.digits().windows(2).any(|w| w[0] == w[1])
    }

    fn repeated_digit_group(&self) -> bool {
        let digits = self.digits();
        let middle = digits.windows(4).any(|w| {
            w[0] != w[1] && w[1] == w[2] && w[2] != w[3]
        });
        let left_edge = digits[0] == digits[1] && digits[1] != digits[2];
        let l = digits.len();
        let right_edge = digits[l-1] == digits[l-2] && digits[l-2] != digits[l-3];
        middle || left_edge || right_edge
    }

}

pub fn solve(input_file: &str){
    let numbers = parse(input_file);

    part1(numbers[0], numbers[1]);
    part2(numbers[0], numbers[1]);
}

fn parse(input_file: &str) -> Vec<usize> {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    
    contents.trim().split("-").map(|x| x.parse::<usize>().unwrap()).collect()
}

fn part1(number1: usize, number2: usize) {
    let mut count = 0usize;
    for i in number1..=number2 {
        let number = NumberUtils(i);
        if number.non_decreasing() && number.repeated_digit() {
            debug!("{:?}", number);
            count += 1;
        }
    }
    info!("P1: Number of valid passwords is {}", count)
}

fn part2(number1: usize, number2: usize) {
    let mut count = 0usize;
    for i in number1..=number2 {
        let number = NumberUtils(i);
        if number.non_decreasing() && number.repeated_digit_group() {
            debug!("{:?}", number);
            count += 1;
        }
    }
    info!("P2: Number of valid passwords is {}", count)
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_digit_groups() {
        assert_eq!(NumberUtils(112233).repeated_digit_group(), true);
        assert_eq!(NumberUtils(123444).repeated_digit_group(), false);
        assert_eq!(NumberUtils(111122).repeated_digit_group(), true);
    }
}
