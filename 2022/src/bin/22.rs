extern crate core;

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;
use num::integer::sqrt;

use advent_of_code::helpers::Point2;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Move(u32),
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    data: HashMap<Point2<i32>, char>,
    teleports: HashMap<Point2<i32>, (Point2<i32>, Option<Point2<i32>>)>,
    instructions: Vec<Instruction>,
    edge_size: i32,
}

#[derive(Debug, Copy, Clone)]
struct State<'a> {
    position: Point2<i32>,
    direction: Point2<i32>,
    instruction_idx: usize,
    map: &'a Map,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Edge {
    start: Point2<i32>,
    end: Point2<i32>,
    normal: Point2<i32>,
}

impl Map {
    pub fn iter(&self) -> State {
        let min_x = self.data.keys()
            .filter_map(|p| (p.y == 0).then_some(p.x))
            .min().unwrap();
        State {
            position: Point2::new(min_x, 0),
            direction: Point2::new(1, 0),
            instruction_idx: 0,
            map: self,
        }
    }

    pub fn generate_teleports(&mut self) {
        let mut row_dimensions: HashMap<i32, (i32, i32)> = HashMap::default();
        let mut col_dimensions: HashMap<i32, (i32, i32)> = HashMap::default();
        self.data.iter().for_each(|(p, _)| {
            let (min_x, max_x) = row_dimensions
                .get(&p.y).cloned()
                .unwrap_or((i32::MAX, i32::MIN));
            row_dimensions.insert(p.y, (min_x.min(p.x), max_x.max(p.x)));

            let (min_y, max_y) = col_dimensions
                .get(&p.x).cloned()
                .unwrap_or((i32::MAX, i32::MIN));
            col_dimensions.insert(p.x, (min_y.min(p.y), max_y.max(p.y)));
        });

        row_dimensions
            .iter()
            .for_each(|(y, (min_x, max_x))| {
                self.teleports.insert(
                    Point2::new(min_x - 1, *y),
                    (Point2::new(*max_x, *y), None),
                );
                self.teleports.insert(
                    Point2::new(max_x + 1, *y),
                    (Point2::new(*min_x, *y), None),
                );
            });
        col_dimensions
            .iter()
            .for_each(|(x, (min_y, max_y))| {
                self.teleports.insert(
                    Point2::new(*x, *min_y - 1),
                    (Point2::new(*x, *max_y), None),
                );
                self.teleports.insert(
                    Point2::new(*x, *max_y + 1),
                    (Point2::new(*x, *min_y), None),
                );
            });
    }

    pub fn generate_teleports_part2(&mut self) {
        let edges = self.edges();
        // println!("{:?}", edges);
        let mut ordered_edges: VecDeque<Edge> = VecDeque::default();
        ordered_edges.push_back(*edges.iter().next().unwrap());
        while ordered_edges.len() < edges.len() {
            let tail = ordered_edges.iter().last().unwrap();
            let next = edges.iter()
                .find(|x| tail.is_next(x))
                .unwrap();
            ordered_edges.push_back(*next);
        }
        let mut matching: Vec<(Edge, Edge)> = Vec::default();
        for i in 0..ordered_edges.len() {
            let first = ordered_edges[i];
            let second = ordered_edges[(i + 1) % ordered_edges.len()];
            if first.left_rotation(&second) {
                // println!("Found left turn {:?} - {:?}", first, second);
                ordered_edges.rotate_left(i+1);
                // println!("{:?}", ordered_edges);
                let mut offset = 0;
                while offset < ordered_edges.len() / 2 {
                    let first = ordered_edges[offset];
                    let second = ordered_edges[ordered_edges.len() - 1 - offset];
                    if offset > 0 {
                        if let Some((prev_first, prev_second)) = matching.last() {
                            if prev_first.right_rotation(&first) && prev_second.left_rotation(&second) {
                                // println!("Convex disconnect A {:?} <!-!> {:?}", prev_first, first);
                                // println!("Convex disconnect B {:?} <!-!> {:?}", prev_second, second);
                                break
                            }
                        }
                    }
                    // println!("Adding edge {:?} <-> {:?}", first, second);
                    offset += 1;
                    matching.push((first, second));
                }
                ordered_edges.rotate_right(i+1);
            }
        }
        // println!("Matching");
        // matching.iter().for_each(|x| println!("\t{:?}", x));

        if matching.len() * 2 != ordered_edges.len() {
            // this case happens only in a couple cases that could be solved easily,
            // but i cant be bothered
            panic!("Unmatched edges! {:?}", ordered_edges.len() as i32 - matching.len() as i32 * 2)
        }

        matching.iter()
            .for_each(|(first, second)| {
                // println!("Adding teleports for edges:");
                // println!("\t{:?}", first);
                // println!("\t{:?}", second);
                first.points().iter()
                    .zip(second.points().iter().rev())
                    .for_each(|(first_point, second_point)|{
                        // println!("Adding teleport from {:?} to {:?}", first_point, *second_point + second.normal);
                        // println!("Adding teleport from {:?} to {:?}", second_point, *first_point + first.normal);
                        self.teleports.insert(
                            *first_point,
                            (*second_point + second.normal, Some(second.normal))
                        );
                        self.teleports.insert(
                            *second_point,
                            (*first_point + first.normal, Some(first.normal))
                        );
                    });
            });
    }

