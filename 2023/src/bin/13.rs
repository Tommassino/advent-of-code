use itertools::Itertools;
advent_of_code::solution!(13);

#[derive(Debug)]
struct Mirrors {
    horizontal: Vec<u32>,
    vertical: Vec<u32>,
}

impl From<&str> for Mirrors {
    fn from(value: &str) -> Self {
        let mirror_map: Vec<Vec<char>> = value
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let horizontal = mirror_map
            .iter()
            .map(|line| {
                line.iter()
                    .enumerate()
                    .map(|(offset, c)| match c {
                        '#' => 1 << offset,
                        '.' => 0,
                        _ => panic!("Invalid map character: {}", c),
                    })
                    .sum::<u32>()
            })
            .collect_vec();
        let width = mirror_map.first().map(|x| x.len()).unwrap();
        let vertical = (0..width)
            .map(|x| {
                mirror_map
                    .iter()
                    .enumerate()
                    .map(|(offset, line)| {
                        let c = line[x];
                        match c {
                            '#' => 1 << offset,
                            '.' => 0,
                            _ => panic!("Invalid map character: {}", c),
                        }
                    })
                    .sum()
            })
            .collect_vec();
        Mirrors {
            vertical,
            horizontal,
        }
    }
}

impl Mirrors {
    fn mirror_points(&self, error_count: usize) -> (Option<usize>, Option<usize>) {
        (
            Mirrors::find_mirror_point(&self.vertical, error_count),
            Mirrors::find_mirror_point(&self.horizontal, error_count),
        )
    }

    fn find_mirror_point(numbers: &Vec<u32>, error_count: usize) -> Option<usize> {
        (0..numbers.len()).find(|mirror_point| {
            let count = (numbers.len() - mirror_point).min(*mirror_point);
            let errors = (1..=count)
                .map(|dx| {
                    u32::count_ones(numbers[mirror_point - dx] ^ numbers[mirror_point + dx - 1])
                })
                .sum::<u32>();
            errors as usize == error_count && count > 0
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mirrors = input
        .split("\n\n")
        .map(Mirrors::from)
        .map(|x| {
            let mirror_points = x.mirror_points(0);
            match mirror_points {
                (Some(n), None) => n,
                (None, Some(n)) => n * 100,
                _ => panic!("Unknown mirror points: {:?}", mirror_points),
            }
        })
        .sum::<usize>();
    Some(mirrors)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mirrors = input
        .split("\n\n")
        .map(Mirrors::from)
        .map(|x| {
            let mirror_points = x.mirror_points(1);
            match mirror_points {
                (Some(n), None) => n,
                (None, Some(n)) => n * 100,
                _ => panic!("Unknown mirror points: {:?}", mirror_points),
            }
        })
        .sum::<usize>();
    Some(mirrors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
