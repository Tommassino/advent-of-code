use log::{debug, info};
use std::fs;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
//use num_i128::{i128, Toi128, Fromi128};

#[derive(Clone, Debug)]
pub struct Program{
    pub program: Vec<i128>
}

impl Program{
    pub fn from_str(contents: &str) -> Program {
        let program = contents.split(",").map(|x| x.parse::<i128>().unwrap()).collect();

        Program{
            program: program
        }
    }

    pub fn from_file(input_file: &str) -> Program {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        Program::from_str(&contents)
    }
}

#[derive(Debug)]
pub enum OpCode{
    Add,
    Multiply,
    Read,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    SetRelative,
    Exit
}

impl From<usize> for OpCode {
    fn from(x: usize) -> OpCode {
        match x {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Read,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::SetRelative,
            99 => OpCode::Exit,
            _ => panic!("Unknown opcode {}", x)
        }
    }
}

#[derive(Debug)]
struct Instruction{
    operation: OpCode,
    mode: [ParameterMode; 3]
}

impl From<i128> for Instruction {
    fn from(x: i128) -> Instruction {
        let opcode = OpCode::from((x % 100) as usize);
        let mut i = x;
        i /= 100;
        let mode1 = ParameterMode::from((i % 10) as usize);
        i /= 10;
        let mode2 = ParameterMode::from((i % 10) as usize);
        i /= 10;
        let mode3 = ParameterMode::from((i % 10) as usize);
        Instruction{
            operation: opcode,
            mode: [mode1, mode2, mode3]
        }
    }
}

#[derive(Debug)]
enum ParameterMode{
    Position,
    Absolute,
    Relative
}

impl From<usize> for ParameterMode{
    fn from(mode: usize) -> ParameterMode {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Absolute,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown mode encountered: {}", mode)
        }
    }
}

#[derive(Debug)]
pub struct Computer{
    memory: Vec<i128>,
    pc: usize,
    pub state: ComputerState,
    input: Option<Rc<RefCell<Stream>>>,
    output: Option<Rc<RefCell<Stream>>>,
    relative_base: i128
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComputerState{
    Idle,
    Halted,
    Paused
}

impl Computer{
    pub fn new(program: &Program) -> Computer {
        Computer{
            memory: program.program.clone(),
            pc: 0,
            state: ComputerState::Idle,
            input: None,
            output: None,
            relative_base: 0
        }
    }

    pub fn input(&self) -> Option<Rc<RefCell<Stream>>> {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: Option<Rc<RefCell<Stream>>>) {
        self.input = input;
    }

    pub fn output(&self) -> Option<Rc<RefCell<Stream>>> {
        self.output.clone()
    }

    pub fn set_output(&mut self, output: Option<Rc<RefCell<Stream>>>) {
        self.output = output;
    }

    pub fn run(&mut self, input: Vec<i128>) -> Vec<i128> {
        let mut input_stream = Stream::new();
        input.iter().for_each(|x| input_stream.write(*x));
        let mut output_stream = Stream::new();
        self.run_io(&mut input_stream, &mut output_stream);
        output_stream.into_vec()
    }
    
    pub fn execute(&mut self) {
        let input_ref = self.input.clone()
            .unwrap_or_else(|| Rc::new(RefCell::new(Stream::new())));
            
        let output_ref = self.output.clone()
            .unwrap_or_else(|| Rc::new(RefCell::new(Stream::new())));

        self.run_io(&mut input_ref.borrow_mut(), &mut output_ref.borrow_mut());
    }
    
