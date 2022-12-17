extern crate core;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use advent_of_code::helpers::Point2;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Movement {
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Block {
    points: Vec<Point2<i32>>,
    height: i32
}

impl Block {
    pub fn new(points: Vec<Point2<i32>>) -> Block {
        let max_y = points.iter().map(|p| p.y)
            .max().unwrap();
        Block {
            points,
            height: max_y
        }
    }

    pub fn blocks() -> Vec<Block> {
        let line_block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(3, 0),
        ]);
        let plus_block = Block::new(vec![
            Point2::new(1, 0),
            Point2::new(0, 1),
            Point2::new(1, 1),
            Point2::new(2, 1),
            Point2::new(1, 2),
        ]);
        let l_block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(2, 1),
            Point2::new(2, 2),
        ]);
        let col_block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(0, 1),
            Point2::new(0, 2),
            Point2::new(0, 3)
        ]);
        let box_block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(0, 1),
            Point2::new(1, 1)
        ]);
        vec![
            line_block,
            plus_block,
            l_block,
            col_block,
            box_block
        ]
    }
}

#[derive(Clone)]
struct Chamber {
    materialized: Vec<[bool; 7]>,
    jet_stream: Vec<Movement>,
    time: usize,
}

impl FromStr for Chamber {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let jet_stream = input.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                match c {
                    '<' => Movement::Left,
                    '>' => Movement::Right,
                    _ => panic!("Unknown character {}", c)
                }
            }).collect();
        Ok(Chamber {
            materialized: vec![],
            jet_stream,
            time: 0,
        })
    }
}

impl Debug for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self.materialized.iter().rev()
            .flat_map(|line| {
                let mut line_vec: Vec<char> = line.iter().map(|x| {
                    if *x {
                        '#'
                    } else {
                        '.'
                    }
                }).collect();
                line_vec.push('\n');
                line_vec
            }).into_iter().collect();
        writeln!(f, "{}", s)
    }
}


impl Chamber {
    pub fn materialize(
        &mut self,
        block: &Block,
        position: &Point2<i32>,
    ) {
        let top_row = block.height + position.y;
        while self.materialized.len() <= top_row as usize {
            self.materialized.push([false; 7]);
        }
        block.points.iter().for_each(|point| {
            let placed_point = *point + *position;
            self.materialized[placed_point.y as usize][placed_point.x as usize] = true;
        });
    }

    pub fn drop_block(
        &mut self,
        block: &Block,
    ) {
        let mut block_position: Point2<i32> = Point2::new(
            2,
            self.materialized.len() as i32 + 3,
        );

        loop {
            let movement = self.jet_stream[self.time];
            self.time = (self.time + 1) % self.jet_stream.len();
            if !self.has_collision(block, &block_position, movement) {
                // println!("Jet of gas pushes rock {:?}", movement);
                block_position.x += if movement == Movement::Left { -1 } else { 1 };
            } else {
                // println!("Jet of gas pushes rock {:?}, but nothing happens", movement);
            }
            if !self.has_collision(block, &block_position, Movement::Down) {
                // println!("Rock falls 1 unit");
                block_position.y -= 1;
            } else {
                // println!("Rock falls 1 unit, causing it to come to rest");
                break
            }
        }
        self.materialize(block, &block_position);
    }

    pub fn has_collision(
        &self,
        block: &Block,
        block_position: &Point2<i32>,
        movement: Movement,
    ) -> bool {
        let new_position = match movement {
            Movement::Down => Point2::new(block_position.x, block_position.y - 1),
            Movement::Left => Point2::new(block_position.x - 1, block_position.y),
            Movement::Right => Point2::new(block_position.x + 1, block_position.y),
        };
        block.points.iter().any(|point| {
            let point_position = *point + new_position;
            if point_position.x < 0 || point_position.x >= 7 || point_position.y < 0 {
                // collision with walls
                true
            } else if point_position.y < self.materialized.len() as i32 {
                // collision with materialized block
                self.materialized[point_position.y as usize][point_position.x as usize]
            } else {
                false
            }
        })
    }

