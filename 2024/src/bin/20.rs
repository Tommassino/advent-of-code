use advent_of_code::helpers::Point2;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
advent_of_code::solution!(20);

struct Input {
    map: Vec<Vec<char>>,
    end: Point2<i32>,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let mut end = Point2::new(0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        'S' => '.',
                        'E' => {
                            end = Point2::new(x as i32, y as i32);
                            '.'
                        }
                        _ => char,
                    })
                    .collect()
            })
            .collect();
        Input { map, end }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Input {
    fn enumerate_cheats(
        &self,
        max_cheat_duration: usize,
    ) -> HashMap<(Point2<i32>, Point2<i32>), usize> {
        // Note, this is a pretty slow approach, but i am not sure there is a better one:
        // 1. find all distances from the end
        // 2. sort them by distance decreasing
        // 3. for each combination of distances check if its a cheat that saves times
        let distance_from_end = self.distance_map(self.end);
        let sorted_distances = distance_from_end
            .iter()
            .map(|(&point, &distance)| (distance, point))
            .sorted_by_key(|(distance, _)| *distance)
            .rev()
            .collect::<Vec<_>>();

        let mut cheats = HashMap::new();
        for (idx, (time_to_end_no_cheat, cheat_start)) in sorted_distances.iter().enumerate() {
            for (time_to_end_with_cheat, cheat_end) in sorted_distances.iter().skip(idx + 1) {
                let cheat_vector = *cheat_end - *cheat_start;
                let cheat_distance = cheat_vector.x.abs() + cheat_vector.y.abs();
                if cheat_distance > max_cheat_duration as i32 {
                    continue;
                }
                let saved_time = time_to_end_no_cheat
                    .saturating_sub(*time_to_end_with_cheat + cheat_distance as usize);
                if saved_time > 0 {
                    cheats.insert((*cheat_start, *cheat_end), saved_time);
                }
            }
        }
        cheats
    }

    fn distance_map(&self, from_point: Point2<i32>) -> HashMap<Point2<i32>, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((from_point, 0));
        let width = self.map[0].len() as i32;
        let height = self.map.len() as i32;
        while let Some((point, distance)) = queue.pop_front() {
            if distances.contains_key(&point) {
                continue;
            }
            distances.insert(point, distance);
            for neighbor in point.neighbors_checked(width, height) {
                if self.map[neighbor.y as usize][neighbor.x as usize] == '#' {
                    continue;
                }
                queue.push_back((neighbor, distance + 1));
            }
        }
        distances
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let cheats = input.enumerate_cheats(2);
    let good_cheat_count = cheats.values().filter(|&&time| time >= 100).count();
    Some(good_cheat_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let cheats = input.enumerate_cheats(20);
    let good_cheat_count = cheats.values().filter(|&&time| time >= 100).count();
    Some(good_cheat_count)
}

#[cfg(test)]
mod tests {}
