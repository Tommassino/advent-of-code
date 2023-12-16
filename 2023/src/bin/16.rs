use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

struct Contraption {
    tiles: Vec<Vec<Tile>>,
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '/' => Tile::MirrorForward,
                        '\\' => Tile::MirrorBackward,
                        '|' => Tile::SplitterVertical,
                        '-' => Tile::SplitterHorizontal,
                        _ => panic!("Invalid tile: {}", c),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Self { tiles }
    }
}

impl Contraption {
    fn energize(&self, initial_position: (i32, i32), initial_direction: (i32, i32)) -> usize {
        let mut energized_tiles = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((initial_position, initial_direction));
        let height = self.tiles.len() as i32;
        let width = self.tiles[0].len() as i32;
        while let Some((position, direction)) = queue.pop_front() {
            if visited.contains(&(position, direction)) {
                continue;
            }
            visited.insert((position, direction));
            let (x, y) = position;
            if x >= 0 && y >= 0 && x < width && y < height {
                energized_tiles.insert(position);
            }
            let (dx, dy) = direction;
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                continue;
            }
            let tile = self.tiles[ny as usize][nx as usize];
            match tile {
                Tile::Empty => {
                    queue.push_back(((nx, ny), (dx, dy)));
                }
                Tile::MirrorForward => {
                    queue.push_back(((nx, ny), (-dy, -dx)));
                }
                Tile::MirrorBackward => {
                    queue.push_back(((nx, ny), (dy, dx)));
                }
                Tile::SplitterHorizontal => {
                    if dx == 0 {
                        queue.push_back(((nx, ny), (1, 0)));
                        queue.push_back(((nx, ny), (-1, 0)));
                    } else {
                        queue.push_back(((nx, ny), (dx, dy)));
                    }
                }
                Tile::SplitterVertical => {
                    if dy == 0 {
                        queue.push_back(((nx, ny), (0, 1)));
                        queue.push_back(((nx, ny), (0, -1)));
                    } else {
                        queue.push_back(((nx, ny), (dx, dy)));
                    }
                }
            }
        }

        energized_tiles.len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let contraption = Contraption::from(input);
    Some(contraption.energize((-1, 0), (1, 0)) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let contraption = Contraption::from(input);
    let width = contraption.tiles[0].len() as i32;
    let height = contraption.tiles.len() as i32;
    let mut starts = Vec::new();
    for x in 0..width {
        starts.push(((x, -1), (0, 1)));
        starts.push(((x, height), (0, -1)));
    }
    for y in 0..height {
        starts.push(((-1, y), (1, 0)));
        starts.push(((width, y), (-1, 0)));
    }
    let max_energized = starts
        .par_iter()
        .map(|(position, direction)| contraption.energize(*position, *direction) as u32)
        .max();
    max_energized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
