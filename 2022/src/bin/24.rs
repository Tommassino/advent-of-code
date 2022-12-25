use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use advent_of_code::helpers::Point2;

use crate::Direction::{East, North, South, West};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone)]
struct Valley {
    blizzards: HashSet<(Point2<i32>, Direction)>,
    start_position: Point2<i32>,
    end_position: Point2<i32>,
    width: i32,
    height: i32,
}

impl Valley {
    pub fn occupied_after(&self, steps: i32) -> HashSet<Point2<i32>> {
        self.blizzards.iter()
            .map(|(point, direction)| {
                let vector = match direction {
                    North => Point2::new(0, -1),
                    South => Point2::new(0, 1),
                    West => Point2::new(-1, 0),
                    East => Point2::new(1, 0)
                };
                let next_position = Point2::new(
                    (point.x + vector.x * (steps + 1) - 1).rem_euclid(self.width - 2) + 1,
                    (point.y + vector.y * (steps + 1) - 1).rem_euclid(self.height - 2) + 1,
                );
                // println!("Moving blizz {:?} at {:?} -> {:?}", direction, point, next_position);
                next_position
            }).collect()
    }

    pub fn possible_moves<'a>(&'a self, position: &'a Point2<i32>, blizzards: &'a HashSet<Point2<i32>>) -> impl Iterator<Item=Point2<i32>> + '_ {
        [
            (1, 0),
            (0, 1),
            (0, -1),
            (-1, 0),
            (0, 0),
        ].iter()
            .map(|(cx, cy)| Point2::new(position.x + cx, position.y + cy))
            .filter(move |candidate| {
                let end_or_start = *candidate == self.start_position || *candidate == self.end_position;
                let out_of_bounds = candidate.x <= 0 || candidate.x >= self.width - 1 ||
                    candidate.y <= 0 || candidate.y >= self.height - 1;
                let in_blizzard = blizzards.contains(candidate);
                // println!("Checking position {:?}, {}, {}, {}", candidate, end_or_start, out_of_bounds, in_blizzard);
                end_or_start || (!out_of_bounds && !in_blizzard)
            })
    }

    pub fn find_path(&self, start_time: i32, reverse: bool) -> Option<i32> {
        let mut blizzard_positions: HashMap<i32, HashSet<Point2<i32>>> = HashMap::default();
        let mut stack: VecDeque<(Point2<i32>, i32)> = VecDeque::default();
        let mut visited: HashSet<(Point2<i32>, i32)> = HashSet::default();
        let (start_position, end_position) = if reverse {
            (self.end_position, self.start_position)
        } else {
            (self.start_position, self.end_position)
        };

        stack.push_back((start_position, start_time));
        let mut max_steps = -1;
        while let Some((current_position, steps)) = stack.pop_front() {
            if current_position == end_position {
                // println!("Found path to exit! {:?}", path);
                return Some(steps);
            }
            if steps > max_steps {
                // println!("Stack after {} minutes: {:?}, {:?}", steps, path, stack);
                max_steps = steps;
            }

            if !blizzard_positions.contains_key(&steps) {
                blizzard_positions.insert(steps, self.occupied_after(steps));
                // println!("Blizzards after {} minutes: {:?}", steps, blizzard_positions[&steps]);
            }
            self.possible_moves(&current_position, &blizzard_positions[&steps])
                .for_each(|next_position| {
                    if !visited.contains(&(next_position, steps + 1)) {
                        // println!("Moving from {:?} to {:?}", position, next_position);
                        stack.push_back((next_position, steps + 1));
                        visited.insert((next_position, steps + 1));
                    }
                });
        }
        None
    }
}

