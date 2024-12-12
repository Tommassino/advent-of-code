use advent_of_code::helpers::Point2;
use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(10);

#[derive(Debug)]
struct TopographicMap {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl From<&str> for TopographicMap {
    fn from(value: &str) -> Self {
        let map: Vec<_> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect();
        let width = map[0].len();
        let height = map.len();
        TopographicMap { map, width, height }
    }
}

impl TopographicMap {
    fn trailhead_score(&self, position: Point2<usize>) -> usize {
        let mut peaks_reached = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(position);
        while let Some(next) = frontier.pop_front() {
            let height = self.map[next.y][next.x];
            if height == 9 {
                peaks_reached.insert(next);
                continue;
            }
            let neighbors = next.neighbors_checked(self.width, self.height);
            for neighbor in neighbors {
                let neighbor_height = self.map[neighbor.y][neighbor.x];
                if neighbor_height == height + 1 {
                    frontier.push_back(neighbor);
                }
            }
        }
        peaks_reached.len()
    }
    fn trailhead_rating(&self, position: Point2<usize>) -> usize {
        let mut peaks_reached = 0;
        let mut frontier = VecDeque::new();
        frontier.push_back(position);
        while let Some(next) = frontier.pop_front() {
            let height = self.map[next.y][next.x];
            if height == 9 {
                peaks_reached += 1;
                continue;
            }
            let neighbors = next.neighbors_checked(self.width, self.height);
            for neighbor in neighbors {
                let neighbor_height = self.map[neighbor.y][neighbor.x];
                if neighbor_height == height + 1 {
                    frontier.push_back(neighbor);
                }
            }
        }
        peaks_reached
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = TopographicMap::from(input);
    let mut score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.map[y][x] == 0 {
                score += map.trailhead_score(Point2::new(x, y));
            }
        }
    }
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = TopographicMap::from(input);
    let mut score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.map[y][x] == 0 {
                score += map.trailhead_rating(Point2::new(x, y));
            }
        }
    }
    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_trailheads() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let map = TopographicMap::from(input);
        let score = map.trailhead_score(Point2::new(2, 0));
        assert_eq!(score, 5);
    }
}
