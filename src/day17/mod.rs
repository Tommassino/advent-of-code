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
use std::char;
use std::str::FromStr;
use std::convert::Infallible;
use std::ops::Add;
use std::ops::Sub;

mod intcode;
use intcode::*;

#[derive(Debug, Clone, Copy)]
struct Point{
    x: isize,
    y: isize
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point{
    fn len(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction{
    North,
    South,
    West,
    East
}

impl Into<Point> for Direction{
    fn into(self) -> Point {
        use Direction::*;
        match self {
            North => Point{x: 0, y: -1},
            South => Point{x: 0, y: 1},
            West => Point{x: -1, y: 0},
            East => Point{x: 1, y: 0}
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

    fn color_at_point(&self, point: Point) -> char {
        self.color_at(point.x, point.y)
    }
}

impl FromStr for Canvas {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut canvas = Canvas{paint: HashMap::new()};
        let mut x = 0;
        let mut y = 0;
        input.chars().for_each(|c|{
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                canvas.paint(x, y, c);
                x += 1;
            }
        });
        Ok(canvas)
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
    position_x: isize,
    position_y: isize
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
            position_x: 0,
            position_y: 0
        }
    }

    fn scan(&mut self) {
        let output = self.computer.run(vec![]);

        let mut x = 0;
        let mut y = 0;
        output.iter().for_each(|c|{
            if *c == 10 {
                x = 0;
                y += 1;
            } else {
                let point = char::from_u32(*c as u32).unwrap();
                self.canvas.paint(x, y, point);
                if point == '^' || point == '<' || point == '>' || point == 'v' {
                    self.position_x = x;
                    self.position_y = y;
                }
                x += 1;
            }
        });
    }
}

pub fn solve(input_file: &str){
    let code = Program::from_file(&input_file);

    //part1(&code);
    part2(&code);
}

fn part1(program: &Program) {
    let mut robot = Robot::new(program);
    robot.scan();

    let alignment: isize = compute_alignment(&robot.canvas);

    println!("{}", robot.canvas);
    println!("Alignment: {}", alignment);
}

fn part2(program: &Program) {
    let mut robot = Robot::new(program);
    robot.computer.memory[0] = 2;
    robot.scan();

    let path = generate_path(&robot.canvas, robot.position_x, robot.position_y);
    let path_command: String = path.moves.iter().map(|x| format!("{}", x.command())).collect::<Vec<String>>().join(",");
    println!("{}", path_command);

    let robot_input = r#"A,B,A,C,A,A,C,B,C,B
L,12,L,8,R,12
L,10,L,8,L,12,R,12
R,12,L,8,L,10
n
"#;

    let robot_input_ints: Vec<i128> = robot_input.chars().map(|c| c as i128).collect();
    let output = robot.computer.run(robot_input_ints);
    let output_string: String = output.iter().map(|x| char::from_u32(*x as u32).unwrap_or(' ')).collect();
    println!("{}", output_string);
    println!("{}", output.last().unwrap());
}

fn compute_alignment(canvas: &Canvas) -> isize {
    canvas.paint.keys().flat_map(|(x, y)| {
        let adjacent = vec![
            canvas.color_at(x - 1, *y) == '#',
            canvas.color_at(x + 1, *y) == '#',
            canvas.color_at(*x, y - 1) == '#',
            canvas.color_at(*x, y + 1) == '#'
        ].iter().filter(|c| **c).count();
        if canvas.color_at(*x, *y) == '#' && adjacent >= 3 {
            debug!("Intersection at {} {}", x, y);
            Some(x * y)
        } else {
            None
        }
    }).sum()
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Forward(usize),
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Path{
    moves: Vec<Move>
}

impl Move {
    fn command(&self) -> String {
        use Move::*;
        match self {
            Forward(count) => count.to_string(),
            Left => String::from("L"),
            Right => String::from("R")
        }
    }
}

fn generate_path(canvas: &Canvas, robot_x: isize, robot_y: isize) -> Path {
    let mut last_turn = Point{x: robot_x, y: robot_y};
    let mut position = Point{x: robot_x, y: robot_y};
    //would have to change this for a general input
    let mut direction = Direction::West;
    let mut path: Vec<Move> = Vec::new();
    path.push(Move::Left);
    loop {
        if canvas.color_at_point(position + direction.into()) == '#' {
            position = position + direction.into();
        } else {
            path.push(Move::Forward((position - last_turn).len()));
            last_turn = position;
            let left_turn = canvas.color_at_point(position + direction.turn_left().into());
            let right_turn = canvas.color_at_point(position + direction.turn_right().into());
            if left_turn == '#' {
                path.push(Move::Left);
                direction = direction.turn_left();
            } else if right_turn == '#' {
                path.push(Move::Right);
                direction = direction.turn_right();
            } else {
                debug!("Dead end at {:?}", position);
                break Path{moves: path}
            }
            debug!("Going {:?} from {:?}", direction, position);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn name() {
        let input = r#"
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."#.trim();
        let canvas = Canvas::from_str(input).unwrap();
        println!("{}", canvas);
        println!("{}", compute_alignment(&canvas));
    }
}
