use advent_of_code::helpers::Point2;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
advent_of_code::solution!(22);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(a: usize, b: usize) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    fn span(&self) -> usize {
        self.max - self.min
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    name: usize,
    x_range: Range,
    y_range: Range,
    z_range: Range,
    bricks_on_top: Vec<usize>,
    on_top_of: Vec<usize>,
}

impl Brick {
    fn new(index: usize, line: &str) -> Self {
        let mut parts = line.split('~');
        let corner_1 = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let corner_2 = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            name: index,
            x_range: Range::new(corner_1[0], corner_2[0]),
            y_range: Range::new(corner_1[1], corner_2[1]),
            z_range: Range::new(corner_1[2], corner_2[2]),
            bricks_on_top: Vec::new(),
            on_top_of: Vec::new(),
        }
    }

    fn intersects_perimeter(&self, other: &Self) -> bool {
        !(self.x_range.max < other.x_range.min
            || self.x_range.min > other.x_range.max
            || self.y_range.min > other.y_range.max
            || self.y_range.max < other.y_range.min)
    }

    fn above(&self, other: &Self) -> bool {
        self.z_range.min > other.z_range.max
    }

    fn rests_on(&self, other: &Self) -> bool {
        self.name != other.name
            && self.intersects_perimeter(other)
            && self.z_range.min == other.z_range.max + 1
    }

    fn drop_on(&mut self, other: &mut Self) {
        self.z_range = Range::new(
            other.z_range.max + 1,
            other.z_range.max + 1 + self.z_range.span(),
        );
    }
}

#[derive(Debug)]
struct Snapshot {
    bricks: Vec<Brick>,
}

