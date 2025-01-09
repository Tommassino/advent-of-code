use advent_of_code::helpers::{Direction, Point2};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
advent_of_code::solution!(16);

/*
\--- Day 16: Reindeer Maze ---
----------

It's time again for the [Reindeer Olympics](/2015/day/14)! This year, the big event is the *Reindeer Maze*, where the Reindeer compete for the *lowest score*.

You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?

The Reindeer start on the Start Tile (marked `S`) facing *East* and need to reach the End Tile (marked `E`). They can move forward one tile at a time (increasing their score by `1` point), but never into a wall (`#`). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by `1000` points).

To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:

```
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############

```

There are many paths through this maze, but taking any of the best paths would incur a score of only `*7036*`. This can be achieved by taking a total of `36` steps forward and turning 90 degrees a total of `7` times:

```

###############
#.......#....E#
#.#.###.#.###^#
#.....#.#...#^#
#.###.#####.#^#
#.#.#.......#^#
#.#.#####.###^#
#..>>>>>>>>v#^#
###^#.#####v#^#
#>>^#.....#v#^#
#^#.#.###.#v#^#
#^....#...#v#^#
#^###.#.#.#v#^#
#S..#.....#>>^#
###############

```

Here's a second example:

```
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################

```

In this maze, the best paths cost `*11048*` points; following one such path would look like this:

```
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################

```

Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.

Analyze your map carefully. *What is the lowest score a Reindeer could possibly get?*
 */

#[derive(Debug)]
struct Maze {
    walls: HashSet<Point2<i32>>,
    start: Point2<i32>,
    end: Point2<i32>,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert(Point2::new(x as i32, y as i32));
                    }
                    'S' => {
                        start = Some(Point2::new(x as i32, y as i32));
                    }
                    'E' => {
                        end = Some(Point2::new(x as i32, y as i32));
                    }
                    _ => {}
                }
            }
        }
        Self {
            walls,
            start: start.expect("No start found"),
            end: end.expect("No end found"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct SearchNode {
    point: Point2<i32>,
    direction: Direction,
}

impl SearchNode {
    fn new(point: Point2<i32>, direction: Direction) -> Self {
        Self { point, direction }
    }

    fn next(&self) -> Vec<(Self, i32)> {
        // next moves along with cost (turns 1000, moves 1)
        vec![
            (self.forward(), 1),
            (self.rotate(true), 1000),
            (self.rotate(false), 1000),
        ]
    }

    fn forward(&self) -> Self {
        let next_position = match self.direction {
            Direction::North => Point2::new(self.point.x, self.point.y - 1),
            Direction::South => Point2::new(self.point.x, self.point.y + 1),
            Direction::East => Point2::new(self.point.x + 1, self.point.y),
            Direction::West => Point2::new(self.point.x - 1, self.point.y),
            _ => panic!("Invalid direction"),
        };
        Self {
            point: next_position,
            direction: self.direction,
        }
    }

    fn rotate(&self, clockwise: bool) -> Self {
        let next_direction = match (self.direction, clockwise) {
            (Direction::North, true) => Direction::East,
            (Direction::North, false) => Direction::West,
            (Direction::South, true) => Direction::West,
            (Direction::South, false) => Direction::East,
            (Direction::East, true) => Direction::South,
            (Direction::East, false) => Direction::North,
            (Direction::West, true) => Direction::North,
            (Direction::West, false) => Direction::South,
            _ => panic!("Invalid direction"),
        };
        Self {
            point: self.point,
            direction: next_direction,
        }
    }
}

type Path = Vec<SearchNode>;

impl Maze {
    fn is_wall(&self, point: &Point2<i32>) -> bool {
        self.walls.contains(point)
    }

    fn end_nodes(&self) -> [SearchNode; 4] {
        [
            SearchNode::new(self.end, Direction::North),
            SearchNode::new(self.end, Direction::South),
            SearchNode::new(self.end, Direction::East),
            SearchNode::new(self.end, Direction::West),
        ]
    }

    fn shortest_paths(&self) -> (i32, Vec<Path>) {
        let mut previous: HashMap<SearchNode, Vec<SearchNode>> = HashMap::new();
        let mut node_distances: HashMap<SearchNode, i32> = HashMap::new();
        let mut frontier = BinaryHeap::new();

        node_distances.insert(SearchNode::new(self.start, Direction::East), 0);
        frontier.push((0, SearchNode::new(self.start, Direction::East)));

        while let Some((distance, current_node)) = frontier.pop() {
            let distance = -distance;
            for (next_node, move_cost) in current_node.next() {
                if self.is_wall(&next_node.point) {
                    continue;
                }

                let new_distance = distance + move_cost;
                let current_distance = *node_distances.get(&next_node).unwrap_or(&i32::MAX);
                if new_distance < current_distance {
                    frontier.push((-new_distance, next_node));
                    node_distances.insert(next_node, new_distance);
                }

                if new_distance <= current_distance {
                    previous.entry(next_node).or_default().push(current_node);
                }
            }
        }
        let best_cost = self
            .end_nodes()
            .iter()
            .map(|node| node_distances.get(node).unwrap_or(&i32::MAX))
            .min()
            .cloned()
            .unwrap();

        let mut best_paths = Vec::new();
        let mut backtrace_stack = VecDeque::new();
        self.end_nodes().iter().for_each(|node| {
            if node_distances.get(node).unwrap_or(&i32::MAX) == &best_cost {
                for previous_node in previous.get(node).unwrap() {
                    backtrace_stack.push_back((vec![*node], previous_node));
                }
            }
        });

        while let Some((mut path, current_node)) = backtrace_stack.pop_front() {
            path.push(*current_node);
            if current_node.point == self.start {
                best_paths.push(path);
                continue;
            }
            for previous_node in previous.get(current_node).unwrap() {
                backtrace_stack.push_back((path.clone(), previous_node));
            }
        }

        (best_cost, best_paths)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = Maze::from(input);
    let (best_cost, _) = grid.shortest_paths();
    Some(best_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Maze::from(input);
    let (_, best_paths) = grid.shortest_paths();
    let points_on_paths: HashSet<Point2<i32>> =
        best_paths.iter().flatten().map(|node| node.point).collect();
    Some(points_on_paths.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }
    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
