use advent_of_code::helpers::Point2;
use std::collections::HashSet;
advent_of_code::solution!(6);

#[derive(Debug)]
struct Grid {
    obstacles: HashSet<Point2<i32>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Guard {
    position: Point2<i32>,
    direction: Point2<i32>,
}

#[derive(Debug)]
struct Puzzle {
    grid: Grid,
    guard: Guard,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let mut obstacles = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        let mut guard = None;
        for (y, line) in input.lines().enumerate() {
            height = y + 1;
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert(Point2::new(x as i32, y as i32));
                    }
                    '^' => {
                        guard = Some(Guard {
                            position: Point2::new(x as i32, y as i32),
                            direction: Point2::new(0, -1),
                        });
                    }
                    _ => {}
                }
            }
        }
        let guard = guard.unwrap();
        let grid = Grid {
            obstacles,
            width,
            height,
        };
        Puzzle { grid, guard }
    }
}

impl Guard {
    fn patrol(
        &self,
        grid: &Grid,
        extra_obstacle: Option<Point2<i32>>,
    ) -> Option<HashSet<Point2<i32>>> {
        let mut seen_states = HashSet::new();
        let mut path = Vec::new();
        let mut position = self.position;
        let mut direction = self.direction;
        path.push(position);
        loop {
            let next = position + direction;
            if next.x < 0
                || next.x >= grid.width as i32
                || next.y < 0
                || next.y >= grid.height as i32
            {
                return Some(path.into_iter().collect());
            } else if grid.obstacles.contains(&next) || extra_obstacle == Some(next) {
                if seen_states.contains(&(next, direction)) {
                    return None;
                }
                seen_states.insert((next, direction));
                direction = Point2::new(-direction.y, direction.x);
            } else {
                path.push(next);
                position = next;
            }
        }
    }

    fn possible_obstructions(&self, grid: &Grid) -> HashSet<Point2<i32>> {
        let mut obstruction_candidates = self.patrol(grid, None).unwrap();
        obstruction_candidates.remove(&self.position);

        let mut found_obstructions = HashSet::new();
        // iterate over candidates
        for candidate in obstruction_candidates {
            let patrol = self.patrol(grid, Some(candidate));
            if patrol.is_none() {
                found_obstructions.insert(candidate);
            }
        }
        found_obstructions
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from(input);
    let patrol = puzzle.guard.patrol(&puzzle.grid, None).unwrap();
    Some(patrol.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = Puzzle::from(input);
    let obstructions = puzzle.guard.possible_obstructions(&puzzle.grid);
    Some(obstructions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
