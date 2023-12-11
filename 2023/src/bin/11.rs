use advent_of_code::helpers::Point2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(11);

#[derive(Debug)]
struct GalaxyMap {
    galaxies: HashSet<Point2<usize>>,
    dx: HashMap<usize, usize>,
    dy: HashMap<usize, usize>,
}

impl GalaxyMap {
    fn new(value: &str) -> Self {
        let mut galaxies = HashSet::new();
        value.lines().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    galaxies.insert(Point2::new(x, y));
                })
        });
        let width = value.lines().map(|line| line.len()).max().unwrap();
        let height = value.lines().count();
        let mut empty_rows = vec![true; width];
        let mut empty_cols = vec![true; height];
        galaxies.iter().for_each(|g| {
            empty_cols[g.y] = false;
            empty_rows[g.x] = false;
        });
        let dx: HashMap<usize, usize> = empty_rows
            .iter()
            .enumerate()
            .scan(0usize, |acc, (x, empty)| {
                if *empty {
                    *acc += 1;
                }
                Some((x, *acc))
            })
            .collect();
        let dy: HashMap<usize, usize> = empty_cols
            .iter()
            .enumerate()
            .scan(0usize, |acc, (x, empty)| {
                if *empty {
                    *acc += 1;
                }
                Some((x, *acc))
            })
            .collect();
        Self { galaxies, dx, dy }
    }

    fn paths(&self, factor: usize) -> Vec<(Point2<usize>, Point2<usize>, usize)> {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| (*a, *b, self.distance(a, b, factor)))
            .collect()
    }

    fn distance(&self, a: &Point2<usize>, b: &Point2<usize>, factor: usize) -> usize {
        let dx = a.x.abs_diff(b.x);
        let dy = a.y.abs_diff(b.y);
        let expansion =
            self.dx[&a.x].abs_diff(self.dx[&b.x]) + self.dy[&a.y].abs_diff(self.dy[&b.y]);
        dx + dy + expansion * factor
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = GalaxyMap::new(input);
    let distances: usize = map.paths(1).iter().map(|(_, _, distance)| distance).sum();
    Some(distances as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = GalaxyMap::new(input);
    let distances = map
        .paths(1000000 - 1)
        .iter()
        .map(|(_, _, distance)| *distance as u64)
        .sum();
    Some(distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_simpler() {
        let map = GalaxyMap::new(&advent_of_code::template::read_file("examples", DAY));
        let distances: usize = map.paths(9).iter().map(|(_, _, distance)| distance).sum();
        assert_eq!(distances, 1030);
    }

    #[test]
    fn test_part_two_simpler_2() {
        let map = GalaxyMap::new(&advent_of_code::template::read_file("examples", DAY));
        let distances: usize = map.paths(99).iter().map(|(_, _, distance)| distance).sum();
        assert_eq!(distances, 8410);
    }
}
