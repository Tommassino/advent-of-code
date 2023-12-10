use advent_of_code::helpers::Point2;
use std::collections::HashSet;
advent_of_code::solution!(10);

#[derive(Debug)]
struct Pipes {
    data: Vec<Vec<char>>,
    start: Point2<usize>,
    width: usize,
    height: usize,
}

impl From<&str> for Pipes {
    fn from(value: &str) -> Self {
        let mut data: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let width = value.lines().map(|line| line.len()).max().unwrap();
        let height = value.lines().count();
        let start = data
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter().enumerate().find_map(|(x, c)| {
                    if *c == 'S' {
                        Some(Point2::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        //figure out the correct pipe at the start point
        let left = if start.x > 1 {
            let c = data[start.y][start.x - 1];
            c == '-' || c == 'L' || c == 'F'
        } else {
            false
        };
        let right = if start.x < (width - 1) {
            let c = data[start.y][start.x + 1];
            c == '-' || c == 'J' || c == '7'
        } else {
            false
        };
        let up = if start.y > 1 {
            let c = data[start.y - 1][start.x];
            c == '|' || c == '7' || c == 'F'
        } else {
            false
        };
        let down = if start.y < (height - 1) {
            let c = data[start.y + 1][start.x];
            c == '|' || c == 'L' || c == 'J'
        } else {
            false
        };
        match (left, right, up, down) {
            (false, false, true, true) => {
                data[start.y][start.x] = '|';
            }
            (true, true, false, false) => {
                data[start.y][start.x] = '-';
            }
            (false, true, true, false) => {
                data[start.y][start.x] = 'L';
            }
            (true, false, true, false) => {
                data[start.y][start.x] = 'J';
            }
            (true, false, false, true) => {
                data[start.y][start.x] = '7';
            }
            (false, true, false, true) => {
                data[start.y][start.x] = 'F';
            }
            _ => panic!("Invalid start pipe: {:?}", (left, right, up, down)),
        }
        Pipes {
            data,
            start,
            width,
            height,
        }
    }
}

impl Pipes {
    fn next(&self, point: Point2<usize>) -> Vec<Point2<usize>> {
        let point_i = Point2::new(point.x as i32, point.y as i32);
        let result = match self.data[point.y][point.x] {
            '|' => {
                vec![point_i + Point2::new(0, 1), point_i - Point2::new(0, 1)]
            }
            '-' => {
                vec![point_i + Point2::new(1, 0), point_i - Point2::new(1, 0)]
            }
            'L' => {
                vec![point_i - Point2::new(0, 1), point_i + Point2::new(1, 0)]
            }
            'J' => {
                vec![point_i - Point2::new(0, 1), point_i - Point2::new(1, 0)]
            }
            '7' => {
                vec![point_i + Point2::new(0, 1), point_i - Point2::new(1, 0)]
            }
            'F' => {
                vec![point_i + Point2::new(0, 1), point_i + Point2::new(1, 0)]
            }
            _ => panic!("Invalid pipe character: {}", self.data[point.y][point.x]),
        };
        result
            .iter()
            .filter(|point| {
                point.x >= 0
                    && point.y >= 0
                    && point.x < self.width as i32
                    && point.y < self.height as i32
            })
            .map(|point| Point2::new(point.x as usize, point.y as usize))
            .collect()
    }
    fn cycle(&self) -> Vec<Point2<usize>> {
        let mut position = self.start;
        let mut visited = HashSet::new();
        let mut cycle = Vec::new();
        loop {
            visited.insert(position);
            cycle.push(position);
            let next_position_o = self
                .next(position)
                .iter()
                .find(|connection| {
                    !visited.contains(connection)
                        && self
                            .next(**connection)
                            .iter()
                            .any(|connection| *connection == position)
                })
                .cloned();
            if let Some(next_position) = next_position_o {
                position = next_position;
            } else {
                break;
            }
        }
        cycle
    }

    fn cycle_area(&mut self) -> u32 {
        let cycle = self.cycle();
        let mut area = 0;
        for y in 0..self.width {
            let mut is_inside = false;
            let mut corner: Option<char> = None;
            for x in 0..self.height {
                let position = Point2::new(x, y);
                let in_cycle = cycle.contains(&position);
                if !in_cycle && is_inside {
                    area += 1;
                    self.data[y][x] = 'x';
                } else if in_cycle {
                    let c = self.data[y][x];
                    match c {
                        'L' | 'F' => {
                            corner = Some(c);
                        }
                        'J' => {
                            if corner == Some('F') {
                                is_inside = !is_inside;
                            }
                            corner = None;
                        }
                        '7' => {
                            if corner == Some('L') {
                                is_inside = !is_inside;
                            }
                            corner = None;
                        }
                        '|' => {
                            is_inside = !is_inside;
                        }
                        _ => {}
                    }
                }
            }
        }
        area
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pipes = Pipes::from(input);
    Some(pipes.cycle().len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pipes = Pipes::from(input);
    let result = pipes.cycle_area();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }
}
