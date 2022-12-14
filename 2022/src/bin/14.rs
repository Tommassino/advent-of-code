use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use advent_of_code::helpers::Point2;
use itertools::Itertools;
use num::signum;

#[derive(Clone)]
struct Grid{
    rocks: HashSet<Point2<i32>>,
    sand: HashSet<Point2<i32>>,
    bottom: i32
}


impl Grid{
    pub fn fall_sand(&mut self) -> bool {
        let mut sand = Point2::new(500, 0);
        while sand.y < self.bottom {
            let candidates =
                vec![
                    sand + Point2::new(0, 1),
                    sand + Point2::new(-1, 1),
                    sand + Point2::new(1, 1)
                ];
            let maybe_resolved = candidates.iter().find(|candidate| {
                !self.rocks.contains(candidate)
                    && !self.sand.contains(candidate)
            });
            if let Some(resolved) = maybe_resolved {
                sand = *resolved;
            } else {
                break
            }
        }
        if sand.y >= self.bottom  || (sand.x == 500 && sand.y == 0) {
            // println!("Exiting with sand at rest {:?}. Bottom {}", sand, self.bottom);
            false
        } else {
            self.sand.insert(sand);
            true
        }
    }
}


impl Debug for Grid{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self.rocks.iter().map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self.rocks.iter().map(|p| p.y)
            .minmax()
            .into_option()
            .unwrap();

        let mut buffer = Vec::new();
        for y in 0..=max_y+1 {
            for x in min_x-1..=max_x+1 {
                let point = Point2::new(x, y);
                let char = {
                    if x == 500 && y == 0{
                        'x'
                    } else if self.rocks.contains(&point) {
                        '#'
                    } else if self.sand.contains(&point) {
                        'o'
                    } else {
                        '.'
                    }
                };
                buffer.push(char);
            }
            buffer.push('\n');
        }
        let s: String = buffer.into_iter().collect();
        writeln!(f, "{}", s)
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks: HashSet<Point2<i32>> = HashSet::new();
        s.lines().for_each(|line| {
            let points: Vec<Point2<i32>> = line
                .split(" -> ")
                .map(|point| {
                    let (x, y) = point
                        .split(',')
                        .map(|n| n.parse::<i32>().expect(""))
                        .next_tuple()
                        .unwrap();
                    Point2::new(x, y)
                })
                .collect();
            points
                .windows(2)
                .for_each(|point_line| {
                    let vector = point_line[1] - point_line[0];
                    let step = Point2::new(
                        signum(vector.x),
                        signum(vector.y)
                    );
                    let mut point = point_line[0];
                    while point != point_line[1] {
                        rocks.insert(point);
                        point += step;
                    }
                    rocks.insert(point_line[1]);
                });
        });

        let bottom = rocks.iter().map(|p| p.y).max().unwrap();

        Ok(Grid{
            rocks,
            sand: Default::default(),
            bottom
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::from_str(input).expect("");
    // println!("{:?}", grid);
    while grid.fall_sand() {
        // println!("{:?}", grid);
    };
    // println!("{:?}", grid);
    Some(grid.sand.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::from_str(input).expect("");

    // create the bottom floor
    grid.bottom+=2;
    let start_x = 500 - grid.bottom;
    let end_x = 500 + grid.bottom;
    for x in start_x..=end_x {
        grid.rocks.insert(Point2::new(x, grid.bottom));
    }

    // println!("{:?}", grid);
    while grid.fall_sand(){
        // println!("{:?}", grid);
    }
    // println!("{:?}", grid);
    Some(grid.sand.len() + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14, None);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14, None);
        assert_eq!(part_two(&input), Some(93));
    }
}
