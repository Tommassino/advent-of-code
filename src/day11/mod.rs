use log::{debug, info};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

mod intcode;
use intcode::*;

#[derive(Debug)]
struct Canvas {
    paint: HashMap<(isize, isize), usize>
}

impl Canvas {
    fn new() -> Canvas {
        Canvas{
            paint: HashMap::<(isize, isize), usize>::new()
        }
    }

    fn paint(&mut self, x: isize, y: isize, color: usize) {
        self.paint.insert((x, y), color);
    }

    fn color_at(&self, x: isize, y: isize) -> usize {
        *self.paint.get(&(x, y)).unwrap_or(&0)
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = *self.paint.keys().map(|(x, _)| x).min().unwrap();
        let max_x = *self.paint.keys().map(|(x, _)| x).max().unwrap();
        let min_y = *self.paint.keys().map(|(_, y)| y).min().unwrap();
        let max_y = *self.paint.keys().map(|(_, y)| y).max().unwrap();

        let repr: String = (min_y..=max_y).map(|y| {
            let line: String = (min_x..=max_x).map(|x| {
                //char::from_digit(self.pixel(x, y), 10).unwrap()
                let value = self.color_at(x, y);
                if value == 1 {
                    '█'
                } else {
                    ' '
                }
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
struct Robot{
    x: isize,
    y: isize,
    direction: usize //0 ^, 1 >, 2 v, 3 <
}

impl Robot {
    fn new() -> Robot {
        Robot{
            x: 0,
            y: 0,
            direction: 0
        }
    }

    fn turn(&mut self, turn: usize) {
        if turn == 0 {
            self.direction = (self.direction + 3) % 4;
        } else {
            self.direction = (self.direction + 1) % 4;
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => panic!("unknown direction {}", self.direction)
        }
    }
}

pub fn solve(input_file: &str){
    let code = Program::from_file(&input_file);

    part1(&code);
    part2(&code);
}

fn part1(program: &Program) {
    let mut computer = Computer::new(&program);
    {
        let input = intcode::Stream::new();
        computer.set_input(Some(Rc::new(RefCell::new(input))));
        let output = intcode::Stream::new();
        computer.set_output(Some(Rc::new(RefCell::new(output))));
    }
    
    let mut robot = Robot::new();
    let mut canvas = Canvas::new();
    while computer.state != ComputerState::Halted {
        let color = canvas.color_at(robot.x, robot.y);
        computer.input().unwrap().borrow_mut().write(color as i128);
        computer.execute();
        let paint = computer.output().unwrap().borrow_mut().read().expect("No output from robot!") as usize;
        let turn = computer.output().unwrap().borrow_mut().read().expect("No output from robot!") as usize;
        debug!("Robot at position {:?}", robot);
        debug!("Painting with {} and turning {}", paint, turn);
        canvas.paint(robot.x, robot.y, paint);
        robot.turn(turn);
        robot.move_forward();
    }

    info!("painted panels {:?}", canvas.paint.len());
}

fn part2(program: &Program) {
    let mut computer = Computer::new(&program);
    {
        let input = intcode::Stream::new();
        computer.set_input(Some(Rc::new(RefCell::new(input))));
        let output = intcode::Stream::new();
        computer.set_output(Some(Rc::new(RefCell::new(output))));
    }
    
    let mut robot = Robot::new();
    let mut canvas = Canvas::new();
    canvas.paint(0, 0, 1);

    while computer.state != ComputerState::Halted {
        let color = canvas.color_at(robot.x, robot.y);
        computer.input().unwrap().borrow_mut().write(color as i128);
        computer.execute();
        let paint = computer.output().unwrap().borrow_mut().read().expect("No output from robot!") as usize;
        let turn = computer.output().unwrap().borrow_mut().read().expect("No output from robot!") as usize;
        debug!("Robot at position {:?}", robot);
        debug!("Painting with {} and turning {}", paint, turn);
        canvas.paint(robot.x, robot.y, paint);
        robot.turn(turn);
        robot.move_forward();
    }

    info!("\n{}", canvas);
}


#[cfg(test)]
mod tests{
    use super::*;
    
}
