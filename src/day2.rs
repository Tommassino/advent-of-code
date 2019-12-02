use log::{debug, info, warn};
use std::fs;

pub fn solve(input_file: &str){
    let program = parse(&input_file);

    debug!("{:?}", program);

    part1(&program);
    part2(&program);
}

fn parse(input_file: &str) -> Vec<usize> {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    
    contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect()
}

fn part1(program: &Vec<usize>) {
    let result = run_program(12, 2, program);
    info!("Value at position 0: {}", result[0])
}

fn part2(program: &Vec<usize>) {
    let solution = (0usize..=99)
        .flat_map(|x| (0usize..=99).map(move |y| (x, y)))
        .find(|(noun, verb)| {
            let result = run_program(*noun, *verb, program);
            result[0] == 19690720
        })
        .unwrap();
    info!("The solution is {}", 100 * solution.0 + solution.1);
}

fn run_program(noun: usize, verb: usize, program: &Vec<usize>) -> Vec<usize> {
    let mut memory = program.to_vec();
    memory[1] = noun;
    memory[2] = verb;
    let mut pc: usize = 0;
    while pc < memory.len() {
        let op = memory[pc];
        match op {
            1 => {
                let result = memory[memory[pc+1]] + memory[memory[pc+2]];
                let target = memory[pc+3];
                memory[target] = result;
                pc += 4;
            },
            2 => {
                let result = memory[memory[pc+1]] * memory[memory[pc+2]];
                let target = memory[pc+3];
                memory[target] = result;
                pc += 4;
            },
            99 => {
                break;
            }
            _ => {
                warn!("Unknown opcode encountered {}", op);
            }
        }
        debug!("{:?}", memory);
    }
    memory
}
