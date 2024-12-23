use std::collections::HashSet;
advent_of_code::solution!(17);

#[derive(Debug)]
struct Input {
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
    program: Vec<u8>,
    output: Vec<i32>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let regex = regex::Regex::new(
            r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.*)",
        )
        .unwrap();
        let captures = regex.captures(s).unwrap();
        let a = captures[1].parse().unwrap();
        let b = captures[2].parse().unwrap();
        let c = captures[3].parse().unwrap();
        let program = captures[4].split(',').map(|s| s.parse().unwrap()).collect();
        Input {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }
}

impl Input {
    fn combo_operand(&self, value: u8) -> i32 {
        match value {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn literal_operand(&self, value: u8) -> i32 {
        match value {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            _ => panic!("Invalid literal operand"),
        }
    }

    fn div(&self, operand: u8) -> i32 {
        let denominator = 2u64.pow(self.combo_operand(operand) as u32);
        let div = self.a as f64 / denominator as f64;
        div.floor() as i32
    }

    fn adv(&mut self, operand: u8) {
        self.a = self.div(operand);
        self.ip += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= self.literal_operand(operand);
        self.ip += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo_operand(operand) % 8;
        self.ip += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.ip = self.literal_operand(operand) as usize;
        } else {
            self.ip += 2;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.b ^= self.c;
        self.ip += 2;
    }

    fn out(&mut self, operand: u8) {
        self.output.push(self.combo_operand(operand) % 8);
        self.ip += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.div(operand);
        self.ip += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.div(operand);
        self.ip += 2;
    }

    fn run(&mut self) {
        self.ip = 0;
        loop {
            if self.ip >= self.program.len() {
                break;
            }
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Invalid opcode"),
            }
        }
    }

    fn get_output(a: i64) -> u8 {
        let p = (a % 8) ^ 3;
        let b = (p ^ (a >> p)) ^ 5;
        (b % 8) as u8
    }

    fn find_identity(&self) -> Option<i64> {
        // Only works for my input
        // A=?, B=0, C=0
        // rewrite and simplify:
        // while true:
        //      B := A % 8          // 2, 4 bst(4) combo A
        //      B ^= 3              // 1, 3 bxl(3) literal 3
        //      C := A >> B         // 7, 5 cdv(5) combo B
        //      B ^= C              // 4, 7 bxc(7)
        //      A := A >> 3         // 0, 3 adv(3) combo 3
        //      B ^= 5              // 1, 5 bxl(5) literal 5
        //      out(B)              // 5, 5 out(5) combo B
        //      A = A >> 3
        //      jnz(A)              // 3, 0 jnz(0)

        // flatten the function
        // A = A >> 3
        // B = (P ^ (A >> P)) ^ 5
        // C = A >> P
        // P = (A % 8) ^ 3

        // after every loop, we output B and can zero out C, and shift A by 3 bits
        // so we need to find solutions for A so that B is the first digit of the output
        let mut current = HashSet::new();
        current.insert(0);
        for &target in self.program.clone().iter().rev() {
            let mut new_current = HashSet::new();
            for &a in &current {
                // append 3 bits to the left of a
                for i in 0..8 {
                    let new_a = (a << 3) + i;
                    if Input::get_output(new_a) == target {
                        new_current.insert(new_a);
                    }
                }
            }
            if new_current.is_empty() {
                panic!("No solution found");
            }
            current = new_current;
        }
        current.iter().min().cloned()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut input = Input::from(input);
    input.run();
    Some(
        input
            .output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = Input::from(input);
    input.find_identity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_program_1() {
        let mut input = Input {
            a: 0,
            b: 0,
            c: 9,
            ip: 0,
            program: vec![2, 6],
            output: Vec::new(),
        };
        input.run();
        assert_eq!(input.b, 1);
    }

    #[test]
    fn test_program_2() {
        let mut input = Input {
            a: 10,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
        };
        input.run();
        assert_eq!(input.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_program_3() {
        let mut input = Input {
            a: 2024,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        };
        input.run();
        assert_eq!(input.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_program_4() {
        let mut input = Input {
            a: 0,
            b: 29,
            c: 0,
            ip: 0,
            program: vec![1, 7],
            output: Vec::new(),
        };
        input.run();
        assert_eq!(input.b, 26);
    }

    #[test]
    fn test_program_5() {
        let mut input = Input {
            a: 0,
            b: 2024,
            c: 43690,
            ip: 0,
            program: vec![4, 0],
            output: Vec::new(),
        };
        input.run();
        assert_eq!(input.b, 44354);
    }
}
