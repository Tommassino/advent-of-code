use advent_of_code::helpers::Point2;
use std::marker::PhantomData;
use std::usize;
advent_of_code::solution!(18);

#[derive(Debug)]
struct Part1 {}
#[derive(Debug)]
struct Part2 {}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug)]
struct Instruction<Part> {
    direction: Direction,
    distance: usize,
    phantom: PhantomData<Part>,
}

impl From<&str> for Instruction<Part1> {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = parts.next().unwrap().parse::<usize>().unwrap();
        Self {
            direction,
            distance,
            phantom: PhantomData,
        }
    }
}

impl From<&str> for Instruction<Part2> {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        parts.next();
        parts.next();
        let hex = parts.next().unwrap()[2..8].to_string();
        let distance_hex = hex[0..5].to_string();
        let distance = usize::from_str_radix(&distance_hex, 16).unwrap();
        let direction = match hex.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction"),
        };
        Self {
            direction,
            distance,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
struct CityBlocks {
    perimeter: Vec<Point2<i64>>,
}

impl<Part> From<&Vec<Instruction<Part>>> for CityBlocks {
    fn from(value: &Vec<Instruction<Part>>) -> Self {
        let mut position: Point2<i64> = Point2::new(0, 0);
        let mut perimeter = Vec::new();
        for instruction in value {
            perimeter.push(position);
            match instruction.direction {
                Direction::Up => position.y -= instruction.distance as i64,
                Direction::Down => position.y += instruction.distance as i64,
                Direction::Left => position.x -= instruction.distance as i64,
                Direction::Right => position.x += instruction.distance as i64,
            }
        }
        perimeter.push(position);
        Self { perimeter }
    }
}

impl CityBlocks {
    fn area(&self) -> usize {
        let mut area = 0;
        let mut perimeter = 0;
        for i in 0..self.perimeter.len() {
            let a = self.perimeter[i];
            let b = self.perimeter[(i + 1) % self.perimeter.len()];
            area += a.x * b.y;
            area -= a.y * b.x;
            perimeter += (a.x - b.x).abs() + (a.y - b.y).abs();
        }
        (area.abs() / 2) as usize + (perimeter as usize) / 2 + 1
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions: Vec<Instruction<Part1>> = input.lines().map(|line| line.into()).collect();
    let city_blocks = CityBlocks::from(&instructions);
    Some(city_blocks.area())
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions: Vec<Instruction<Part2>> = input.lines().map(|line| line.into()).collect();
    let city_blocks = CityBlocks::from(&instructions);
    Some(city_blocks.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