    pub fn edges(&mut self) -> HashSet<Edge> {
        let mut edges: HashSet<Edge> = HashSet::new();
        self.generate_teleports();
        // add vertical edges
        self.teleports.keys()
            .filter(|p| {
                let point_above = Point2::new(p.x, p.y - 1);
                let point_below = Point2::new(p.x, p.y + 1);
                self.teleports.contains_key(&point_above) || self.teleports.contains_key(&point_below)
            })
            .for_each(|p| {
                let line_start = Point2::new(p.x, p.y - (p.y % self.edge_size));
                let line_end = Point2::new(p.x, line_start.y + self.edge_size - 1);
                let normal = if self.data.contains_key(
                    &Point2::new(line_start.x + 1, line_start.y)
                ) {
                    Point2::new(1, 0)
                } else {
                    Point2::new(-1, 0)
                };
                edges.insert(Edge::new(
                    line_start,
                    line_end,
                    normal,
                ));
            });
        // add horizontal edges
        self.teleports.keys()
            .filter(|p| {
                let point_left = Point2::new(p.x - 1, p.y);
                let point_right = Point2::new(p.x + 1, p.y);
                self.teleports.contains_key(&point_left) || self.teleports.contains_key(&point_right)
            })
            .for_each(|p| {
                let line_start = Point2::new(p.x - (p.x % self.edge_size), p.y);
                let line_end = Point2::new(line_start.x + self.edge_size - 1, p.y);
                let normal = if self.data.contains_key(
                    &Point2::new(line_start.x, line_start.y + 1)
                ) {
                    Point2::new(0, 1)
                } else {
                    Point2::new(0, -1)
                };
                edges.insert(Edge::new(
                    line_start,
                    line_end,
                    normal
                ));
            });
        self.teleports.clear();
        edges
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (map_str, instructions_str): (&str, &str) = input.split("\n\n").next_tuple().unwrap();

        let data: HashMap<Point2<i32>, char> = map_str
            .lines().enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars().enumerate()
                    .filter(|(_, c)| !c.is_whitespace())
                    .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
            })
            .collect();

        let mut number: u32 = 0;
        let mut instructions: Vec<Instruction> = Vec::default();
        for c in instructions_str.trim().chars() {
            if c == 'L' || c == 'R' {
                if number > 0 {
                    instructions.push(Instruction::Move(number));
                    number = 0;
                }
                if c == 'L' {
                    instructions.push(Instruction::Left);
                } else {
                    instructions.push(Instruction::Right);
                }
            } else {
                number = number * 10 + c.to_digit(10).unwrap();
            }
        }
        if number > 0 {
            instructions.push(Instruction::Move(number));
        }

        let edge_size = sqrt(data.len() / 6) as i32;

        Ok(Map {
            data,
            teleports: HashMap::default(),
            instructions,
            edge_size,
        })
    }
}

