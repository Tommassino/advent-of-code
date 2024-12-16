use advent_of_code::helpers::Point2;
use std::collections::HashSet;
use std::fmt::Display;
advent_of_code::solution!(14);

struct Robot {
    position: Point2<i32>,
    velocity: Point2<i32>,
}

struct Input {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let regex = regex::Regex::new(r"p=([-\d]+),([-\d]+)\s+v=([-\d]+),([-\d]+)").unwrap();
        let robots = regex
            .captures_iter(input)
            .map(|captures| Robot {
                position: Point2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                velocity: Point2::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            })
            .collect();
        Input {
            robots,
            width: 101,
            height: 103,
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // display # on point where robots are
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            grid[robot.position.y as usize][robot.position.x as usize] = '#';
        }
        for row in grid {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Input {
    fn move_seconds(&self, seconds: i32) -> Input {
        let robots = self
            .robots
            .iter()
            .map(|robot| {
                let unchecked_position = robot.position + robot.velocity * seconds;
                let position = Point2::new(
                    unchecked_position.x.rem_euclid(self.width),
                    unchecked_position.y.rem_euclid(self.height),
                );
                Robot {
                    position,
                    velocity: robot.velocity,
                }
            })
            .collect();
        Input {
            robots,
            width: self.width,
            height: self.height,
        }
    }

    fn count_quadrants(&self) -> [i32; 4] {
        let mut quadrants = [0; 4];
        let grid_x = self.width / 2;
        let grid_y = self.height / 2;
        for robot in &self.robots {
            let position = robot.position;
            if position.x == grid_x || position.y == grid_y {
                continue;
            }
            if position.x < self.width / 2 && position.y < self.height / 2 {
                quadrants[0] += 1;
            } else if position.x >= self.width / 2 && position.y < self.height / 2 {
                quadrants[1] += 1;
            } else if position.x < self.width / 2 && position.y >= self.height / 2 {
                quadrants[2] += 1;
            } else {
                quadrants[3] += 1;
            }
        }
        quadrants
    }

    fn robot_unique_positions(&self) -> bool {
        let positions = self
            .robots
            .iter()
            .map(|robot| robot.position)
            .collect::<HashSet<_>>();
        positions.len() == self.robots.len()
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let robots = Input::from(input);
    let moved_robots = robots.move_seconds(100);
    let quadrants = moved_robots.count_quadrants();
    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    // couple of heuristics tried
    // 1. checking if the image is symetric
    // 2. checking some statistical properties of coordinates
    // ^^ both of these did not work, simple unique position check worked
    let mut robots = Input::from(input);
    let mut seconds = 0;
    loop {
        robots = robots.move_seconds(1);
        seconds += 1;
        if robots.robot_unique_positions() {
            break;
        }
    }
    println!("{}", robots);
    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let mut robots = Input::from(input);
        assert_eq!(robots.robots.len(), 12);
        (robots.width, robots.height) = (11, 7);
        let moved_robots = robots.move_seconds(100);
        println!("{}", moved_robots);
        let quadrants = moved_robots.count_quadrants();
        assert_eq!(quadrants, [1, 3, 4, 1]);
    }

    #[test]
    fn test_symmetric() {
        let robots = Input {
            robots: vec![
                Robot {
                    position: Point2::new(0, 0),
                    velocity: Point2::new(0, 0),
                },
                Robot {
                    position: Point2::new(2, 0),
                    velocity: Point2::new(0, 1),
                },
            ],
            width: 3,
            height: 3,
        };
        assert!(robots.symmetric());
        assert!(!robots.move_seconds(1).symmetric())
    }
}
