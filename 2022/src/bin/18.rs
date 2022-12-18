use std::collections::HashSet;
use std::str::FromStr;

use itertools::{iproduct, Itertools};

use advent_of_code::helpers::Point3;

#[derive(Debug, Clone)]
struct LavaDroplet {
    chunks: HashSet<Point3<i32>>,
}

impl LavaDroplet {
    pub fn neighbors(point: &Point3<i32>) -> impl Iterator<Item=Point3<i32>> + '_ {
        iproduct!(-1i32..=1, -1i32..=1, -1i32..=1)
            .filter(|(x, y, z)| {
                (x.abs() + y.abs() + z.abs()) == 1
            })
            .map(|(x, y, z)| {
                Point3::new(point.x + x, point.y + y, point.z + z)
            })
    }

    pub fn area(&self) -> usize {
        self.chunks.iter().map(|point| {
            LavaDroplet::neighbors(point).filter(|neighbor| {
                !self.chunks.contains(neighbor)
            }).count()
        }).sum()
    }

    pub fn outer_area(&self) -> usize {
        let (min_x, max_x) = self.chunks.iter()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .map(|(min, max)| (min - 1, max+1))
            .unwrap();
        let (min_y, max_y) = self.chunks.iter()
            .map(|p| p.y)
            .minmax()
            .into_option()
            .map(|(min, max)| (min - 1, max+1))
            .unwrap();
        let (min_z, max_z) = self.chunks.iter()
            .map(|p| p.z)
            .minmax()
            .into_option()
            .map(|(min, max)| (min - 1, max+1))
            .unwrap();

        let mut water: HashSet<Point3<i32>> = HashSet::new();
        let mut stack: Vec<Point3<i32>> = Vec::new();
        stack.push(Point3::new(min_x, min_y, min_z));
        while let Some(current) = stack.pop() {
            if !water.contains(&current) {
                // println!("Filling {:?} with water", current);
                LavaDroplet::neighbors(&current)
                    .filter(|p| {
                        !self.chunks.contains(p) &&
                            !water.contains(p)
                    })
                    .filter(|p| {
                        p.x >= min_x && p.x <= max_x &&
                            p.y >= min_y && p.y <= max_y &&
                            p.z >= min_z && p.z <= max_z
                    })
                    .for_each(|p| {
                        // println!("Adding {:?} to stack", p);
                        stack.push(p)
                    });
                water.insert(current);
            }
        }
        // println!("{:?}", water);

        self.chunks.iter().map(|chunk| {
            LavaDroplet::neighbors(chunk).filter(|side| {
                water.contains(side)
            }).count()
        }).sum()
    }
}

impl FromStr for LavaDroplet {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let droplets = input.lines()
            .map(|line| {
                let (x, y, z) = line.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .next_tuple().unwrap();
                Point3::new(x, y, z)
            }).collect();
        Ok(
            LavaDroplet {
                chunks: droplets
            }
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let droplets = LavaDroplet::from_str(input).expect("");
    Some(droplets.area())
}

pub fn part_two(input: &str) -> Option<usize> {
    let droplets = LavaDroplet::from_str(input).expect("");
    Some(droplets.outer_area())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18, None);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18, None);
        assert_eq!(part_two(&input), Some(58));
    }

    #[test]
    fn test_part_two_simple(){
        let droplet = LavaDroplet{
            chunks: HashSet::from([Point3::new(1,1,1), Point3::new(2,1,1)])
        };
        println!("{:?}", droplet.outer_area());
    }
}
