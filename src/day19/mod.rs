use log::{debug, info};

mod intcode;
use intcode::*;

pub fn solve(input_file: &str){
    use std::time::Instant;

    let input = Program::from_file(&input_file);

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(program: &Program) {
    let mut points: Vec<i128> = Vec::new();

    for x in 0..50 {
        for y in 0..50 {
            let mut computer = Computer::new(program);
            let output = computer.run(vec![x, y]);
            output.iter().for_each(|o| points.push(*o));
        }
    }

    let solution: i128 = points.iter().sum();
    println!("Tractor area {}", solution);
}

#[derive(Clone)]
struct Drone{
    program: Program
}

impl Drone{
    fn new(program: &Program) -> Drone {
        Drone{
            program: program.clone()
        }
    }

    fn deploy(&self, x: usize, y: usize) -> usize {
        let mut computer = Computer::new(&self.program);
        let output = computer.run(vec![x as i128, y as i128]);
        output[0] as usize
    }

    fn scan_line(&self, y: usize, start: usize, end: usize) -> Option<usize> {
        (start..end).map(|x| self.deploy(x, y)).position(|x| x == 1).map(|x| start + x)
    }

    fn follow_tractor(&self, x: usize, y: usize) -> IterEdge{
        IterEdge{
            drone: self.clone(),
            x: x,
            y: y
        }
    }
}

struct IterEdge{
    drone: Drone,
    x: usize,
    y: usize
}

impl Iterator for IterEdge {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let mut new_x = self.x;
        let new_y = self.y + 1;
        loop {
            let step = self.drone.deploy(new_x, new_y);
            if step == 1 { 
                self.x = new_x;
                self.y = new_y;
                break Some((self.x, self.y))
            }
            new_x += 1;
            if new_x > self.x + 100 {
                break None
            }
        }
    }
}

fn part2(program: &Program) {
    let drone = Drone::new(program);
    let start_y = 200;
    let left_edge = drone.scan_line(start_y, 0, start_y);

    let (solution_x, solution_y) = drone
        .follow_tractor(left_edge.unwrap(), start_y)
        .find(|(x, y)| {
            let fit_top_left = drone.deploy(*x, y - 99);
            let fit_top_right = drone.deploy(x + 99, y - 99);
            let fit_bottom_right = drone.deploy(x + 99, *y);

            fit_bottom_right == 1 && fit_top_left == 1 && fit_top_right == 1
        })
        .unwrap();
    println!("The solution is {}", solution_x * 10000 + solution_y - 99);
}