    pub fn get_block_snapshot(&self) -> Vec<[bool; 7]> {
        let mut top_view = [false; 7];
        let mut state: Vec<[bool; 7]> = Vec::new();
        for row in self.materialized.iter().rev() {
            state.push(*row);
            row.iter().enumerate()
                .for_each(|(idx, val)| top_view[idx] |= val);
            if row.iter().all(|x| x.eq(&true)) {
                break
            }
        }
        state
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut chamber = Chamber::from_str(input).expect("");
    Block::blocks().iter().cycle().take(2022).for_each(|block| {
        chamber.drop_block(block);
    });
    Some(chamber.materialized.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let round_count: usize = 1_000_000_000_000;
    let mut chamber = Chamber::from_str(input).expect("");
    let blocks = Block::blocks();

    type State = (Vec<[bool; 7]>, usize, usize);
    let mut states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut idx = 0;
    let mut state = (Vec::new(), 0, 0);

    for i in 0.. {
        let block = blocks.get(i % blocks.len()).unwrap();
        chamber.drop_block(block);

        // this shapshot is used to approximately represent the state of the top of the cave
        let block_snapshot = chamber.get_block_snapshot();
        let cur_state = (block_snapshot, i % blocks.len(), chamber.time);
        if states.contains_key(&cur_state) {
            idx = i;
            state = cur_state;
            break
        }
        states.insert(cur_state, (i, chamber.materialized.len()));
    }

    let cycle_start = states[&state];
    let cycle_end = (idx, chamber.materialized.len());
    let cycle_height = cycle_end.1 - cycle_start.1;
    let cycle_length = cycle_end.0 - cycle_start.0;

    let cycle_offset = (round_count - cycle_start.0) % cycle_length;
    for i in (cycle_end.0 + 1)..(cycle_end.0 + cycle_offset) {
        let block = blocks.get(i % blocks.len()).unwrap();
        chamber.drop_block(block);
    }
    let cycle_count = (round_count - (cycle_start.0 + cycle_offset)) / cycle_length;
    Some(cycle_height * (cycle_count - 1) + chamber.materialized.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17, None);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17, None);
        assert_eq!(part_two(&input), Some(1514285714288));
    }

    #[test]
    fn test_drop_block() {
        let mut chamber = Chamber::from_str("<><><>").expect("");
        chamber.materialized.push([false, false, true, false, false, false, false]);
        let block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(3, 0),
        ]);
        println!("{:?}", chamber);
        chamber.drop_block(&block);
        println!("{:?}", chamber);
    }

    #[test]
    fn test_materialize() {
        let mut chamber = Chamber::from_str("<><><>").expect("");
        chamber.materialized.push([false, false, true, false, false, false, false]);
        let block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(3, 0),
        ]);
        println!("{:?}", chamber);
        chamber.materialize(&block, &Point2::new(2, 1));
        println!("{:?}", chamber);
    }

    #[test]
    fn test_has_collision() {
        let mut chamber = Chamber::from_str("<><><>").expect("");
        chamber.materialized.push([false, false, true, false, false, false, false]);
        let block = Block::new(vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(3, 0),
        ]);
        assert_eq!(chamber.has_collision(
            &block,
            &Point2::new(2, 1),
            Movement::Down,
        ), true);
        assert_eq!(chamber.has_collision(
            &block,
            &Point2::new(2, 2),
            Movement::Down,
        ), false);
        assert_eq!(chamber.has_collision(
            &block,
            &Point2::new(3, 1),
            Movement::Down,
        ), false);
        assert_eq!(chamber.has_collision(
            &block,
            &Point2::new(3, 0),
            Movement::Left,
        ), true);
        assert_eq!(chamber.has_collision(
            &block,
            &Point2::new(3, 0),
            Movement::Down,
        ), true);
    }
}
