use log::{debug, info};
use std::rc::Rc;
use std::cell::RefCell;

mod intcode;
use intcode::*;

pub fn solve(input_file: &str){
    let code = Program::from_file(&input_file);

    part1(&code);
    part2(&code);
}

fn part1(program: &Program) {
    let mut computer = Computer::new(&program);
    let output = computer.run(vec![1]);
    info!("BOOST {:?}", output);
}

fn part2(program: &Program) {
    let mut computer = Computer::new(&program);
    let output = computer.run(vec![2]);
    info!("BOOST {:?}", output);
}


#[cfg(test)]
mod tests{
    use super::*;
    
}