    fn run_io(&mut self, input: &mut Stream, output: &mut Stream) {
        info!("Running program from pc {} with input {:?}", self.pc, input.data);
        self.state = loop {
            let instruction = Instruction::from(self.memory[self.pc]);
            debug!("{:?} at {}", instruction, self.pc);
            match instruction.operation {
                OpCode::Add => {
                    let result = self.read(1, &instruction) + self.read(2, &instruction);
                    self.write(3, &instruction, result);
                    self.pc += 4;
                }
                OpCode::Multiply => {
                    let result = self.read(1, &instruction) * self.read(2, &instruction);
                    self.write(3, &instruction, result);
                    self.pc += 4;
                }
                OpCode::Read => {
                    match input.read() {
                        Some(result) => {
                            self.write(1, &instruction, result);
                            self.pc += 2;
                        }
                        None => break ComputerState::Paused
                    }
                }
                OpCode::Output => {
                    let result = self.read(1, &instruction);
                    output.write(result);
                    self.pc += 2;
                }
                OpCode::JumpIfTrue => {
                    let value = self.read(1, &instruction);
                    if value != 0 {
                        self.pc = self.read(2, &instruction) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let value = self.read(1, &instruction);
                    if value == 0 {
                        self.pc = self.read(2, &instruction) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::LessThan => {
                    if self.read(1, &instruction) < self.read(2, &instruction) {
                        self.write(3, &instruction, 1);
                    } else {
                        self.write(3, &instruction, 0);
                    }
                    self.pc += 4;
                }
                OpCode::Equals => {
                    if self.read(1, &instruction) == self.read(2, &instruction) {
                        self.write(3, &instruction, 1);
                    } else {
                        self.write(3, &instruction, 0);
                    }
                    self.pc += 4;
                }
                OpCode::SetRelative => {
                    self.relative_base += self.read(1, &instruction);
                    self.pc += 2;
                }
                OpCode::Exit => break ComputerState::Halted
            }
            debug!("{:?}", self.memory);
        };
        debug!("program finished in state {:?} with output {:?}", self.state, output.data);
    }

    fn read(&mut self, offset: usize, instruction: &Instruction) -> i128{
        let address = self.get_address(offset, instruction);
        self._read(address)
    }

    fn write(&mut self, offset: usize, instruction: &Instruction, value: i128) {
        let address = self.get_address(offset, instruction);
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value
    }

    fn _read(&mut self, position: usize) -> i128 {
        if position >= self.memory.len() {
            self.memory.resize(position + 1, 0);
        }
        self.memory[position]
    }

    fn get_address(&mut self, offset: usize, instruction: &Instruction) -> usize {
        match instruction.mode[offset - 1] {
            ParameterMode::Position => self._read(self.pc + offset) as usize,
            ParameterMode::Absolute => self.pc + offset,
            ParameterMode::Relative => (self._read(self.pc + offset) + self.relative_base) as usize
        }
    }
}

#[derive(Debug)]
pub struct Stream{
    data: VecDeque<i128>
}

impl Stream{
    pub fn new() -> Stream {
        Stream{
            data: VecDeque::new()
        }
    }

    pub fn read(&mut self) -> Option<i128> {
        self.data.pop_front()
    }

    pub fn write(&mut self, value: i128) {
        self.data.push_back(value)
    }

    fn into_vec(self) -> Vec<i128> {
        self.data.into()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger;

    #[test]
    fn test_relative_instruction() {
        let program = Program::from_str("109,19,204,-34,99");
        let mut computer = Computer::new(&program);
        computer.relative_base = 2000;
        let output = computer.run(vec![]);
        assert_eq!(computer.relative_base, 2019);
        assert_eq!(output[0], 0);
    }
    
    #[test]
    fn test_copy() {
        let program = Program::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut computer = Computer::new(&program);
        let output = computer.run(vec![]);
        println!("{:?}", output);
    }

    #[test]
    fn test_large_number() {
        let program = Program::from_str("1102,34915192,34915192,7,4,7,99,0");
        let mut computer = Computer::new(&program);
        let output = computer.run(vec![]);
        assert_eq!(format!("{}", output[0]).len(), 16);
    }

    #[test]
    fn test_large_number2() {
        let program = Program::from_str("104,1125899906842624,99");
        let mut computer = Computer::new(&program);
        let output = computer.run(vec![]);
        assert_eq!(format!("{}", output[0]).len(), 16);
    }
}