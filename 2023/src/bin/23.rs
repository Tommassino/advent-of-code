use advent_of_code::helpers::Point2;
use pathfinding::prelude::bfs;
use petgraph::algo::all_simple_paths;
use petgraph::{Directed, Graph};
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(23);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone)]
struct IslandMap {
    map: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl IslandMap {
    fn new(input: &str, ignore_slopes: bool) -> Self {
        let map: Vec<_> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Path,
                        '#' => Tile::Forest,
                        '^' | '>' | 'v' | '^' if ignore_slopes => Tile::Path,
                        '^' => Tile::Slope(Direction::Up),
                        '>' => Tile::Slope(Direction::Right),
                        'v' => Tile::Slope(Direction::Down),
                        '<' => Tile::Slope(Direction::Left),
                        _ => panic!("Invalid character in map"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let width = map[0].len();
        let height = map.len();

        Self { map, width, height }
    }
    fn successors(&self, p: &Point2<usize>) -> Vec<Point2<usize>> {
        let mut directions = Vec::new();
        match self.map[p.y][p.x] {
            Tile::Path => {
                directions.push(Direction::Up);
                directions.push(Direction::Down);
                directions.push(Direction::Left);
                directions.push(Direction::Right);
            }
            Tile::Forest => (),
            Tile::Slope(direction) => directions.push(direction),
        }
        directions
            .iter()
            .filter_map(|direction| {
                let position_o = match direction {
                    Direction::Down => {
                        if p.y + 1 < self.height {
                            Some(Point2::new(p.x, p.y + 1))
                        } else {
                            None
                        }
                    }
                    Direction::Up => {
                        if p.y > 0 {
                            Some(Point2::new(p.x, p.y - 1))
                        } else {
                            None
                        }
                    }
                    Direction::Right => {
                        if p.x + 1 < self.width {
                            Some(Point2::new(p.x + 1, p.y))
                        } else {
                            None
                        }
                    }
                    Direction::Left => {
                        if p.x > 0 {
                            Some(Point2::new(p.x - 1, p.y))
                        } else {
                            None
                        }
                    }
                };
                if let Some(position) = position_o {
                    let tile = self.map[position.y][position.x];
                    if tile != Tile::Forest {
                        Some(position)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn longest_path(&self, start: Point2<usize>, end: Point2<usize>) -> isize {
        let mut graph: Graph<Point2<usize>, isize, Directed> = Graph::new();
        let mut nodes = Vec::new();
        let mut node_pointers = HashMap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let p = Point2::new(x, y);
                let tile = self.map[p.y][p.x];
                if tile != Tile::Forest && (p == start || p == end || self.successors(&p).len() > 2)
                {
                    nodes.push(p);
                    node_pointers.insert(p, graph.add_node(p));
                }
            }
        }
        for &node in nodes.iter() {
            let mut visited = HashSet::new();
            // find n next edges
            for _ in 0..1.max(self.successors(&node).len()) {
                let path = bfs(
                    &node,
                    |neigh| {
                        if visited.contains(neigh) {
                            vec![]
                        } else {
                            self.successors(neigh)
                        }
                    },
                    |&neigh| {
                        neigh != node
                            && node_pointers.contains_key(&neigh)
                            && !visited.contains(&neigh)
                    },
                );
                if let Some(path) = path {
                    // connect start and end in graph
                    let last_node = path.last().cloned().unwrap();
                    visited.insert(last_node);
                    graph.add_edge(
                        *node_pointers.get(&node).unwrap(),
                        *node_pointers.get(&last_node).unwrap(),
                        path.len() as isize - 1,
                    );
                }
            }
        }
        all_simple_paths::<Vec<_>, _>(&graph, node_pointers[&start], node_pointers[&end], 0, None)
            .map(|path| {
                path.windows(2)
                    .map(|window| {
                        graph
                            .edges_connecting(window[0], window[1])
                            .next()
                            .unwrap()
                            .weight()
                    })
                    .sum::<isize>()
            })
            .max()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let map = IslandMap::new(input, false);
    let start = Point2::new(1, 0);
    let end = Point2::new(map.width - 2, map.height - 1);
    Some(map.longest_path(start, end))
}

pub fn part_two(input: &str) -> Option<isize> {
    let map = IslandMap::new(input, true);
    let start = Point2::new(1, 0);
    let end = Point2::new(map.width - 2, map.height - 1);
    Some(map.longest_path(start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_successors() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let map = IslandMap::new(input.as_str(), false);
        assert_eq!(map.successors(&Point2::new(1, 0)).len(), 1);
        assert_eq!(map.successors(&Point2::new(1, 1)).len(), 2);
        assert_eq!(map.successors(&Point2::new(3, 5)).len(), 3);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
