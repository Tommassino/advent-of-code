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
    use permutohedron::heap_recursive;
    let mut best = 0;
    let mut data = [0, 1, 2, 3, 4];
    heap_recursive(&mut data, |permutation| {
        debug!("{:?}", permutation);
        let amplified = run_amplifier(permutation.to_vec(), program);
        if amplified > best {
            best = amplified;
        }
    });
    info!("{}", best);
}

fn part2(program: &Program) {
    use permutohedron::heap_recursive;
    let mut best = 0;
    let mut data = [5, 6, 7, 8, 9];

    heap_recursive(&mut data, |permutation| {
        debug!("{:?}", permutation);
        let amplified = run_amplifier_recursive(permutation.to_vec(), program);
        if amplified > best {
            best = amplified;
        }
    });
    
    info!("{}", best);
}

fn run_amplifier(phase_sequence: Vec<isize>, program: &Program) -> isize {
    phase_sequence.iter().fold(0isize, |input, phase| {
        let mut computer = intcode::Computer::new(program);
        let output = computer.run(vec![*phase, input]);
        *output.get(0).expect("No output generated!")
    })
}

fn run_amplifier_recursive(phase_sequence: Vec<isize>, program: &Program) -> isize {
    let mut amplifiers = vec![
        intcode::Computer::new(program),
        intcode::Computer::new(program),
        intcode::Computer::new(program),
        intcode::Computer::new(program),
        intcode::Computer::new(program)
    ];
    //init input
    for i in 0..=4{
        let phase = phase_sequence[i];
        let mut input = intcode::Stream::new();
        input.write(phase);
        if i == 0 {
            //init input
            input.write(0);
        }
        amplifiers[i].set_input(Some(Rc::new(RefCell::new(input))));
    }
    //loop input
    for i in 0..=4{
        debug!("Linking output of {} to input of {}", i, (i + 1) % 5);
        let input = amplifiers[(i + 1) % 5].input();
        amplifiers[i].set_output(input);
    }

    'outer: loop {
        for i in 0..=4 {
            amplifiers[i].execute();
            if i == 4 && amplifiers[i].state == intcode::ComputerState::Halted {
                break 'outer;
            }
        }
    }
    amplifiers[4].output().unwrap().borrow_mut().read().unwrap()
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn amplifier_correct() {
        assert_eq!(
            43210,
            run_amplifier(
                vec![4,3,2,1,0], 
                &Program{program: vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]}
            )
        );

    }
    #[test]
    fn amplifier_recursive_correct() {
        assert_eq!(
            139629729,
            run_amplifier_recursive(
                vec![9,8,7,6,5], 
                &Program{program: vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]}
            )
        );
        assert_eq!(
            18216,
            run_amplifier_recursive(
                vec![9,7,8,5,6], 
                &Program{program: vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]}
            )
        );


    }
}
