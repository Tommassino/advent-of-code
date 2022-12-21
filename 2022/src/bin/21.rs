use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Sum,
    Sub,
    Div,
    Mul,
}

impl Operation {
    pub fn apply(&self, left: &Expression, right: &Expression) -> Expression {
        match (left, right) {
            (Expression::Literal(left_i), Expression::Literal(right_i)) => {
                match self {
                    Operation::Sum => Expression::Literal(left_i + right_i),
                    Operation::Sub => Expression::Literal(left_i - right_i),
                    Operation::Div => Expression::Literal(left_i / right_i),
                    Operation::Mul => Expression::Literal(left_i * right_i)
                }
            }
            (x, y) => {
                Expression::Op(
                    *self,
                    Box::new(x.clone()),
                    Box::new(y.clone()),
                )
            }
        }
    }

    pub fn invert(&self) -> Operation {
        match self {
            Operation::Sum => Operation::Sub,
            Operation::Sub => Operation::Sum,
            Operation::Div => Operation::Mul,
            Operation::Mul => Operation::Div
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Op(Operation, Box<Expression>, Box<Expression>),
    Variable(String),
    Literal(u64),
    Unknown(String),
}

impl Expression {
    pub fn value(&self) -> Option<u64> {
        match self {
            Expression::Literal(value) => Some(*value),
            _ => None
        }
    }

    pub fn invert(&self, value: Expression) -> Expression {
        // println!("{:?} == {:?}", self, value);
        match self {
            Expression::Unknown(_) => value,
            Expression::Op(op, left, right) => {
                if matches!(**left, Expression::Literal(_)) {
                    match op {
                        // equation: c_1 ~ x = c_2
                        //           x = c_2 !~ c_1
                        Operation::Sum | Operation::Mul =>
                            right.invert(op.invert().apply(&value, left)),
                        // equation: c_1 ~ x = c_2
                        //           x = c_1 ~ c_2
                        Operation::Sub | Operation::Div =>
                            right.invert(op.apply(left, &value))
                    }
                } else {
                    // equation: x ~ c_1 = c_2
                    //           x = c_2 !~ c_1
                    left.invert(op.invert().apply(&value, right))
                }
            }
            _ => panic!("")
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "+" => Ok(Operation::Sum),
            "-" => Ok(Operation::Sub),
            "/" => Ok(Operation::Div),
            "*" => Ok(Operation::Mul),
            _ => Err(format!("Unknown operation {}", input))
        }
    }
}

struct Equations {
    equations: HashMap<String, Expression>,
}

impl Equations {
    pub fn eval(&self, expression: &Expression) -> Expression {
        match expression {
            Expression::Op(op, expr1, expr2) =>
                op.apply(&self.eval(expr1), &self.eval(expr2)),
            Expression::Literal(num) =>
                Expression::Literal(*num),
            Expression::Variable(var) =>
                self.eval(&self.equations[var]),
            Expression::Unknown(name) =>
                Expression::Unknown(name.clone()),
        }
    }
}

impl FromStr for Equations {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = HashMap::new();
        input.lines().for_each(|x| {
            let (idx, job): (&str, &str) = x.split(": ").next_tuple().unwrap();
            if job.chars().next().unwrap().is_ascii_digit() {
                result.insert(
                    idx.to_string(),
                    Expression::Literal(job.parse::<u64>().unwrap()),
                );
            } else {
                let (operand_1, operation_str, operand_2) = job.split(' ').next_tuple().unwrap();
                let operation = Operation::from_str(operation_str).expect("");
                result.insert(
                    idx.to_string(),
                    Expression::Op(
                        operation,
                        Box::new(Expression::Variable(operand_1.to_string())),
                        Box::new(Expression::Variable(operand_2.to_string())),
                    ),
                );
            }
        });
        Ok(Equations {
            equations: result
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = Equations::from_str(input).expect("");
    match equations.eval(&Expression::Variable("root".to_string())) {
        Expression::Literal(num) => Some(num),
        _ => None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut equations = Equations::from_str(input).expect("");
    equations.equations.insert("humn".to_string(), Expression::Unknown("humn".to_string()));
    let root_eq = equations.equations.get("root").unwrap();
    match root_eq {
        Expression::Op(_, left, right) => {
            let left_eq = equations.eval(left);
            let value = equations.eval(right);
            left_eq.invert(value).value()
        }
        _ => panic!("")
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::ptr::eq;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21, None);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21, None);
        assert_eq!(part_two(&input), Some(301));
    }

    #[test]
    fn test_invert() {
        let mut equations = Equations {
            equations: Default::default()
        };
        // equation 13 - x == 7 ~ x == 6
        let op_unknown_right = Expression::Op(
            Operation::Sub,
            Box::new(Expression::Literal(13)),
            Box::new(Expression::Unknown("x".to_string())),
        );
        assert_eq!(
            op_unknown_right.invert(Expression::Literal(7)).value(),
            Some(6)
        );
        // equation x - 10 == 11 ~ x == 21
        let op_unknown_left = Expression::Op(
            Operation::Sub,
            Box::new(Expression::Unknown("x".to_string())),
            Box::new(Expression::Literal(10)),
        );
        assert_eq!(
            op_unknown_left.invert(Expression::Literal(11)).value(),
            Some(21)
        );
        // equation 6 / x == 3 ~ x == 2
        let op_unknown_right = Expression::Op(
            Operation::Div,
            Box::new(Expression::Literal(6)),
            Box::new(Expression::Unknown("x".to_string())),
        );
        assert_eq!(
            op_unknown_right.invert(Expression::Literal(3)).value(),
            Some(2)
        );
        // equation x / 5 == 3 ~ x == 15
        let op_unknown_left = Expression::Op(
            Operation::Div,
            Box::new(Expression::Unknown("x".to_string())),
            Box::new(Expression::Literal(5)),
        );
        assert_eq!(
            op_unknown_left.invert(Expression::Literal(3)).value(),
            Some(15)
        );
    }
}
