use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

struct EncodedFile {
    data: VecDeque<(usize, i64)>,
}

impl EncodedFile {
    pub fn decode(&mut self, decryption_key: i64, rounds: usize) {
        // println!("{:?}", self);
        for _ in 0..rounds {
            for original_idx in 0..self.data.len() {
                // find the element to move
                let idx = self.data.iter()
                    .enumerate()
                    .find_map(|(ni, (oi, _))| {
                        (*oi == original_idx).then_some(ni)
                    }).unwrap();
                // rotate list to start
                self.data.rotate_left(idx);
                let (oi, val) = self.data.pop_front().unwrap();
                let rotate = (val * decryption_key).rem_euclid(self.data.len() as i64) as usize;
                // println!("Moving {} by rotating {}", val, rotate);
                // rotate and push item back in place
                self.data.rotate_left(rotate);
                self.data.push_front((oi, val));
                // println!("{:?}", self);
            }
        }
    }

    pub fn iter_zero(&self) -> impl Iterator<Item=i64> + '_ {
        let start_idx = self.data
            .iter()
            .enumerate()
            .find_map(|(idx, (_, val))| (0 == *val).then_some(idx))
            .unwrap();
        self.data.iter()
            .cycle()
            .map(|(_, val)| *val)
            .skip(start_idx)
            .take(self.data.len())
    }

    pub fn coordinates(&self, x: usize, y: usize, z: usize) -> (i64, i64, i64) {
        (
            self.get(x),
            self.get(y),
            self.get(z)
        )
    }

    fn get(&self, idx: usize) -> i64 {
        let offset = idx % self.data.len();
        self.iter_zero().nth(offset).unwrap()
    }
}

impl Debug for EncodedFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let buffer: Vec<i64> = self.iter_zero().collect();
        writeln!(f, "{:?}", buffer)
    }
}

impl FromStr for EncodedFile {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input.lines().enumerate().map(|(idx, line)| {
            let value = line.parse::<i64>().unwrap();
            (idx, value)
        }).collect();
        Ok(EncodedFile {
            data
        })
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut encoded_file = EncodedFile::from_str(input).expect("");
    encoded_file.decode(1, 1);
    let (xc, yc, zc) = encoded_file.coordinates(1000, 2000, 3000);
    // println!("{}, {}, {}", xc, yc, zc);
    Some(xc + yc + zc)
}

pub fn part_two(input: &str) -> Option<i64> {
    let decryption_key = 811589153;
    let mut encoded_file = EncodedFile::from_str(input).expect("");
    encoded_file.decode(decryption_key, 10);
    let (xc, yc, zc) = encoded_file.coordinates(1000, 2000, 3000);
    // println!("{}, {}, {}", xc, yc, zc);
    Some((xc + yc + zc) * decryption_key)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20, None);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20, None);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
