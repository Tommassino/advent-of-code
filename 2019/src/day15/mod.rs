use log::{debug, info};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

mod intcode;
use intcode::*;

#[derive(Copy, Clone)]
enum Direction{
    North,
    South,
    West,
    East
}

impl Into<i128> for Direction{
    fn into(self) -> i128 {
        use Direction::*;
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4
        }
    }
}

impl From<i128> for Direction{
    fn from(value: i128) -> Direction {
        use Direction::*;
        match value {
            1 => North,
            2 => South,
            3 => West,
            4 => East,
            _ => panic!("")
        }
    }
}

impl Direction {
    fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            South => West,
            West => North,
            East => South
        }
    }

    fn turn_left(self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            South => East,
            West => South,
            East => North
        }
    }
}

#[derive(Debug)]
struct Canvas {
    paint: HashMap<(isize, isize), char>
}

impl Canvas {
    fn new() -> Canvas {
        Canvas{
            paint: HashMap::<(isize, isize), char>::new()
        }
    }

    fn paint(&mut self, x: isize, y: isize, color: char) {
        self.paint.insert((x, y), color);
    }

    fn color_at(&self, x: isize, y: isize) -> char {
        *self.paint.get(&(x, y)).unwrap_or(&' ')
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
                self.color_at(x, y)
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}

struct Robot{
    computer: Computer,
    canvas: Canvas,
    x: isize,
    y: isize,
    moves_made: usize,
    oxygen_location: Option<(isize, isize)>
}

impl Robot {
    fn new(program: &Program) -> Robot {
        let mut computer = Computer::new(&program);
        {
            let input = intcode::Stream::new();
            computer.set_input(Some(Rc::new(RefCell::new(input))));
            let output = intcode::Stream::new();
            computer.set_output(Some(Rc::new(RefCell::new(output))));
        }

        Robot {
            computer: computer,
            canvas: Canvas::new(),
            x: 0,
            y: 0,
            moves_made: 0,
            oxygen_location: None
        }
    }

    fn command(&mut self, move_command: Direction) -> isize {
        self.computer.input().unwrap().borrow_mut().write(move_command.into());
        self.computer.execute();
        let result = self.computer.output().unwrap().borrow_mut().read().unwrap();
        self.moves_made += 1;

        let (new_x, new_y) = match move_command {
            Direction::North => (self.x, self.y - 1),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
            Direction::East => (self.x + 1, self.y)
        };

        match result {
            0 => self.canvas.paint(new_x, new_y, '#'),
            1 => {
                self.x = new_x;
                self.y = new_y;
                self.canvas.paint(new_x, new_y, '.');
            }
            2 => {
                self.x = new_x;
                self.y = new_y;
                self.oxygen_location = Some((self.x, self.y));
                self.canvas.paint(new_x, new_y, 'o');
            }
            _ => panic!("Unknown result {}", result)
        }
        result as isize
    }

    fn move_to_wall(&mut self, init: Direction) {
        while self.command(init) != 0 {
            //debug!("{}", self.canvas);
        }
    }
}

pub fn solve(input_file: &str){
    let code = Program::from_file(&input_file);

    part1(&code);
}

fn part1(program: &Program) {
    use Direction::*;
    let mut robot = Robot::new(program);
    robot.move_to_wall(West);
    let starting_position = (robot.x, robot.y);
    robot.moves_made = 0;

    //follow wall until we find loop
    let mut wall_direction = West;
    loop {
        if robot.moves_made > 0 && starting_position.0 == robot.x && starting_position.1 == robot.y {
            break
        }
        let move_right = robot.command(wall_direction.turn_right());
        if move_right == 0 {
            wall_direction = wall_direction.turn_right();
            continue;
        }
        let move_parallel = robot.command(wall_direction);
        if move_parallel == 0 {
            continue;
        }
        wall_direction = wall_direction.turn_left();
    }
    println!("{}", robot.canvas);
    println!("{:?}", robot.oxygen_location);
    //we assume the oxygen gets revealed
    let oxygen_location = robot.oxygen_location.unwrap();

    {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();

        queue.push_back((0, 0, 0));
        let path_length = loop {
            let (position_x, position_y, length) = queue.pop_front().unwrap();
            if oxygen_location.0 == position_x && oxygen_location.1 == position_y {
                break length;
            }
            visited.insert((position_x, position_y));

            vec![
                (position_x - 1, position_y),
                (position_x + 1, position_y),
                (position_x, position_y - 1),
                (position_x, position_y + 1)
            ]
                .iter()
                .filter(|(x, y)| robot.canvas.color_at(*x, *y) != '#')
                .filter(|target| !visited.contains(target))
                .for_each(|(x, y)| {
                    queue.push_back((*x, *y, length + 1));
                });
        };
        
        println!("Path length {}", path_length);
    }

    
    {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();
        let mut minutes = 0; 

        queue.push_back((oxygen_location.0, oxygen_location.1, 0));
        while !queue.is_empty() {
            let (position_x, position_y, length) = queue.pop_front().unwrap();
            visited.insert((position_x, position_y));
            minutes = length;

            vec![
                (position_x - 1, position_y),
                (position_x + 1, position_y),
                (position_x, position_y - 1),
                (position_x, position_y + 1)
            ]
                .iter()
                .filter(|(x, y)| robot.canvas.color_at(*x, *y) == '.')
                .filter(|target| !visited.contains(target))
                .for_each(|(x, y)| {
                    queue.push_back((*x, *y, length + 1));
                });
        }

        println!("Time to fill up with oxygen {} minutes", minutes);
    }
    


}


#[cfg(test)]
mod tests{
    use super::*;
    
}
