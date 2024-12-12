use advent_of_code::helpers::{Direction, Point2};
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
advent_of_code::solution!(12);


#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let data: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let width = data[0].len();
        let height = data.len();
        Self {
            data,
            width,
            height,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Plot {
    coordinates: HashSet<Point2<i32>>,
    perimeter: HashSet<Point2<i32>>,
}

impl Grid {
    fn contains(&self, point: Point2<i32>) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    fn get(&self, point2: Point2<i32>) -> Option<char> {
        if self.contains(point2) {
            Some(self.data[point2.y as usize][point2.x as usize])
        } else {
            None
        }
    }

    fn plots(&self) -> Vec<Plot> {
        let mut plots = Vec::new();
        let mut visited = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let current = Point2::new(x as i32, y as i32);
                let current_value = self.get(current).unwrap();
                if visited.contains(&current) {
                    continue;
                }
                let mut stack = VecDeque::new();
                stack.push_back(current);
                let mut field = HashSet::new();
                let mut perimeter_points = HashSet::new();
                while let Some(point) = stack.pop_front() {
                    if visited.contains(&point) {
                        continue;
                    }
                    visited.insert(point);
                    field.insert(point);

                    point
                        .neighbors()
                        .iter()
                        .filter(|&&neighbor| self.get(neighbor) == Some(current_value))
                        .for_each(|neighbor| {
                            stack.push_back(*neighbor);
                        });

                    let at_perimeter = point
                        .neighbors_with_diagonal()
                        .iter()
                        .any(|&neighbor| self.get(neighbor) != Some(current_value));
                    if at_perimeter {
                        perimeter_points.insert(point);
                    }
                }
                plots.push(Plot {
                    coordinates: field,
                    perimeter: perimeter_points.clone(),
                });
            }
        }
        plots
    }
}

impl Plot {
    fn area(&self) -> usize {
        self.coordinates.len()
    }

    fn perimeter(&self) -> usize {
        self.perimeter
            .iter()
            .map(|point| 4 - self.neighbors_in_plot(point))
            .sum()
    }

    fn perimeter_corners(&self) -> usize {
        let mut total_corners = 0;
        for point in &self.perimeter {
            let (north, south, east, west) = (
                self.coordinates.contains(&point.neighbor(Direction::North)),
                self.coordinates.contains(&point.neighbor(Direction::South)),
                self.coordinates.contains(&point.neighbor(Direction::East)),
                self.coordinates.contains(&point.neighbor(Direction::West)),
            );

            let (north_east, north_west, south_east, south_west) = (
                self.coordinates
                    .contains(&point.neighbor(Direction::NorthEast)),
                self.coordinates
                    .contains(&point.neighbor(Direction::NorthWest)),
                self.coordinates
                    .contains(&point.neighbor(Direction::SouthEast)),
                self.coordinates
                    .contains(&point.neighbor(Direction::SouthWest)),
            );

            //convex corners
            total_corners += match self.neighbors_in_plot(point) {
                0 => 4, // special case where its a single point plot
                1 => {
                    2
                } // a single block pointing out
                2 => {
                    // check that its not a | or -
                    if north && south || east && west {
                        0
                    } else {
                        1
                    }
                }
                _ => 0,
            };

            //concave corners
            if !north_east && north && east {
                total_corners += 1;
            }
            if !north_west && north && west {
                total_corners += 1;
            }
            if !south_east && south && east {
                total_corners += 1;
            }
            if !south_west && south && west {
                total_corners += 1;
            }
        }
        total_corners
    }

    fn neighbors_in_plot(&self, point: &Point2<i32>) -> usize {
        point
            .neighbors()
            .iter()
            .filter(|neighbor| self.coordinates.contains(neighbor))
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let plots = grid.plots();
    let mut total_price = 0;
    for plot in plots {
        total_price += plot.area() * plot.perimeter();
    }
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let plots = grid.plots();
    let mut total_cost = 0;
    for plot in plots {
        let corners = plot.perimeter_corners();
        total_cost += plot.area() * corners;
    }
    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_simple2() {
        let result = part_two("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE");
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_simple3() {
        let result = part_two("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA");
        assert_eq!(result, Some(368));
    }
}

/*
  01234
0 1RR1
1 1RRR
2 ..112
3 ..2..

normal corner at Point2 { x: 0, y: 1 }
normal corner at Point2 { x: 3, y: 0 }
normal corner at Point2 { x: 0, y: 0 }

concave corner at Point2 { x: 3, y: 2 }
concave corner at Point2 { x: 2, y: 2 }

double corner at Point2 { x: 2, y: 3 }
double corner at Point2 { x: 4, y: 2 }
 */
