extern crate core;

use itertools::Itertools;
use queues::{IsQueue, Queue};

#[derive(Debug, Copy, Clone)]
enum OpCode {
    NoOp,
    AddX,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    op_code: OpCode,
    parameter: i64,
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        if instruction.starts_with("noop") {
            Instruction {
                op_code: OpCode::NoOp,
                parameter: 0,
            }
        } else {
            let (op, param_str) = instruction.split(' ').next_tuple().unwrap();
            let param_int = param_str.parse::<i64>().unwrap();
            match op {
                "addx" => Instruction {
                    op_code: OpCode::AddX,
                    parameter: param_int,
                },
                _ => panic!("Unknown op: {}", op),
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cpu {
    cycle: usize,
    register: i64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            cycle: 1,
            register: 1,
        }
    }

    pub fn interpret(&self, program: &str) -> CPUIterator {
        let mut queue: Queue<Instruction> = Queue::new();
        program.lines().for_each(|x| {
            queue.add(Instruction::from(x)).expect("");
        });
        CPUIterator {
            program: queue,
            current_instruction: None,
            instruction_counter: 0,
            cpu: *self,
        }
    }
}

#[derive(Debug)]
struct CPUIterator {
    program: Queue<Instruction>,
    current_instruction: Option<Instruction>,
    instruction_counter: u8,
    cpu: Cpu,
}

impl Iterator for CPUIterator {
    type Item = Cpu;

    fn next(&mut self) -> Option<Self::Item> {
        let next_instruction = if self.current_instruction.is_none() {
            self.program.remove().ok()
        } else {
            self.current_instruction
        };

        if let Some(instruction) = next_instruction {
            // we return the CPU state before we evaluate the instruction
            let result = self.cpu;
            // println!("Interpreting {:?}", instruction);
            match instruction.op_code {
                OpCode::NoOp => {
                    self.cpu.cycle += 1;
                    self.instruction_counter = 0;
                    self.current_instruction = None
                }
                OpCode::AddX => {
                    self.cpu.cycle += 1;
                    self.instruction_counter += 1;
                    if self.instruction_counter == 2 {
                        self.cpu.register += instruction.parameter;
                        self.instruction_counter = 0;
                        self.current_instruction = None;
                    } else {
                        self.current_instruction = next_instruction;
                    }
                }
            }
            Some(result)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let cpu = Cpu::new();
    let mut signal_counter: i64 = 0;
    cpu.interpret(input).for_each(|state| {
        if state.cycle % 40 == 20 {
            println!("{:?}", state);
            signal_counter += state.cycle as i64 * state.register;
        }
    });
    Some(signal_counter)
}

pub fn part_two(input: &str) -> Option<String> {
    let cpu = Cpu::new();
    let render: Vec<String> = cpu
        .interpret(input)
        .map(|state| {
            let pixel_position = (state.cycle as i64 - 1) % 40;
            let sprite_position = state.register;
            if (pixel_position - sprite_position).abs() <= 1 {
                '█'
            } else {
                ' '
            }
        })
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|x| {
            x.iter().collect()
        }).collect();
    render.iter().for_each(|line| {
        println!("{}", line);
    });
    Some(String::from("ECZUZALR"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10, None);
        let cpu = Cpu::new();
        cpu.interpret(&input).for_each(|state| {
            match state.cycle {
                1 => assert_eq!(state.register, 1),
                2 => assert_eq!(state.register, 1),
                3 => assert_eq!(state.register, 1),
                4 => assert_eq!(state.register, 4),
                5 => assert_eq!(state.register, 4),
                6 => assert_eq!(state.register, -1),
                _ => panic!("")
            }
        });
    }

    #[test]
    fn test_part_one_larger() {
        let input = advent_of_code::read_file("examples", 10, Some("larger"));
        assert_eq!(part_one(&input), Some(13140))
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10, Some("larger"));
        assert_eq!(part_two(&input), Some(String::from("ECZUZALR")));
    }
}
