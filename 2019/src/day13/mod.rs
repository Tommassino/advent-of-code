use log::{debug, info};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

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
                match value {
                    0 => ' ',
                    1 => '█',
                    2 => '■',
                    3 => '=',
                    4 => '●',
                    _ => panic!("Unknown character to paint {}", value)
                }
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}

struct Game{
    computer: Computer,
    canvas: Canvas,
    score: usize,
    paddle_position: isize,
    ball_position: isize
}

impl Game {
    fn new(program: &Program) -> Game {
        let mut computer = Computer::new(&program);
        {
            let input = intcode::Stream::new();
            computer.set_input(Some(Rc::new(RefCell::new(input))));
            let output = intcode::Stream::new();
            computer.set_output(Some(Rc::new(RefCell::new(output))));
        }
        //infinite coins
        computer.memory[0] = 2;

        let mut game = Game {
            computer: computer,
            canvas: Canvas::new(),
            score: 0,
            paddle_position: 0,
            ball_position: 0
        };
        game.tick(0);
        game
    }

    fn is_running(&self) -> bool {
        self.computer.state != ComputerState::Halted
    }

    fn tick(&mut self, paddle_move: isize) {
        self.computer.input().unwrap().borrow_mut().write(paddle_move as i128);
        self.computer.execute();
        let mut output = Vec::<i128>::new();
        loop {
            let pop = self.computer.output().unwrap().borrow_mut().read();
            if pop.is_some() {
                output.push(pop.unwrap());
            } else {
                break;
            }
        }
        for chunk in output.chunks(3) {
            assert_eq!(chunk.len(), 3);
            let x = chunk[0] as isize;
            let y = chunk[1] as isize;
            let data = chunk[2] as usize;
            if x == -1 {
                self.score = data;
            } else {
                self.canvas.paint(x, y, data);
                match data {
                    3 => self.paddle_position = x,
                    4 => self.ball_position = x,
                    _ => {}
                }
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\nSCORE:{}\n", self.canvas, self.score)
    }
}

pub fn solve(input_file: &str){
    let code = Program::from_file(&input_file);

    part1(&code);
    part2(&code);
}

fn part1(program: &Program) {
    let game = Game::new(program);
    info!("\n{}", game);
    println!("Block count {:?}", game.canvas.paint.values().filter(|x| **x == 2).count());
}

fn part2(program: &Program) {
    let mut game = Game::new(program);
    while game.is_running() {
        info!("{}", game);
        let paddle_move = (game.ball_position - game.paddle_position).signum();
        game.tick(paddle_move);
        //sleep(Duration::from_millis(100));
    }
    println!("Final score is: {}", game.score);
}


#[cfg(test)]
mod tests{
    use super::*;
    
}
