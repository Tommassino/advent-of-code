use log::{debug, info};
use std::fs;
use std::{str, char};
use std::fmt;
use num::integer::gcd;
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;
use std::ops::{Sub, Add};
use std::f64::consts::PI;

#[derive(Debug)]
struct Map{
    asteroids: HashSet<Point>
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point{
    x: isize,
    y: isize
}

impl Map {
    fn from_string(data: &str) -> Map {
        let asteroids: HashSet<Point> = data.lines().enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate()
                    .flat_map(move |(x, point)| {
                        match point {
                            '#' => Some(Point{x: x as isize, y: y as isize}),
                            '.' => None,
                            _ => panic!("Unknown map character {}", point)
                        }
                    })
            })
            .collect();
        
        Map{
            asteroids: asteroids
        }
    }

    fn is_visible(&self, first: &Point, second: &Point) -> bool {
        assert_ne!(first, second);
        let point_gcd = gcd(first.x - second.x, first.y - second.y);
        //println!("{:?} {:?} {}", first, second, point_gcd);
        let x_diff = (first.x - second.x) / point_gcd;
        let y_diff = (first.y - second.y) / point_gcd;
        let steps = if x_diff != 0 {
            (first.x - second.x) / x_diff
        } else {
            (first.y - second.y) / y_diff
        };

        (1..steps).find(|i| {
            let candidate = Point{
                x: first.x - x_diff * i,
                y: first.y - y_diff * i,
            };
            self.asteroids.contains(&candidate)
        }).is_none()
    }

    fn list_visible(&self, from: &Point) -> Vec<Point> {
        self.asteroids
            .iter()
            .filter(|b| b != &from)
            .filter(|b| self.is_visible(from, b))
            .map(|x| x.clone())
            .collect()
    }

    fn destroy(&mut self, asteroid: &Point) {
        self.asteroids.remove(asteroid);
    }
}

impl Point {
    fn size(&self) -> f64 {
        ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt()
    }

    fn angle(&self, other: &Point) -> f64 {
        let dot = (self.x * other.x + self.y * other.y) as f64;
        let det = (self.y * other.x - self.x * other.y) as f64;
        let angle = det.atan2(dot);
        if angle < 0.0 {
            angle + PI * 2.0
        } else {
            angle
        }
    }

    fn ord(&self, station: &Point, up: &Point) -> usize {
        let vec = Point{
            x: self.x - station.x,
            y: self.y - station.y
        };
        (vec.angle(up) * 18000.0 / PI) as usize
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut input = Map::from_string(&contents);
    debug!("{:?}", input.asteroids);

    let station_position = part1(&input);
    part2(&mut input, &station_position);
}

fn part1(input: &Map) -> Point {
    let best_position = input.asteroids
        .iter()
        .max_by_key(|a| {
            input.list_visible(a).len()
        })
        .unwrap();
    let total_visible = input.list_visible(best_position).len();
    info!("Best position is {:?} that sees {} total other roids", best_position, total_visible);
    best_position.clone()
}

fn part2(input: &mut Map, station: &Point) {
    let up = Point{x: 0, y: - 10};
    let mut took = 0;
    let mut solution = 0;
    loop {
        let mut targets = input.list_visible(station);
        if targets.len() == 0 {
            break;
        }
        //WTF no Ord for f64??
        targets.sort_by_key(|x| {
            let angle = x.ord(station, &up);
            angle
        });
        let to_destroy = targets.iter().take(200 - took);
        to_destroy.for_each(|roid| {
            input.destroy(&roid);
            took += 1;
            solution = roid.x * 100 + roid.y;
            debug!("Destroying roid {}: {:?}", took, roid);
        });
        if took >= 200 {
            break;
        }
    }
    info!("Solution is {}", solution);
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(4, 2), 2);
    }

    #[test]
    fn test() {
        let a = Point{x: 3, y: 4};
        let b = Point{x: 1, y: 0};
        let intersect = Point{x: 2, y: 2};
        let map = Map{
            asteroids: HashSet::from_iter(vec![a.clone(), intersect, b.clone()])
        };
        assert_eq!(map.is_visible(&a, &b), false);
        part1(&map);
    }

    #[test]
    fn test_full() {
        let input = "
        .#..#\n\
        .....\n\
        #####\n\
        ....#\n\
        ...##\n\
        ".trim();
        let map = Map::from_string(input);
        part1(&map);
    }

    #[test]
    fn test_angle() {
        //full square around 0
        let points = vec![
            Point{x: 0, y: -2},
            Point{x: 1, y: -2},
            Point{x: 2, y: -2},
            Point{x: 2, y: -1},
            Point{x: 2, y: 0},
            Point{x: 2, y: 1},
            Point{x: 2, y: 2},
            Point{x: 1, y: 2},
            Point{x: 0, y: 2},
            Point{x: -1, y: 2},
            Point{x: -2, y: 2},
            Point{x: -2, y: 1},
            Point{x: -2, y: 0},
            Point{x: -2, y: -1},
            Point{x: -2, y: -2},
            Point{x: -1, y: -2},
            Point{x: 0, y: -2}
        ];
        points.iter().for_each(|x| {
            println!("{:?} {}", x, x.angle(&Point{x: 0, y: -2}));
        });
    }

    #[test]
    fn test_order() {
        //example
        let points = vec![
            Point{x: 8, y: 1},
            Point{x: 9, y: 0},
            Point{x: 9, y: 1},
            Point{x: 10, y: 0},
            Point{x: 9, y: 2},
            Point{x: 11, y: 1},
            Point{x: 12, y: 1},
            Point{x: 11, y: 2}
        ];
        let up = Point{x: 0, y: -10};
        let station = Point{x: 8, y: 3};
        points.iter().for_each(|x| {
            debug!("angle between {:?} and {:?}: {}", station, x, x.ord(&station, &up));
        });
    }
}