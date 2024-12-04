use advent_of_code::helpers::Point2;
use std::ops::Add;
advent_of_code::solution!(4);

#[derive(Debug)]
struct Grid {
    pub data: Vec<Vec<char>>,
    pub width: i32,
    pub height: i32,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let data: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let width = value.lines().map(|line| line.len()).max().unwrap() as i32;
        let height = value.lines().count() as i32;
        Self {
            data,
            width,
            height,
        }
    }
}

impl Grid {
    pub fn match_xmas(&self, position: &Point2<i32>, direction: &Point2<i32>, data: &str) -> bool {
        let mut current_position = *position;
        for c in data.chars() {
            if current_position.x < 0
                || current_position.x >= self.width
                || current_position.y < 0
                || current_position.y >= self.height
            {
                return false;
            }
            if self.data[current_position.y as usize][current_position.x as usize] != c {
                return false;
            }
            current_position = current_position.add(*direction);
        }
        true
    }

    pub fn match_crossmas(&self, position: &Point2<i32>) -> bool {
        if self.data[position.y as usize][position.x as usize] != 'A' {
            return false;
        }
        let up_left = self.data[(position.y - 1) as usize][(position.x - 1) as usize];
        let up_right = self.data[(position.y - 1) as usize][(position.x + 1) as usize];
        let down_left = self.data[(position.y + 1) as usize][(position.x - 1) as usize];
        let down_right = self.data[(position.y + 1) as usize][(position.x + 1) as usize];

        match (up_left, down_right) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            (_, _) => {
                return false;
            }
        }
        match (up_right, down_left) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            (_, _) => {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    let search_string = "XMAS";
    let mut count = 0;
    let directions = [
        Point2::new(1, 0),
        Point2::new(-1, 0),
        Point2::new(0, 1),
        Point2::new(0, -1),
        Point2::new(1, 1),
        Point2::new(-1, -1),
        Point2::new(-1, 1),
        Point2::new(1, -1),
    ];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let position = Point2::new(x, y);
            for direction in directions.iter() {
                if grid.match_xmas(&position, direction, search_string) {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    let mut count = 0;
    for y in 1..(grid.height - 1) {
        for x in 1..(grid.width - 1) {
            if grid.match_crossmas(&Point2::new(x, y)) {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
