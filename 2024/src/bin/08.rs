use advent_of_code::helpers::Point2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

#[derive(Debug)]
struct Input {
    antennas: HashMap<char, Vec<Point2<i32>>>,
    width: i32,
    height: i32,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut antennas = HashMap::new();
        let width = value.lines().next().unwrap().len() as i32;
        let height = value.lines().count() as i32;
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Point2::new(x as i32, y as i32));
                }
            }
        }
        Input {
            antennas,
            width,
            height,
        }
    }
}

impl Input {
    fn antinodes(&self) -> HashSet<Point2<i32>> {
        let mut antinodes = HashSet::new();
        for (_, positions) in self.antennas.iter() {
            positions
                .iter()
                .tuple_combinations::<(_, _)>()
                .for_each(|(&first, &second)| {
                    let vector = first - second;
                    let antinode_one = first + vector;
                    if self.in_grid(&antinode_one) {
                        antinodes.insert(antinode_one);
                    }
                    let antinode_two = second - vector;
                    if self.in_grid(&antinode_two) {
                        antinodes.insert(antinode_two);
                    }
                })
        }
        antinodes
    }

    fn resonant_antinodes(&self) -> HashSet<Point2<i32>> {
        let mut antinodes = HashSet::new();
        for (_, positions) in self.antennas.iter() {
            for &position in positions {
                antinodes.insert(position);
            }
            positions
                .iter()
                .tuple_combinations::<(_, _)>()
                .for_each(|(&first, &second)| {
                    let vector = first - second;
                    let mut position = first;
                    loop {
                        position += vector;
                        if !self.in_grid(&position) {
                            break;
                        }
                        antinodes.insert(position);
                    }
                    position = first;
                    loop {
                        position -= vector;
                        if !self.in_grid(&position) {
                            break;
                        }
                        antinodes.insert(position);
                    }
                })
        }
        antinodes
    }

    fn in_grid(&self, position: &Point2<i32>) -> bool {
        self.width > position.x && position.x >= 0 && self.height > position.y && position.y >= 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::from(input);
    let antinodes = input.antinodes();
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::from(input);
    let antinodes = input.resonant_antinodes();
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
