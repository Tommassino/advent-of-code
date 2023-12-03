advent_of_code::solution!(3);

use advent_of_code::helpers::Point2;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let data: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let width = data[0].len();
        let height = data.len();
        Schematic {
            width,
            height,
            data,
        }
    }
}

#[derive(Debug)]
struct Number {
    number: u32,
    points: HashSet<Point2<usize>>,
}

impl Number {
    fn contains(&self, point: &Point2<usize>) -> bool {
        self.points.contains(point)
    }

    fn neighbors(&self, schematic: &Schematic) -> Vec<Point2<usize>> {
        let mut neighbors = Vec::new();
        for point in &self.points {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = point.x as i32 + dx;
                    let ny = point.y as i32 + dy;
                    if nx < 0
                        || ny < 0
                        || nx >= schematic.width as i32
                        || ny >= schematic.height as i32
                    {
                        continue;
                    }
                    let neighbor = Point2::new(nx as usize, ny as usize);
                    if !self.contains(&neighbor) {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }

    pub fn is_part_number(&self, schematic: &Schematic) -> bool {
        let mut neighbors = self.neighbors(schematic);
        neighbors.retain(|point| {
            let c = schematic.data[point.y][point.x];
            !c.is_ascii_digit() && c != '.'
        });
        !neighbors.is_empty()
    }

    pub fn gear_coordinate(&self, schematic: &Schematic) -> Option<Point2<usize>> {
        let mut neighbors = self.neighbors(schematic);
        neighbors.retain(|point| {
            let c = schematic.data[point.y][point.x];
            c == '*'
        });
        neighbors.first().copied()
    }
}

#[derive(Debug)]
struct PartNumbers {
    numbers: Vec<Number>,
}

impl From<&Schematic> for PartNumbers {
    fn from(schematic: &Schematic) -> Self {
        let mut numbers = Vec::new();
        for y in 0..schematic.height {
            let mut number: Number = Number {
                number: 0,
                points: HashSet::new(),
            };
            for x in 0..schematic.width {
                let c = schematic.data[y][x];
                if c.is_ascii_digit() {
                    let n = c.to_digit(10).unwrap();
                    number.number = number.number * 10 + n;
                    number.points.insert(Point2::new(x, y));
                } else if number.number > 0 {
                    if number.is_part_number(schematic) {
                        numbers.push(number);
                    }
                    number = Number {
                        number: 0,
                        points: HashSet::new(),
                    };
                }
            }
            if number.number > 0 && number.is_part_number(schematic) {
                numbers.push(number);
            }
        }
        PartNumbers { numbers }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from(input);
    let part_numbers = PartNumbers::from(&schematic);
    let result = part_numbers.numbers.iter().map(|x| x.number).sum();
    Some(result)
}

#[derive(Debug)]
struct GearNumbers {
    numbers: Vec<(Number, Point2<usize>)>,
}

impl From<&Schematic> for crate::GearNumbers {
    fn from(schematic: &Schematic) -> Self {
        let mut numbers = Vec::new();
        for y in 0..schematic.height {
            let mut number: Number = Number {
                number: 0,
                points: HashSet::new(),
            };
            for x in 0..schematic.width {
                let c = schematic.data[y][x];
                if c.is_ascii_digit() {
                    let n = c.to_digit(10).unwrap();
                    number.number = number.number * 10 + n;
                    number.points.insert(Point2::new(x, y));
                } else if number.number > 0 {
                    let gear = number.gear_coordinate(schematic);
                    if let Some(g) = gear {
                        numbers.push((number, g));
                    }
                    number = Number {
                        number: 0,
                        points: HashSet::new(),
                    };
                }
            }
            if number.number > 0 {
                let gear = number.gear_coordinate(schematic);
                if let Some(g) = gear {
                    numbers.push((number, g));
                }
            }
        }
        crate::GearNumbers { numbers }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = Schematic::from(input);
    let gear_numbers = GearNumbers::from(&schematic);
    let mut gears = HashMap::new();
    gear_numbers.numbers.iter().for_each(|(number, gear)| {
        if gears.contains_key(gear) {
            let gear_numbers: &mut Vec<&Number> = gears.get_mut(gear).unwrap();
            gear_numbers.push(number);
        } else {
            gears.insert(gear, vec![number]);
        }
    });
    let result = gears
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.iter().map(|number| number.number).product::<u32>())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
