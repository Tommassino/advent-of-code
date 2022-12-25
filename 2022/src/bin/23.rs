use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use advent_of_code::helpers::Point2;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Move{
    North,
    South,
    West,
    East
}

impl Move{
    pub fn iter(&self) -> impl Iterator<Item = Move> {
        let mut current = *self;
        (0..4).map(move |_| {
            let result = current.clone();
            current = match current {
                Move::North => Move::South,
                Move::South => Move::West,
                Move::West => Move::East,
                Move::East => Move::North,
            };
            result
        })
    }
}

#[derive(Clone)]
struct Grove {
    elves: HashSet<Point2<i32>>,
    direction: Move
}

impl Iterator for Grove {
    type Item = Grove;

    fn next(&mut self) -> Option<Self::Item> {
        let mut proposed_moves: HashMap<Point2<i32>, Point2<i32>> = HashMap::default();
        let mut collisions: HashMap<Point2<i32>, usize> = HashMap::default();
        self.elves.iter()
            .for_each(|elf_position| {
                if let Some(next_position) = self.propose_move(elf_position) {
                    proposed_moves.insert(*elf_position, next_position);
                    let collided = collisions.get(&next_position).unwrap_or(&0);
                    collisions.insert(next_position, collided + 1);
                } else {
                    let collided = collisions.get(elf_position).unwrap_or(&0);
                    collisions.insert(*elf_position, collided + 1);
                }
            });
        if proposed_moves.len() == 0 {
            return None;
        }
        // println!("Proposed moves: {:?}", proposed_moves);
        // println!("Collisions: {:?}", collisions);
        self.elves = self.elves
            .iter()
            .map(|original_position| {
                let next_position = proposed_moves.get(original_position)
                    .unwrap_or(original_position);
                let collided = collisions
                    .get(next_position).copied()
                    .unwrap_or_default();
                if collided <= 1 {
                    *next_position
                } else {
                    *original_position
                }
            })
            .collect();
        self.direction = self.direction.iter().nth(1).unwrap();
        Some(self.clone())
    }
}

impl Grove {
    pub fn adjacent<'a>(&'a self, elf_position: &'a Point2<i32>) -> impl Iterator<Item=Point2<i32>> + '_ {
        (-1..=1).cartesian_product(-1..=1)
            .filter(|(dx, dy)|
                *dx != 0 || *dy != 0
            )
            .map(|(dx, dy)| Point2::new(elf_position.x + dx, elf_position.y + dy))
    }

    pub fn propose_move(&self, elf_position: &Point2<i32>) -> Option<Point2<i32>> {
        let adjacent: Vec<Point2<i32>> = self.adjacent(elf_position).collect();

        let should_move = adjacent.iter()
            .map(|p| {
                // println!("Proposing moves for {:?}: adjacent? {:?}: {:?}", elf_position, p, self.elves.contains(&p));
                p
            })
            .find(|p| self.elves.contains(p))
            .is_some();

        if should_move {
            self.direction.iter().find_map(|direction|{
                let points_to_check: Vec<Point2<i32>> = adjacent
                    .iter()
                    .filter(|point| {
                        match direction {
                            Move::North => point.y < elf_position.y,
                            Move::South => point.y > elf_position.y,
                            Move::West => point.x < elf_position.x,
                            Move::East => point.x > elf_position.x
                        }
                    })
                    .cloned()
                    .collect();

                let can_move = points_to_check
                    .iter()
                    .all(|point| {
                        !self.elves.contains(point)
                    });
                if can_move {
                    // println!("Proposing to move {:?} to {:?}", elf_position, adjacent[0]);
                    points_to_check.iter().find(|p| p.x == elf_position.x || p.y == elf_position.y).cloned()
                } else {
                    None
                }
            }).or(Some(*elf_position))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String{
        let (min_x, max_x) = self.elves.iter()
            .map(|p| p.x)
            .minmax().into_option().unwrap();
        let (min_y, max_y) = self.elves.iter()
            .map(|p| p.y)
            .minmax().into_option().unwrap();
        let mut buffer: Vec<char> = Vec::default();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elves.contains(&Point2::new(x, y)) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }
        buffer.iter().collect()
    }

    pub fn iter(&self) -> Grove {
        self.clone()
    }
}

impl Debug for Grove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.to_string())
    }
}

impl FromStr for Grove {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let elves = input
            .lines().enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars().enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Point2::new(x as i32, y as i32))
            })
            .collect();
        Ok(Grove {
            elves,
            direction: Move::North
        })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grove_scan = Grove::from_str(input).expect("");
    let tenth_move = grove_scan
        .iter().take(10)
        // .enumerate().map(|(step, grove)| {
        //     println!("Step {}\nDirection: {:?}\n{:?}", step+1, grove.direction, grove);
        //     grove
        // })
        .last().unwrap();
    // println!("{:?}", tenth_move);

    let (min_x, max_x) = tenth_move.elves.iter()
        .map(|p| p.x)
        .minmax().into_option().unwrap();
    let (min_y, max_y) = tenth_move.elves.iter()
        .map(|p| p.y)
        .minmax().into_option().unwrap();
    let ground_tiles = (max_x - min_x + 1) * (max_y - min_y + 1) - tenth_move.elves.len() as i32;

    Some(ground_tiles)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grove_scan = Grove::from_str(input).expect("");
    let (step, _) = grove_scan.iter().enumerate().last().unwrap();
    Some(step+2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23, None);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23, None);
        assert_eq!(part_two(&input), Some(20));
    }

    #[test]
    fn test_steps() {
        let input = advent_of_code::read_file("examples", 23, None);
        let grove_scan = Grove::from_str(&input).expect("");
        let steps: Vec<Grove> = grove_scan.iter().take(5).collect();
        let comparison = vec![
            Grove::from_str("..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............").expect(""),
            Grove::from_str("
..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............").expect(""),
            Grove::from_str("
..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............").expect(""),
            Grove::from_str("
..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............").expect(""),
            Grove::from_str("
.......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............").expect(""),
        ];
        steps.iter().zip(comparison.iter()).enumerate().for_each(|(idx, (first, second))|{
            println!("step {}", idx+1);
            assert_eq!(first.to_string(), second.to_string());
        });
        let final_step = grove_scan.iter().nth(10).unwrap();
        let final_truth = Grove::from_str("
.......#......
...........#..
..#.#..#......
......#.......
...#.....#..#.
.#......##....
.....##.......
..#........#..
....#.#..#....
..............
....#..#..#...
..............").expect("");
        println!("step 10");
        assert_eq!(final_step.to_string(), final_truth.to_string());
    }

    #[test]
    fn test_adjacent() {
        let grove = Grove{
            elves: Default::default(),
            direction: Move::North
        };
        let adjacent: HashSet<Point2<i32>> = grove.adjacent(&Point2::new(1, 1)).collect();
        assert_eq!(adjacent.len(), 8);
    }

}
