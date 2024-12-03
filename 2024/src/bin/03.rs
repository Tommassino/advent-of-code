advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for cap in mul_regex.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        sum += x * y;
    }
    Some(sum)
}

#[derive(Debug)]
enum Token {
    Mul(u32, u32),
    Do,
    Dont,
}

trait Interpreter {
    fn apply(&mut self, token: &Token);
}

#[derive(Debug)]
struct State {
    sum: u32,
    enabled: bool,
}
impl Interpreter for State {
    fn apply(&mut self, token: &Token) {
        match token {
            Token::Mul(x, y) => {
                if self.enabled {
                    self.sum += x * y;
                }
            }
            Token::Do => {
                self.enabled = true;
            }
            Token::Dont => {
                self.enabled = false;
            }
        }
    }
}

#[derive(Debug)]
struct Program {
    tokens: Vec<Token>,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();
        for cap in regex.captures_iter(value) {
            if let Some(x) = cap.get(1) {
                let x: u32 = x.as_str().parse().unwrap();
                let y: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
                tokens.push(Token::Mul(x, y));
            } else if cap.get(3).is_some() {
                tokens.push(Token::Do);
            } else if cap.get(4).is_some() {
                tokens.push(Token::Dont);
            }
        }
        Program { tokens }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let program = Program::from(input);
    let mut interpreter = State {
        sum: 0,
        enabled: true,
    };
    for token in program.tokens {
        interpreter.apply(&token);
    }
    Some(interpreter.sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
