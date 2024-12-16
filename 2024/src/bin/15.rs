use advent_of_code::helpers::Point2;
use std::fmt::{Display, Formatter};
advent_of_code::solution!(15);

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: Point2<i32>,
    instructions: Vec<Point2<i32>>,
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let (warehouse, instructions) = value.trim().split_once("\n\n").unwrap();
        let mut robot = Point2::new(0, 0);
        let grid = warehouse
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            robot = Point2::new(x as i32, y as i32);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect()
            })
            .collect();

        let instructions = instructions
            .trim()
            .chars()
            .flat_map(|c| match c {
                '^' => Some(Point2::new(0, -1)),
                'v' => Some(Point2::new(0, 1)),
                '<' => Some(Point2::new(-1, 0)),
                '>' => Some(Point2::new(1, 0)),
                _ => None,
            })
            .collect();

        Warehouse {
            grid,
            robot,
            instructions,
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Warehouse {
    fn run_instructions(&mut self) {
        for &instruction in &self.instructions {
            let next = self.robot + instruction;
            match self.grid[next.y as usize][next.x as usize] {
                '.' => {
                    self.robot = next;
                }
                'O' => {
                    // find all boxes in the direction that would be pushed
                    let mut next_box = next;
                    while self.grid[next_box.y as usize][next_box.x as usize] == 'O' {
                        next_box += instruction;
                    }
                    // if the next cell is empty, move the robot and the boxes
                    if self.grid[next_box.y as usize][next_box.x as usize] == '.' {
                        self.robot = next;
                        self.grid[next.y as usize][next.x as usize] = '.';
                        self.grid[next_box.y as usize][next_box.x as usize] = 'O';
                    }
                }
                _ => {}
            }
        }
    }

    fn to_part_2(&self) -> Warehouse {
        let grid = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .flat_map(|&c| match c {
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '.' => vec!['.', '.'],
                        '@' => vec!['@', '.'], // never should be hit
                        _ => vec![c],
                    })
                    .collect()
            })
            .collect::<Vec<_>>();
        Warehouse {
            grid,
            robot: Point2::new(self.robot.x * 2, self.robot.y),
            instructions: self.instructions.clone(),
        }
    }

    fn run_instructions_part2(&mut self) {
        for &instruction in &self.instructions {
            let next = self.robot + instruction;
            match self.grid[next.y as usize][next.x as usize] {
                '.' => {
                    self.robot = next;
                }
                part @ '[' | part @ ']' => {
                    // find all boxes in the direction that would be pushed
                    let mut boxes_to_move = vec![next];
                    // add other part of box
                    if part == '[' {
                        boxes_to_move.push(next + Point2::new(1, 0));
                    } else {
                        boxes_to_move.push(next + Point2::new(-1, 0));
                    }
                    let mut blocked = false;
                    // simple case, moving left or right - we are just skipping extra point when
                    // we hit a box
                    if instruction.x.abs() > 0 {
                        let mut next_box = next + instruction + instruction;
                        while ['[', ']']
                            .contains(&self.grid[next_box.y as usize][next_box.x as usize])
                        {
                            boxes_to_move.push(next_box);
                            next_box += instruction;
                        }
                        if self.grid[next_box.y as usize][next_box.x as usize] == '#' {
                            blocked = true;
                        }
                    } else {
                        // complicated case, since the 'width' of the push area can increase as we
                        // collect more boxes

                        // the current row of boxes we are pushing, start with just the first box
                        let mut current_boxes = boxes_to_move.clone();

                        while !current_boxes.is_empty() && !blocked {
                            let mut next_boxes = Vec::new();
                            for current_box in current_boxes {
                                let next_box = current_box + instruction;
                                match self.grid[next_box.y as usize][next_box.x as usize] {
                                    '#' => {
                                        blocked = true;
                                        break;
                                    }
                                    part @ '[' | part @ ']' => {
                                        // if it already is in the list, we have already moved it
                                        if !next_boxes.contains(&next_box) {
                                            boxes_to_move.push(next_box);
                                            next_boxes.push(next_box);
                                            if part == '[' {
                                                boxes_to_move.push(next_box + Point2::new(1, 0));
                                                next_boxes.push(next_box + Point2::new(1, 0));
                                            } else {
                                                boxes_to_move.push(next_box + Point2::new(-1, 0));
                                                next_boxes.push(next_box + Point2::new(-1, 0));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            current_boxes = next_boxes;
                        }
                    }

                    if !blocked {
                        // move all boxes in reverse order
                        for &box_pos in boxes_to_move.iter().rev() {
                            let next_box = box_pos + instruction;
                            self.grid[next_box.y as usize][next_box.x as usize] =
                                self.grid[box_pos.y as usize][box_pos.x as usize];
                            self.grid[box_pos.y as usize][box_pos.x as usize] = '.';
                        }
                        self.robot = next;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse = Warehouse::from(input);
    warehouse.run_instructions();
    // println!("{}", warehouse);
    let sum = warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| match c {
                'O' => Some(100 * y as u32 + x as u32),
                _ => None,
            })
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut warehouse = Warehouse::from(input).to_part_2();
    warehouse.run_instructions_part2();
    // println!("{}", warehouse);
    let sum = warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| match c {
                '[' => Some(100 * y as u32 + x as u32),
                _ => None,
            })
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_small_example() {
        let input = "########\n\
                     #..O.O.#\n\
                     ##@.O..#\n\
                     #...O..#\n\
                     #.#.O..#\n\
                     #...O..#\n\
                     #......#\n\
                     ########\n\
                     \n\
                     <^^>>>vv<v>>v<<";
        let mut warehouse = Warehouse::from(input);
        warehouse.run_instructions();
        println!("{}", warehouse);
    }
}
