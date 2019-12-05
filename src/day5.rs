use log::{debug, info};
use std::fs;

#[derive(Clone, Debug)]
struct IntCode{
    program: Vec<isize>
}

#[derive(Debug)]
enum OpCode{
    Add,
    Multiply,
    Read,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit
}

impl From<isize> for OpCode {
    fn from(x: isize) -> OpCode {
        match x {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Read,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Exit,
            _ => panic!("Unknown opcode {}", x)
        }
    }
}

#[derive(Debug)]
struct Instruction{
    operation: OpCode,
    immediate: [bool; 3]
}

impl From<isize> for Instruction {
    fn from(x: isize) -> Instruction {
        let opcode = OpCode::from(x % 100);
        let mut i = x;
        i /= 100;
        let mode1 = (i % 10) > 0;
        i /= 10;
        let mode2 = (i % 10) > 0;
        i /= 10;
        let mode3 = (i % 10) > 0;
        Instruction{
            operation: opcode,
            immediate: [mode1, mode2, mode3]
        }
    }
}

impl IntCode{
    fn run(&mut self, input: Vec<isize>) -> Vec<isize> {
        let mut pc = 0;
        let mut input_counter = 0;
        let mut output = Vec::<isize>::new();

        loop {
            let instruction = Instruction::from(self.program[pc]);
            debug!("{:?} at {}", instruction, pc);
            match instruction.operation {
                OpCode::Add => {
                    let result = self.read(pc + 1, instruction.immediate[0]) + self.read(pc + 2, instruction.immediate[1]);
                    self.write(pc + 3, result);
                    pc += 4;
                }
                OpCode::Multiply => {
                    let result = self.read(pc + 1, instruction.immediate[0]) * self.read(pc + 2, instruction.immediate[1]);
                    self.write(pc + 3, result);
                    pc += 4;
                }
                OpCode::Read => {
                    let result = input[input_counter];
                    input_counter += 1;
                    self.write(pc + 1, result);
                    pc += 2;
                }
                OpCode::Output => {
                    let result = self.read(pc + 1, instruction.immediate[0]);
                    output.push(result);
                    pc += 2;
                }
                OpCode::JumpIfTrue => {
                    let value = self.read(pc + 1, instruction.immediate[0]);
                    if value != 0 {
                        pc = self.read(pc + 2, instruction.immediate[1]) as usize;
                    } else {
                        pc += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let value = self.read(pc + 1, instruction.immediate[0]);
                    if value == 0 {
                        pc = self.read(pc + 2, instruction.immediate[1]) as usize;
                    } else {
                        pc += 3;
                    }
                }
                OpCode::LessThan => {
                    if self.read(pc + 1, instruction.immediate[0]) < self.read(pc + 2, instruction.immediate[1]) {
                        self.write(pc + 3, 1);
                    } else {
                        self.write(pc + 3, 0);
                    }
                    pc += 4;
                }
                OpCode::Equals => {
                    if self.read(pc + 1, instruction.immediate[0]) == self.read(pc + 2, instruction.immediate[1]) {
                        self.write(pc + 3, 1);
                    } else {
                        self.write(pc + 3, 0);
                    }
                    pc += 4;
                }
                OpCode::Exit => break
            }
            debug!("{:?}", self.program);
        }
        output
    }

    fn read(&self, pointer: usize, immediate: bool) -> isize{
        if immediate {
            self.program[pointer]
        } else {
            self.program[self.program[pointer] as usize]
        }
    }

    fn write(&mut self, pointer: usize, value: isize) {
        let address = self.program[pointer] as usize;
        self.program[address] = value
    }

    fn from_file(input_file: &str) -> IntCode {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        let program = contents.split(",").map(|x| x.parse::<isize>().unwrap()).collect();

        IntCode{
            program: program
        }
    }
}

pub fn solve(input_file: &str){
    let code = IntCode::from_file(&input_file);

    part1(&code);
    part2(&code);
}

fn part1(program: &IntCode) {
    let mut memory = program.clone();
    let input = vec![1];
    let output = memory.run(input);
    debug!("{:?}", output);
    info!("{:?}", output[output.len() - 1]);
}

fn part2(program: &IntCode) {
    let mut memory = program.clone();
    let input = vec![5];
    let output = memory.run(input);
    debug!("{:?}", output);
    info!("{:?}", output);
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_simple_input_output_program() {
        let mut code = IntCode{
            program: vec![3,0,4,0,99]
        };
        let input = vec![1];
        let output = code.run(input);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn test_immediate_mode_multiplication() {
        let mut code = IntCode{
            program: vec![1002,4,3,4,33]
        };
        code.run(vec![]);
        assert_eq!(code.program[4], 99);
    }

    #[test]
    fn test_negative_values() {
        let mut code = IntCode{
            program: vec![1101,100,-1,4,0]
        };
        code.run(vec![]);
        assert_eq!(code.program[4], 99);
    }

    #[test]
    fn test_equal_to_position() {
        let code = IntCode{
            program: vec![3,9,8,9,10,9,4,9,99,-1,8]
        };
        
        assert_eq!(code.clone().run(vec![9])[0], 0);
        assert_eq!(code.clone().run(vec![8])[0], 1);
        assert_eq!(code.clone().run(vec![7])[0], 0);
    }

    #[test]
    fn test_equal_to_immediate() {
        let code = IntCode{
            program: vec![3,3,1108,-1,8,3,4,3,99]
        };
        
        assert_eq!(code.clone().run(vec![9])[0], 0);
        assert_eq!(code.clone().run(vec![8])[0], 1);
        assert_eq!(code.clone().run(vec![7])[0], 0);
    }

    #[test]
    fn test_less_than_position() {
        let code = IntCode{
            program: vec![3,9,7,9,10,9,4,9,99,-1,8]
        };
        
        assert_eq!(code.clone().run(vec![9])[0], 0);
        assert_eq!(code.clone().run(vec![8])[0], 0);
        assert_eq!(code.clone().run(vec![7])[0], 1);
    }

    #[test]
    fn test_less_than_immediate() {
        let code = IntCode{
            program: vec![3,3,1107,-1,8,3,4,3,99]
        };
        
        assert_eq!(code.clone().run(vec![9])[0], 0);
        assert_eq!(code.clone().run(vec![8])[0], 0);
        assert_eq!(code.clone().run(vec![7])[0], 1);
    }

    #[test]
    fn test_jump_position_mode() {
        let code = IntCode{
            program: vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]
        };
        assert_eq!(code.clone().run(vec![0])[0], 0);
        assert_eq!(code.clone().run(vec![1])[0], 1);
    }

    #[test]
    fn test_jump_immediate_mode() {
        let code = IntCode{
            program: vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]
        };
        assert_eq!(code.clone().run(vec![0])[0], 0);
        assert_eq!(code.clone().run(vec![1])[0], 1);
    }

    #[test]
    fn test_large_program() {
        let code = IntCode{
            program: vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]
        };
        
        assert_eq!(code.clone().run(vec![7])[0], 999);
        assert_eq!(code.clone().run(vec![8])[0], 1000);
        assert_eq!(code.clone().run(vec![9])[0], 1001);
    }
}
