use log::{debug, info};
use std::fs;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Program{
    pub program: Vec<isize>
}

impl Program{
    pub fn from_file(input_file: &str) -> Program {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        let program = contents.split(",").map(|x| x.parse::<isize>().unwrap()).collect();

        Program{
            program: program
        }
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

#[derive(Debug)]
pub struct Computer{
    memory: Vec<isize>,
    pc: usize,
    pub state: ComputerState,
    input: Option<Rc<RefCell<Stream>>>,
    output: Option<Rc<RefCell<Stream>>>
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
            output: None
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

    pub fn run(&mut self, input: Vec<isize>) -> Vec<isize> {
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
        debug!("Running program from pc {} with input {:?}", self.pc, input.data);
        self.state = loop {
            let instruction = Instruction::from(self.memory[self.pc]);
            //debug!("{:?} at {}", instruction, self.pc);
            match instruction.operation {
                OpCode::Add => {
                    let result = self.read(self.pc + 1, instruction.immediate[0]) + self.read(self.pc + 2, instruction.immediate[1]);
                    self.write(self.pc + 3, result);
                    self.pc += 4;
                }
                OpCode::Multiply => {
                    let result = self.read(self.pc + 1, instruction.immediate[0]) * self.read(self.pc + 2, instruction.immediate[1]);
                    self.write(self.pc + 3, result);
                    self.pc += 4;
                }
                OpCode::Read => {
                    match input.read() {
                        Some(result) => {
                            self.write(self.pc + 1, result);
                            self.pc += 2;
                        }
                        None => break ComputerState::Paused
                    }
                }
                OpCode::Output => {
                    let result = self.read(self.pc + 1, instruction.immediate[0]);
                    output.write(result);
                    self.pc += 2;
                }
                OpCode::JumpIfTrue => {
                    let value = self.read(self.pc + 1, instruction.immediate[0]);
                    if value != 0 {
                        self.pc = self.read(self.pc + 2, instruction.immediate[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let value = self.read(self.pc + 1, instruction.immediate[0]);
                    if value == 0 {
                        self.pc = self.read(self.pc + 2, instruction.immediate[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                OpCode::LessThan => {
                    if self.read(self.pc + 1, instruction.immediate[0]) < self.read(self.pc + 2, instruction.immediate[1]) {
                        self.write(self.pc + 3, 1);
                    } else {
                        self.write(self.pc + 3, 0);
                    }
                    self.pc += 4;
                }
                OpCode::Equals => {
                    if self.read(self.pc + 1, instruction.immediate[0]) == self.read(self.pc + 2, instruction.immediate[1]) {
                        self.write(self.pc + 3, 1);
                    } else {
                        self.write(self.pc + 3, 0);
                    }
                    self.pc += 4;
                }
                OpCode::Exit => break ComputerState::Halted
            }
            //debug!("{:?}", self.program);
        };
        debug!("program finished in state {:?} with output {:?}", self.state, output.data);
    }

    fn read(&self, pointer: usize, immediate: bool) -> isize{
        if immediate {
            self.memory[pointer]
        } else {
            self.memory[self.memory[pointer] as usize]
        }
    }

    fn write(&mut self, pointer: usize, value: isize) {
        let address = self.memory[pointer] as usize;
        self.memory[address] = value
    }
}

#[derive(Debug)]
pub struct Stream{
    data: VecDeque<isize>
}

impl Stream{
    pub fn new() -> Stream {
        Stream{
            data: VecDeque::new()
        }
    }

    pub fn read(&mut self) -> Option<isize> {
        self.data.pop_front()
    }

    pub fn write(&mut self, value: isize) {
        self.data.push_back(value)
    }

    fn into_vec(self) -> Vec<isize> {
        self.data.into()
    }
}