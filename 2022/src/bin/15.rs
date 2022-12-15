use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

use advent_of_code::helpers::Point2;

#[derive(Debug, Copy, Clone)]
struct SensorBeacon {
    sensor: Point2<i32>,
    beacon: Point2<i32>,
    radius: i32,
}

impl SensorBeacon {
    pub fn distance_to(&self, point: &Point2<i32>) -> i32 {
        (self.sensor.x - point.x).abs() +
            (self.sensor.y - point.y).abs()
    }

    /*
    I dont really understand why i need to specify the lifetime here :(
     */
    pub fn perimeter_of(&self) -> impl Iterator<Item = Point2<i32>> + '_ {
        (0..=self.radius).flat_map(|delta|
            [
                Point2::new(self.sensor.x - delta, self.sensor.y - self.radius - 1 + delta),
                Point2::new(self.sensor.x + self.radius + 1 - delta, self.sensor.y + delta),
                Point2::new(self.sensor.x - delta, self.sensor.y + self.radius + 1 + delta),
                Point2::new(self.sensor.x - self.radius - 1 + delta, self.sensor.y - delta)
            ]
        )
    }
}

impl FromStr for SensorBeacon {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(
            r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)"
        ).expect("");
        let captures = pattern.captures(line).expect("");
        let s_x = captures.get(1)
            .and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
        let s_y = captures.get(2)
            .and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
        let b_x = captures.get(3)
            .and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
        let b_y = captures.get(4)
            .and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();

        let radius = (s_x - b_x).abs() + (s_y - b_y).abs();

        Ok(
            SensorBeacon {
                sensor: Point2::new(s_x, s_y),
                beacon: Point2::new(b_x, b_y),
                radius,
            }
        )
    }
}

struct Readings {
    readings: Vec<SensorBeacon>,
}

impl FromStr for Readings {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let readings: Vec<SensorBeacon> = input.lines()
            .map(|x| SensorBeacon::from_str(x).expect("")).collect();

        Ok(Readings {
            readings
        })
    }
}

impl Readings {
    pub fn count_line(&self, line: i32) -> usize {
        let mut occupied: HashSet<Point2<i32>> = HashSet::new();
        self.readings.iter().for_each(|sensor| {
            let line_distance = (sensor.sensor.y - line).abs();
            // println!("Line distance for sensor {:?}: {}", sensor, line_distance);
            if line_distance < sensor.radius {
                let span = sensor.radius - line_distance;
                let left_x = sensor.sensor.x - span;
                let right_x = sensor.sensor.x + span;
                // println!("Adding points between {} and {}", left_x, right_x);
                for x in left_x..=right_x {
                    occupied.insert(Point2::new(x, line));
                }
            }
        });
        self.readings.iter().for_each(|sensor| {
            occupied.remove(&sensor.beacon);
            occupied.remove(&sensor.sensor);
        });
        occupied.len()
    }

    pub fn scan_area(&self, min_coord: i32, max_coord: i32) -> Option<Point2<i32>> {
        self.readings.iter()
            .flat_map(|sensor| sensor.perimeter_of())
            .filter(|p| p.x >= min_coord && p.x <= max_coord)
            .filter(|p| p.y >= min_coord && p.y <= max_coord)
            .find(|pos| {
                self.readings.iter()
                    .all(|sensor|
                        sensor.distance_to(pos) > sensor.radius
                    )
            })
    }

}

pub fn part_one(input: &str) -> Option<usize> {
    let readings = Readings::from_str(input).expect("");
    Some(
        readings.count_line(2000000)
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let readings = Readings::from_str(input).expect("");
    let result = readings.scan_area(0, 4000000);
    result.map(|p| (p.x as u64) * 4000000 + (p.y as u64))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15, None);
        let readings = Readings::from_str(&input).expect("");
        assert_eq!(readings.count_line(10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15, None);

        let readings = Readings::from_str(&input).expect("");
        let result = readings.scan_area(0, 20);
        let frequency = result.map(|p| (p.x as u64) * 4000000 + (p.y as u64));
        assert_eq!(frequency, Some(56000011));
    }
}