impl From<&str> for Snapshot {
    fn from(value: &str) -> Self {
        let bricks = value
            .lines()
            .enumerate()
            .map(|(index, line)| Brick::new(index, line))
            .collect::<Vec<_>>();
        Self { bricks }
    }
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let projection_y = self
            .bricks
            .iter()
            .map(|x| {
                (
                    Point2::new(x.x_range.min, x.z_range.min),
                    Point2::new(x.x_range.max, x.z_range.max),
                )
            })
            .collect::<Vec<_>>();
        let MinMax(min_x, max_x) = projection_y
            .iter()
            .flat_map(|(p_1, p_2)| [p_1.x, p_2.x])
            .minmax()
        else {
            panic!()
        };
        let MinMax(_, max_z) = projection_y
            .iter()
            .flat_map(|(p_1, p_2)| [p_1.y, p_2.y])
            .minmax()
        else {
            panic!()
        };
        for y in (0..=max_z).rev() {
            let line: String = (min_x..=max_x)
                .map(|x| {
                    let block = projection_y
                        .iter()
                        .find(|(p_1, p_2)| p_1.y <= y && y <= p_2.y && p_1.x <= x && x <= p_2.x);
                    if block.is_some() {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl Snapshot {
    fn settle(&mut self) {
        self.bricks.sort_by_key(|b| b.z_range.min);
        let mut settled: Vec<Brick> = Vec::new();
        for brick in self.bricks.iter_mut() {
            let supporting_brick = settled
                .iter_mut()
                .filter_map(|other| {
                    let intersects = brick.name != other.name
                        && brick.intersects_perimeter(other)
                        && brick.above(other);
                    if intersects {
                        Some(other)
                    } else {
                        None
                    }
                })
                .max_by_key(|b| b.z_range.max);
            if let Some(drop_on_brick) = supporting_brick {
                brick.drop_on(drop_on_brick)
            } else {
                brick.z_range = Range::new(1, brick.z_range.span() + 1);
            }
            settled.push(brick.clone());
        }
        self.bricks = settled;
        self.link_supports();
    }

    fn link_supports(&mut self) {
        let mut linked = Vec::new();
        for brick in self.bricks.iter_mut() {
            linked.iter_mut().for_each(|other| {
                if brick.rests_on(other) {
                    other.bricks_on_top.push(brick.name);
                    brick.on_top_of.push(other.name);
                }
            });
            linked.push(brick.clone());
        }
        self.bricks = linked;
    }

    fn save_to_disintegrate(&self) -> Vec<Brick> {
        self.bricks
            .iter()
            .cloned()
            .filter_map(|brick| {
                let safe_to_disintegrate = brick
                    .bricks_on_top
                    .iter()
                    .map(|brick_idx| {
                        let brick = self.bricks.iter().find(|b| b.name == *brick_idx).unwrap();
                        brick.on_top_of.len()
                    })
                    .min();
                if let Some(supported) = safe_to_disintegrate {
                    if supported > 1 {
                        Some(brick)
                    } else {
                        None
                    }
                } else {
                    Some(brick)
                }
            })
            .collect()
    }

    fn chain_reactions(&self) -> HashMap<usize, usize> {
        let mut chain_reactions = HashMap::new();
        for brick in self.bricks.iter() {
            let mut falling = HashSet::new();
            let mut queue = Vec::new();
            queue.push(brick.name);
            falling.insert(brick.name);
            while let Some(brick_idx) = queue.pop() {
                let base_brick = self.bricks.iter().find(|b| b.name == brick_idx).unwrap();
                falling.insert(brick_idx);
                for upper_brick_idx in base_brick.bricks_on_top.iter() {
                    let upper_brick = self
                        .bricks
                        .iter()
                        .find(|b| b.name == *upper_brick_idx)
                        .unwrap();
                    if falling.contains(&upper_brick.name) {
                        continue;
                    }
                    if upper_brick.on_top_of.iter().all(|x| falling.contains(x)) {
                        falling.insert(upper_brick.name);
                        queue.push(upper_brick.name);
                    }
                }
            }
            // println!(
            //     "Brick {}: {:?}",
            //     char::from('A' as u8 + (brick.name as u8)),
            //     disintegrated
            // );
            chain_reactions.insert(brick.name, falling.len() - 1);
        }
        chain_reactions
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut snapshot = Snapshot::from(input);
    snapshot.settle();
    Some(snapshot.save_to_disintegrate().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut snapshot = Snapshot::from(input);
    snapshot.settle();
    // snapshot.bricks.iter().for_each(|x| {
    //     println!(
    //         "Brick {}: {:?}",
    //         char::from('A' as u8 + (x.name as u8)),
    //         x.bricks_on_top
    //     );
    // });
    let result = snapshot.chain_reactions().values().sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_brick_support() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut snapshot = Snapshot::from(input.as_str());
        snapshot.settle();
        let a = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 0)
            .unwrap();
        let b = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 1)
            .unwrap();
        let c = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 2)
            .unwrap();
        let d = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 3)
            .unwrap();
        let e = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 4)
            .unwrap();
        let f = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 5)
            .unwrap();
        let g = snapshot
            .bricks
            .iter()
            .find(|brick| brick.name == 6)
            .unwrap();
        assert!(a.bricks_on_top.contains(&b.name));
        assert!(a.bricks_on_top.contains(&c.name));
        assert!(b.bricks_on_top.contains(&d.name));
        assert!(b.bricks_on_top.contains(&e.name));
        assert!(c.bricks_on_top.contains(&d.name));
        assert!(c.bricks_on_top.contains(&e.name));
        assert!(d.bricks_on_top.contains(&f.name));
        assert!(e.bricks_on_top.contains(&f.name));
        assert!(f.bricks_on_top.contains(&g.name));
    }

    #[test]
    fn test_supports() {
        let brick_a = Brick::new(0, "1,0,1~1,2,1");
        let brick_b = Brick::new(0, "0,0,2~2,0,2");
        let brick_c = Brick::new(0, "0,2,3~2,2,3");
        assert!(brick_a.intersects_perimeter(&brick_b));
        assert!(brick_a.intersects_perimeter(&brick_c));
        assert!(brick_b.above(&brick_a));
        assert!(brick_c.above(&brick_a));
    }
}
