use advent_of_code::helpers::Point2;
use num::Integer;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Rock,
}

#[derive(Debug)]
struct Garden {
    map: Vec<Vec<Tile>>,
    start: Point2<usize>,
    width: usize,
    height: usize,
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let mut map = Vec::new();
        let mut start = Point2::new(0, 0);
        let mut width = 0;
        let mut height = 0;
        for (y, line) in value.lines().enumerate() {
            width = line.len();
            let map_line = line
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Rock,
                    'S' => {
                        start = Point2::new(x, y);
                        Tile::Empty
                    }
                    _ => panic!("Unknown tile {}", c),
                })
                .collect();
            map.push(map_line);
            height = y + 1;
        }
        Garden {
            map,
            start,
            width,
            height,
        }
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.iter() {
            for tile in line.iter() {
                match tile {
                    Tile::Empty => write!(f, ".")?,
                    Tile::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Garden {
    fn get_infinite(&self, x: isize, y: isize) -> Tile {
        let x = x.rem_euclid(self.width as isize) as usize;
        let y = y.rem_euclid(self.height as isize) as usize;
        self.map[y][x]
    }

    fn tile_distances(&self, start: Point2<usize>) -> HashMap<Point2<usize>, usize> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((position, distance)) = queue.pop_front() {
            if visited.contains_key(&position) {
                continue;
            }
            visited.insert(position, distance);
            position
                .neighbors(self.width, self.height)
                .iter()
                .for_each(|neighbor| {
                    if !visited.contains_key(neighbor)
                        && self.map[neighbor.y][neighbor.x] != Tile::Rock
                    {
                        queue.push_back((*neighbor, distance + 1));
                    }
                });
        }
        visited
    }

    #[allow(dead_code)]
    fn infinitely_reachable_simulated(&self, start: Point2<isize>, num_steps: usize) -> usize {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((position, distance)) = queue.pop_front() {
            if visited.contains_key(&position) {
                continue;
            }
            visited.insert(position, distance);
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .for_each(|&(dx, dy)| {
                    let neighbor = Point2::new(position.x + dx, position.y + dy);
                    if distance <= num_steps
                        && !visited.contains_key(&neighbor)
                        && self.get_infinite(neighbor.x, neighbor.y) != Tile::Rock
                    {
                        queue.push_back((neighbor, distance + 1));
                    };
                });
        }
        visited
            .iter()
            .filter(|(_, &distance)| distance % 2 == num_steps % 2)
            .count()
    }

    fn infinitely_reachable(&self, total_steps: usize) -> usize {
        let center_distances = self.tile_distances(self.start);
        let center_distance = self.start.x;
        let even_squares = center_distances
            .values()
            .filter(|&&distance| distance % 2 == 0)
            .count();
        let odd_squares = center_distances
            .values()
            .filter(|&&distance| distance % 2 == 1)
            .count();
        let corner_tile_distances = [
            self.tile_distances(Point2::new(0, 0)),
            self.tile_distances(Point2::new(0, self.height - 1)),
            self.tile_distances(Point2::new(self.width - 1, 0)),
            self.tile_distances(Point2::new(self.width - 1, self.height - 1)),
        ];
        let even_corners = corner_tile_distances
            .iter()
            .map(|distance_map| {
                distance_map
                    .iter()
                    .filter(|(_, &distance)| distance % 2 == 0 && distance <= center_distance)
                    .count()
            })
            .sum::<usize>();
        let odd_corners = corner_tile_distances
            .iter()
            .map(|distance_map| {
                distance_map
                    .iter()
                    .filter(|(_, &distance)| {
                        distance % 2 == 1 && distance > self.width + center_distance
                    })
                    .count()
            })
            .sum::<usize>();

        let (repeats, remainder) = (total_steps - self.start.x).div_rem(&self.width);
        assert_eq!(remainder, 0);
        assert_eq!(repeats % 2, 0);

        let odd_count = (repeats + 1) * (repeats + 1);
        let even_count = repeats * repeats;
        let odd_extra = repeats + 1;
        let even_missing = repeats;

        let result = odd_count
            .checked_mul(odd_squares)
            .and_then(|odd_count| odd_count.checked_add(even_count.checked_mul(even_squares)?))
            .and_then(|odd_count_plus_even_squares| {
                odd_count_plus_even_squares.checked_sub(odd_extra.checked_mul(odd_corners)?)
            })
            .and_then(|odd_count_plus_even_squares_minus_odd_extra| {
                odd_count_plus_even_squares_minus_odd_extra
                    .checked_add(even_missing.checked_mul(even_corners)?)
            });
        assert!(result.is_some());

        odd_count * odd_squares + even_count * even_squares - odd_extra * odd_corners
            + even_missing * even_corners
    }

    #[allow(dead_code)]
    fn infinitely_reachable_interpolation(&self, total_steps: usize) -> f64 {
        let start = Point2::new(self.start.x as isize, self.start.y as isize);
        let (x0, y0) = (65f64, self.infinitely_reachable_simulated(start, 65) as f64);
        let (x1, y1) = (
            196f64,
            self.infinitely_reachable_simulated(start, 196) as f64,
        );
        let (x2, y2) = (
            327f64,
            self.infinitely_reachable_simulated(start, 327) as f64,
        );

        let y01 = (y1 - y0) / (x1 - x0);
        let y12 = (y2 - y1) / (x2 - x1);
        let y012 = (y12 - y01) / (x2 - x0);
        let n = total_steps as f64;

        y0 + y01 * (n - x0) + y012 * (n - x0) * (n - x1)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let garden = Garden::from(input);
    let reachable = garden.tile_distances(garden.start);
    let result = reachable
        .values()
        .filter(|&&distance| distance % 2 == 0 && distance <= 64)
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let garden = Garden::from(input);
    let result_geometric = garden.infinitely_reachable(26501365);
    Some(result_geometric)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let garden = Garden::from(input.as_str());
        let reachable = garden.tile_distances(garden.start);
        let result = reachable
            .iter()
            .filter(|(_, &distance)| distance % 2 == 0 && distance <= 6)
            .count();
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let garden = Garden::from(input.as_str());
        // let start = Point2::new(garden.start.x as isize, garden.start.y as isize);
        // assert_eq!(garden.infinitely_reachable_simulated(start, 65+12*garden.width), 2231730);
        assert_eq!(garden.infinitely_reachable(65 + 2 * garden.width), 89460);
        assert_eq!(garden.infinitely_reachable(65 + 4 * garden.width), 289514);
        assert_eq!(garden.infinitely_reachable(65 + 6 * garden.width), 603768);
        assert_eq!(garden.infinitely_reachable(65 + 8 * garden.width), 1032222);
        assert_eq!(garden.infinitely_reachable(65 + 10 * garden.width), 1574876);
        assert_eq!(garden.infinitely_reachable(65 + 12 * garden.width), 2231730);
    }

    #[test]
    fn test_walk_infinitely_simulated() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let garden = Garden::from(input.as_str());
        let start = Point2::new(garden.start.x as isize, garden.start.y as isize);
        assert_eq!(garden.infinitely_reachable_simulated(start, 6), 16);
        assert_eq!(garden.infinitely_reachable_simulated(start, 10), 50);
        assert_eq!(garden.infinitely_reachable_simulated(start, 50), 1594);
        assert_eq!(garden.infinitely_reachable_simulated(start, 100), 6536);
        assert_eq!(garden.infinitely_reachable_simulated(start, 500), 167004);
        // assert_eq!(garden.infinitely_reachable_simulated(start, 1000), 668697);
        // assert_eq!(garden.infinitely_reachable_simulated(start, 5000), 16733044);
    }

    #[test]
    fn test_corner() {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let garden = Garden::from(input.as_str());
        let mut blank_garden = Garden {
            map: vec![vec![Tile::Empty; garden.width]; garden.height],
            start: Point2::new(0, 0),
            width: garden.width,
            height: garden.height,
        };
        let corner_tile_distances = [
            garden.tile_distances(Point2::new(0, 0)),
            garden.tile_distances(Point2::new(0, garden.height - 1)),
            garden.tile_distances(Point2::new(garden.width - 1, 0)),
            garden.tile_distances(Point2::new(garden.width - 1, garden.height - 1)),
        ];
        let center_distance = garden.start.x;
        let corner_distance = garden.width + center_distance;
        corner_tile_distances.iter().for_each(|distance_map| {
            distance_map.iter().for_each(|(&position, &distance)| {
                if distance % 2 == 1 && distance > corner_distance {
                    blank_garden.map[position.y][position.x] = Tile::Rock;
                }
            });
        });
        // corner_tile_distances.iter().for_each(|distance_map| {
        //     distance_map.iter().for_each(|(&position, &distance)| {
        //         if distance % 2 == 0 && distance <= center_distance {
        //             blank_garden.map[position.y][position.x] = Tile::Rock;
        //         }
        //     });
        // });
        println!("{}", blank_garden);
    }
}
