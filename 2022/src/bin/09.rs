use std::cmp::max;
use std::collections::HashSet;

use itertools::Itertools;

use advent_of_code::helpers::Point2;

pub fn get_direction(dir: &str) -> Point2 {
    match dir {
        "R" => Point2 { x: 1, y: 0 },
        "L" => Point2 { x: -1, y: 0 },
        "U" => Point2 { x: 0, y: -1 },
        "D" => Point2 { x: 0, y: 1 },
        _ => panic!()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tail_positions: HashSet<Point2> = HashSet::new();
    let mut head = Point2 { x: 0, y: 0 };
    let mut tail = Point2 { x: 0, y: 0 };
    input.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split(' ').next_tuple().unwrap();
        let amount = amount_str.parse::<u32>().unwrap();
        let direction = get_direction(direction_str);
        for _ in 0..amount {
            head += direction;
            let vector = head - tail;
            if max(vector.x.abs(), vector.y.abs()) > 1 {
                let adjustment = Point2 {
                    x: if vector.x != 0 { vector.x / vector.x.abs() } else { 0 },
                    y: if vector.y != 0 { vector.y / vector.y.abs() } else { 0 },
                };
                // println!("Moving tail from {:?} towards head at {:?} by {:?}", tail, head, adjustment);
                tail += adjustment;
            }
            tail_positions.insert(tail);
        }
    });
    Some(tail_positions.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tail_positions: HashSet<Point2> = HashSet::new();
    let mut rope_knots: Vec<Point2> = (0..10).map(|_| Point2 { x: 0, y: 0 }).collect();
    input.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split(' ').next_tuple().unwrap();
        let amount = amount_str.parse::<u32>().unwrap();
        let direction = get_direction(direction_str);
        for _ in 0..amount {
            // move the head
            rope_knots[0] += direction;
            // move each knot
            for idx in 1..rope_knots.len() {
                let vector = rope_knots[idx - 1] - rope_knots[idx];
                if max(vector.x.abs(), vector.y.abs()) > 1 {
                    let adjustment = Point2 {
                        x: if vector.x != 0 { vector.x / vector.x.abs() } else { 0 },
                        y: if vector.y != 0 { vector.y / vector.y.abs() } else { 0 },
                    };
                    rope_knots[idx] += adjustment;
                }
            }
            tail_positions.insert(*rope_knots.last().unwrap());
        }
    });
    Some(tail_positions.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9, None);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9, None);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_larger() {
        let input = advent_of_code::read_file("examples", 9, Some("larger"));
        assert_eq!(part_two(&input), Some(36));
    }
}
