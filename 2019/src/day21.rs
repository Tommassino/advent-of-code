use log::{debug, info};
use std::char;

use crate::common::intcode::*;

struct SpringDroid{
    program: Program
}

impl SpringDroid{
    fn new(program: &Program) -> SpringDroid {

        SpringDroid {
            program: program.clone()
        }
    }

    fn walk(&self, instructions: &str) -> i128 {
        let mut computer = Computer::new(&self.program);
        let code = instructions.chars().map(|c| {
            c as i128
        }).collect();
        let output = computer.run(code);
        let output_str: String = output.iter()
            .map(|i| char::from_u32(*i as u32).unwrap_or('x'))
            .collect();
        
        debug!("\n{}\n", output_str);
        *output.last().unwrap()
    }
}

pub fn solve(input_file: &str){
    use std::time::Instant;

    let input = SpringDroid::new(&Program::from_file(&input_file));

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(droid: &SpringDroid) {
    // CNF can be directly written with 3 registers:
    // * J holds running value of conjunctions
    // * T holds current clause value
    // * ? used to hold !x - since we do not have NOR
    // so we cant simply use a normalizer
    //
    // jump if there is a hole between 1-3 and not on 4 ~ (!A | !B | !C) && D
    // (NOT a OR NOT b OR NOT c) AND d
    // the problem is with the NOT x OR, fortunately we can do
    // (NOT a OR NOT b OR NOT c) ~ !(A && B && C)
    let program: String = r#"
OR A T
AND B T
AND C T
# T = A & B & C
NOT T J
# J = !T
AND D J
# J = D & !(A & B & C)
WALK
    "#.trim().lines().filter(|x| !x.starts_with("#")).map(|x| format!("{}\n", x)).collect();
    let solution = droid.walk(&format!("{}\n", program));
    println!("Solution is {}", solution);
}

fn part2(droid: &SpringDroid) {
    // jump if there is a hole between 1-3, not on 4, and not on 5 or 8 - if there is a hole on 8 its risky to jump since we might be forced to jump again and into 8
    // (NOT a OR NOT b OR NOT c) AND d AND (e OR h)
    let program: String = r#"
OR A T
AND B T
AND C T
NOT T J
AND D J
# J == part1
# trick to compute e OR h, since registers have unknown value: p1 && (!p1 || E || H) = p1 && (E || H)
NOT J T
OR E T
OR H T
AND T J
RUN
    "#.trim().lines().filter(|x| !x.starts_with("#")).map(|x| format!("{}\n", x)).collect();
    debug!("{}", program);
    let solution = droid.walk(&format!("{}\n", program));
    println!("Solution is {}", solution);
}
