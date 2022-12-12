use std::collections::{HashMap, HashSet};

use queues::{IsQueue, Queue};

use advent_of_code::helpers::Point2;

#[derive(Debug, Clone)]
struct Grid {
    data: HashMap<Point2<i32>, char>,
    start_position: Point2<i32>,
    end_position: Point2<i32>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut data: HashMap<Point2<i32>, char> = HashMap::new();
        input
            .lines().enumerate()
            .for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    data.insert(Point2::new(x as i32, y as i32), c);
                })
            });
        let start_position = *data.iter().find(|(_, c)| {
            **c == 'S'
        }).map(|x| x.0).unwrap();
        let end_position = *data.iter().find(|(_, c)| {
            **c == 'E'
        }).map(|x| x.0).unwrap();
        Grid {
            data,
            start_position,
            end_position,
        }
    }
}

pub trait PartOne<T> {
    fn find_path_part_one(&self) -> Option<u32>;
    fn neighbors(&self, position: Point2<i32>) -> Vec<Point2<i32>>;
    fn is_valid_transition(&self, position: &Point2<i32>, next: &Point2<i32>) -> bool;
}

impl PartOne<u32> for Grid {
    fn find_path_part_one(&self) -> Option<u32> {
        let mut visited: HashMap<Point2<i32>, u32> = HashMap::new();
        let mut queue: Queue<(u32, Point2<i32>)> = Queue::new();
        queue.add((0, self.start_position)).expect("");
        while let Ok((steps, position)) = queue.remove() {
            //exit early if this point was already visited
            if visited.contains_key(&position) {
                continue;
            }
            visited.insert(position, steps);
            if position == self.end_position {
                break;
            }
            let neighbors = self.neighbors(position);
            neighbors.iter().filter(|candidate| {
                !visited.contains_key(candidate)
            }).for_each(|candidate| {
                queue.add((steps + 1, *candidate)).expect("");
            });
        }
        visited.get(&self.end_position).copied()
    }

    fn neighbors(&self, position: Point2<i32>) -> Vec<Point2<i32>> {
        let mut neighbors: Vec<Point2<i32>> = Vec::new();
        neighbors.push(Point2::new(position.x + 1, position.y));
        neighbors.push(Point2::new(position.x - 1, position.y));
        neighbors.push(Point2::new(position.x, position.y + 1));
        neighbors.push(Point2::new(position.x, position.y - 1));
        neighbors.iter().filter(|x| {
            self.is_valid_transition(&position, x)
        }).copied().collect()
    }

    fn is_valid_transition(&self, position: &Point2<i32>, next: &Point2<i32>) -> bool {
        if !self.data.contains_key(position) || !self.data.contains_key(next) {
            false
        } else {
            let current_height = *self.data.get(position).unwrap();
            let next_height = *self.data.get(next).unwrap();
            if current_height == 'S' {
                next_height == 'a'
            } else if next_height == 'E' {
                current_height == 'z'
            } else {
                let diff = next_height as i32 - current_height as i32;
                diff <= 1
            }
        }
    }
}


pub trait PartTwo<T> {
    fn find_path_part_two(&self) -> Option<u32>;
    fn reverse_neighbors(&self, position: Point2<i32>) -> Vec<Point2<i32>>;
}

impl PartTwo<u32> for Grid {
    fn find_path_part_two(&self) -> Option<u32> {
        let mut to_visit: HashSet<Point2<i32>> = HashSet::new();
        self.data.iter()
            .filter(|(_, c)| **c == 'a')
            .for_each(|(pos, _)| { to_visit.insert(*pos); });
        let mut visited: HashMap<Point2<i32>, u32> = HashMap::new();
        let mut queue: Queue<(u32, Point2<i32>)> = Queue::new();
        queue.add((0, self.end_position)).expect("");
        while let Ok((steps, position)) = queue.remove() {
            //exit early if this point was already visited
            if visited.contains_key(&position) {
                continue;
            }
            visited.insert(position, steps);
            to_visit.remove(&position);
            if to_visit.is_empty() {
                break
            }
            let neighbors = self.reverse_neighbors(position);
            neighbors.iter().filter(|candidate| {
                !visited.contains_key(candidate)
            }).for_each(|candidate| {
                queue.add((steps + 1, *candidate)).expect("");
            });
        }

        self.data.iter()
            .filter(|(_, c)| **c == 'a')
            .flat_map(|(pos, _)| {
                visited.get(pos)
            })
            .min().copied()
    }

    fn reverse_neighbors(&self, position: Point2<i32>) -> Vec<Point2<i32>> {
        let mut neighbors: Vec<Point2<i32>> = Vec::new();
        neighbors.push(Point2::new(position.x + 1, position.y));
        neighbors.push(Point2::new(position.x - 1, position.y));
        neighbors.push(Point2::new(position.x, position.y + 1));
        neighbors.push(Point2::new(position.x, position.y - 1));
        neighbors.iter().filter(|x| {
            self.is_valid_transition(x, &position)
        }).copied().collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    grid.find_path_part_one()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    grid.find_path_part_two()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12, None);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12, None);
        assert_eq!(part_two(&input), Some(29));
    }
}
