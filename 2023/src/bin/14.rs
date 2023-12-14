use itertools::Itertools;
use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Block,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Block => write!(f, "#"),
            Tile::Rock => write!(f, "O"),
        }
    }
}

#[derive(Debug)]
struct ReflectorDish {
    data: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Display for ReflectorDish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for ReflectorDish {
    fn from(value: &str) -> Self {
        let data = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Block,
                        'O' => Tile::Rock,
                        _ => panic!("Invalid tile: {}", c),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();
        Self {
            data,
            width,
            height,
        }
    }
}

impl ReflectorDish {
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        for x in 0..self.width {
            let mut current_top = 0;
            for y in 0..self.height {
                match self.data[y][x] {
                    Tile::Rock => {
                        self.data[y][x] = Tile::Empty;
                        self.data[current_top][x] = Tile::Rock;
                        current_top += 1;
                    }
                    Tile::Block => {
                        current_top = y + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.width {
            let mut current_bottom = self.height - 1;
            for y in (0..self.height).rev() {
                match self.data[y][x] {
                    Tile::Rock => {
                        self.data[y][x] = Tile::Empty;
                        self.data[current_bottom][x] = Tile::Rock;
                        current_bottom = current_bottom.saturating_sub(1);
                    }
                    Tile::Block => {
                        if y > 0 {
                            current_bottom = y - 1
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.height {
            let mut current_left = 0;
            for x in 0..self.width {
                match self.data[y][x] {
                    Tile::Rock => {
                        self.data[y][x] = Tile::Empty;
                        self.data[y][current_left] = Tile::Rock;
                        current_left += 1;
                    }
                    Tile::Block => {
                        current_left = x + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height {
            let mut current_right = self.width - 1;
            for x in (0..self.width).rev() {
                match self.data[y][x] {
                    Tile::Rock => {
                        self.data[y][x] = Tile::Empty;
                        self.data[y][current_right] = Tile::Rock;
                        current_right = current_right.saturating_sub(1);
                    }
                    Tile::Block => {
                        if x > 0 {
                            current_right = x - 1
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    fn weight(&self) -> u32 {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(_, tile)| match tile {
                        Tile::Empty => 0,
                        Tile::Block => 0,
                        Tile::Rock => self.height - y,
                    })
                    .sum::<usize>() as u32
            })
            .sum::<u32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut dish = ReflectorDish::from(input);
    dish.tilt_north();
    Some(dish.weight())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dish = ReflectorDish::from(input);
    let iter = 1_000_000_000;
    let mut hashes = HashMap::new();
    hashes.insert(format!("{}", dish), 0);
    let mut i = 0;
    while i < iter {
        i += 1;
        dish.cycle();
        let hash = format!("{}", dish);
        if hashes.contains_key(&hash) {
            break;
        }
        hashes.insert(hash, i);
    }
    let hash = format!("{}", dish);
    let first = hashes.get(&hash).unwrap();
    let cycle = i - first;
    let remaining = (iter - first) % cycle;
    for _ in 0..remaining {
        dish.cycle();
    }
    Some(dish.weight())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two_first_cycles() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut dish = ReflectorDish::from(input.as_str());
        dish.cycle();
        assert_eq!(
            format!("{}", dish),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"
        );
        dish.cycle();
        assert_eq!(
            format!("{}", dish),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"
        );
        dish.cycle();
        assert_eq!(
            format!("{}", dish),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"
        );
    }
}
