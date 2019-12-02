use log::{debug, info};
use std::fs;

#[derive(Debug)]
struct IntCode{
    program: Vec<usize>
}

#[derive(Debug)]
enum OpCode{
    Add,
    Multiply,
    Exit
}

impl From<usize> for OpCode {
    fn from(x: usize) -> OpCode {
        match x {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Exit,
            _ => panic!("Unknown opcode {}", x)
        }
    }
}

impl IntCode{
    fn run(&self, noun: usize, verb: usize) -> Vec<usize> {
        let mut memory = self.program.clone();
        let mut pc = 0;
        memory[1] = noun;
        memory[2] = verb;

        loop {
            match OpCode::from(memory[pc]) {
                OpCode::Add => {
                    let target = memory[pc+3];
                    memory[target] = memory[memory[pc + 1]] + memory[memory[pc + 2]];
                    pc += 4;
                }
                OpCode::Multiply => {
                    let target = memory[pc+3];
                    memory[target] = memory[memory[pc + 1]] * memory[memory[pc + 2]];
                    pc += 4;
                }
                OpCode::Exit => break
            }
            debug!("{:?}", memory);
        }
        memory
    }

    fn from_file(input_file: &str) -> IntCode {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        let program = contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

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
    let result = program.run(12, 2);
    info!("Value at position 0: {}", result[0])
}

fn part2(program: &IntCode) {
    let solution = (0usize..=99)
        .flat_map(|x| (0usize..=99).map(move |y| (x, y)))
        .find(|(noun, verb)| {
            let result = program.run(*noun, *verb);
            result[0] == 19690720
        })
        .unwrap();
    info!("The solution is {}", 100 * solution.0 + solution.1);
}