impl<'a> Iterator for State<'a> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instruction_idx >= self.map.instructions.len() {
            return None;
        }

        let instruction = self.map.instructions[self.instruction_idx];
        match instruction {
            Instruction::Left => {
                self.direction = Point2::new(self.direction.y, -self.direction.x);
                //println!("Rotated left {:?}", self.direction)
            }
            Instruction::Right => {
                self.direction = Point2::new(-self.direction.y, self.direction.x);
                //println!("Rotated right {:?}", self.direction)
            }
            Instruction::Move(count) => {
                for _ in 0..count {
                    let mut next_position = self.position + self.direction;
                    let mut next_direction = self.direction;
                    if let Some((new_location, new_direction)) = self.map.teleports.get(&next_position) {
                        //println!("Teleporting from {:?} to {:?} with direction {:?}", next_position, new_location, new_direction);
                        if !self.map.data.contains_key(new_location) {
                            panic!("Invalid teleport {:?} -> {:?}", next_position, new_location);
                        }
                        next_position.x = new_location.x;
                        next_position.y = new_location.y;
                        if let Some(dir) = new_direction {
                            //println!("Setting direction from teleport {:?}", dir);
                            next_direction = *dir;
                        }
                    }

                    let thing = self.map.data[&next_position];
                    //println!("Moving to {:?}: {:?}, {:?}", next_position, thing, idx);
                    if thing == '#' {
                        break;
                    }
                    self.position = next_position;
                    self.direction = next_direction;
                }
                // println!("Moved to {:?}", self.position);
            }
        }
        self.instruction_idx += 1;
        Some(*self)
    }
}

impl<'a> State<'a> {
    pub fn password(&self) -> u32 {
        let row = self.position.y + 1;
        let column = self.position.x + 1;
        let facing = match self.direction {
            Point2 { x: 1, y: 0 } => 0,
            Point2 { x: -1, y: 0 } => 2,
            Point2 { x: 0, y: 1 } => 1,
            Point2 { x: 0, y: -1 } => 3,
            _ => panic!("")
        };
        (row * 1000 + column * 4 + facing) as u32
    }
}

impl Edge {
    pub fn new(start: Point2<i32>, end: Point2<i32>, normal: Point2<i32>) -> Edge {
        let (start, end) =
            match normal {
                Point2 { x: 1, y: 0 } => (end, start), //left
                Point2 { x: -1, y: 0 } => (start, end), //right
                Point2 { x: 0, y: 1 } => (start, end), //top
                Point2 { x: 0, y: -1 } => (end, start), //bottom
                _ => panic!("")
            };
        Edge {
            start,
            end,
            normal,
        }
    }

    pub fn is_next(&self, other: &Edge) -> bool {
        let next_point = self.end - other.start;
        next_point.x.abs().max(next_point.y.abs()) <= 1
    }

    pub fn left_rotation(&self, second: &Edge) -> bool {
        second.normal.x == self.normal.y && second.normal.y == -self.normal.x
    }

    pub fn right_rotation(&self, second: &Edge) -> bool {
        second.normal.x == -self.normal.y && second.normal.y == self.normal.x
    }

    pub fn points(&self) -> Vec<Point2<i32>> {
        let step = {
            let vector = self.end - self.start;
            Point2::new(vector.x.signum(), vector.y.signum())
        };
        let mut buffer = Vec::default();
        let mut current = self.start;
        buffer.push(current);
        while current != self.end {
            current += step;
            buffer.push(current)
        }
        buffer
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).expect("");
    map.generate_teleports();
    let last = map.iter().last().unwrap();
    Some(last.password())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).expect("");
    map.generate_teleports_part2();
    let last = map.iter().last().unwrap();
    //println!("Last position {:?}, facing {:?}", last.position, last.direction);
    Some(last.password())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22, None);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22, None);
        assert_eq!(part_two(&input), Some(5031));
    }

    #[test]
    fn test_adjacent() {
        //x: -1 0 1 2 3 4
        //y     x
        //0   x . . . . x
        //1     . . . .
        //2     . . . .
        //3     . . . .
        //4     x
        let left = Edge::new(Point2::new(-1, 0), Point2::new(-1, 4), Point2 { x: 1, y: 0 });
        let top = Edge::new(Point2::new(0, -1), Point2::new(4, -1), Point2 { x: 0, y: 1 });
        let right = Edge::new(Point2::new(4, 0), Point2::new(4, 4), Point2 { x: -1, y: 0 });
        let bottom = Edge::new(Point2::new(0, 4), Point2::new(4, 4), Point2 { x: 0, y: -1 });
        assert_eq!(left.is_next(&top), true);
        assert_eq!(top.is_next(&right), true);
        assert_eq!(right.is_next(&bottom), true);
        assert_eq!(bottom.is_next(&left), true);
    }
}