impl Debug for Valley {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr: String = (0..self.height).flat_map(|y| {
            (0..self.width + 1).map(move |x| {
                if x == self.width {
                    '\n'
                } else if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    '#'
                } else if let Some(dir) = self.blizzards.iter()
                    .find_map(|(pos, dir)| (pos.x == x && pos.y == y).then_some(dir)) {
                    match dir {
                        North => '^',
                        South => 'v',
                        West => '<',
                        East => '>'
                    }
                } else {
                    '.'
                }
            })
        }).collect();
        writeln!(f, "{}", repr)
    }
}

impl FromStr for Valley {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let blizzards: HashSet<(Point2<i32>, Direction)> = input
            .lines().enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars().enumerate()
                    .filter(|(_, c)| *c != '#' && *c != '.')
                    .map(move |(x, c)| {
                        let direction = match c {
                            '^' => North,
                            'v' => South,
                            '>' => East,
                            '<' => West,
                            _ => panic!("Unsupported blizzard {}", c)
                        };
                        (Point2::new(x as i32, y as i32), direction)
                    })
            })
            .collect();
        let start_position = input.lines().next().unwrap()
            .chars().enumerate()
            .find_map(|(x, c)| {
                (c == '.').then_some(Point2::new(x as i32, 0))
            }).unwrap();
        let end_position = input.lines().last().unwrap()
            .chars().enumerate()
            .find_map(|(x, c)| {
                (c == '.').then_some(Point2::new(x as i32, height - 1))
            }).unwrap();
        Ok(Valley {
            blizzards,
            start_position,
            end_position,
            width,
            height,
        })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let valley = Valley::from_str(input).expect("");
    valley.find_path(0, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    let valley = Valley::from_str(input).expect("");
    let first_path = valley.find_path(0, false).unwrap();
    let second_path = valley.find_path(first_path, true).unwrap();
    valley.find_path(second_path, false)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24, None);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24, None);
        assert_eq!(part_two(&input), Some(54));
    }

    #[test]
    fn test_possible_moves() {
        let input = advent_of_code::read_file("examples", 24, None);
        let valley = Valley::from_str(&input).expect("");
        let blizzards = valley.occupied_after(1);
        let possible_moves: Vec<Point2<i32>> = valley.possible_moves(&valley.start_position, &blizzards).collect();
        assert_eq!(possible_moves, vec![Point2 { x: 1, y: 1 }, Point2 { x: 1, y: 0 }]);
    }

    #[test]
    fn test_blizzards_after() {
        let input = advent_of_code::read_file("examples", 24, None);
        let valley = Valley::from_str(&input).expect("");
        let blizzards = valley.occupied_after(7);
        assert_eq!(blizzards.iter().filter(|p| p.y == 1).count(), 4);
        assert_eq!(blizzards.iter().filter(|p| p.y == 2).count(), 3);
        assert_eq!(blizzards.iter().filter(|p| p.y == 3).count(), 3);
        assert_eq!(blizzards.iter().filter(|p| p.y == 4).count(), 4);
    }

    #[test]
    fn test_example_path_possible() {
        let input = advent_of_code::read_file("examples", 24, None);
        let valley = Valley::from_str(&input).expect("");
        let example_path = [
            Point2::new(1, 0),
            Point2::new(1, 1),
            Point2::new(1, 2),
            Point2::new(1, 2),
            Point2::new(1, 1),
            Point2::new(2, 1),
            Point2::new(3, 1),
            Point2::new(3, 2),
            Point2::new(2, 2),
            Point2::new(2, 1),
            Point2::new(3, 1),
            Point2::new(3, 1),
            Point2::new(3, 2),
        ];
        example_path
            .windows(2).enumerate()
            .for_each(|(steps, moves)| {
                let blizzards = valley.occupied_after(steps as i32);
                let possible_moves: Vec<Point2<i32>> = valley.possible_moves(&moves[0], &blizzards).collect();
                // println!("Paths at {:?} from {:?}: {:?}", steps, moves[0], possible_moves);
                let move_possible = possible_moves.iter()
                    .any(|pos| *pos == moves[1]);
                assert_eq!(move_possible, true);
            });
    }
}
