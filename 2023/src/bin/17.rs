use std::cmp::Ordering;
use std::collections::BinaryHeap;
advent_of_code::solution!(17);

struct CityBlocks {
    blocks: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn move_by(
        self,
        x: usize,
        y: usize,
        distance: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => y.checked_sub(distance).map(|ny| (x, ny)),
            Direction::Down => (y + distance < height).then_some((x, y + distance)),
            Direction::Left => x.checked_sub(distance).map(|nx| (nx, y)),
            Direction::Right => (x + distance < width).then_some((x + distance, y)),
        }
    }

    fn next(self) -> [Self; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

impl From<&str> for CityBlocks {
    fn from(value: &str) -> Self {
        let blocks: Vec<Vec<u32>> = value
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let height = blocks.len();
        let width = blocks[0].len();
        Self {
            blocks,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    heat: u32,
    x: usize,
    y: usize,
    previous_direction: Direction,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.heat.cmp(&self.heat))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl State {
    fn new(heat: u32, x: usize, y: usize, previous_direction: Direction) -> Self {
        Self {
            heat,
            x,
            y,
            previous_direction,
        }
    }
}

impl CityBlocks {
    fn best_path(&self, min_moves: usize, max_moves: usize) -> u32 {
        let mut frontier: BinaryHeap<State> = BinaryHeap::new();
        frontier.push(State::new(0, 0, 0, Direction::Right));
        frontier.push(State::new(0, 0, 0, Direction::Down));
        let mut best_paths = vec![vec![vec![u32::MAX; 4]; self.width]; self.height];
        best_paths[0][0][Direction::Right as usize] = 0;
        best_paths[0][0][Direction::Down as usize] = 0;
        while let Some(State {
            heat: _,
            x,
            y,
            previous_direction,
        }) = frontier.pop()
        {
            let heat = best_paths[y][x][previous_direction as usize];
            if x == (self.width - 1) && y == (self.height - 1) {
                return heat;
            }
            let next_directions = previous_direction.next();
            for next_direction in next_directions.iter() {
                for distance in min_moves..=max_moves {
                    let Some((next_x, next_y)) =
                        next_direction.move_by(x, y, distance, self.width, self.height)
                    else {
                        continue;
                    };
                    let mut move_heat = 0;
                    for i in 1..=distance {
                        let (move_x, move_y) = next_direction
                            .move_by(x, y, i, self.width, self.height)
                            .unwrap();
                        move_heat += self.blocks[move_y][move_x];
                    }
                    let next_heat = heat + move_heat;
                    let direction_index = *next_direction as usize;
                    if next_heat >= best_paths[next_y][next_x][direction_index] {
                        continue;
                    }
                    best_paths[next_y][next_x][direction_index] = next_heat;
                    frontier.push(State::new(next_heat, next_x, next_y, *next_direction));
                }
            }
        }
        panic!("No path found");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let city_blocks = CityBlocks::from(input);
    Some(city_blocks.best_path(1, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let city_blocks = CityBlocks::from(input);
    Some(city_blocks.best_path(4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
