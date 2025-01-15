advent_of_code::solution!(25);

#[derive(Debug)]
struct Lock {
    heights: [u32; 5],
}

#[derive(Debug)]
struct Key {
    heights: [u32; 5],
}

impl Lock {
    fn fits_key(&self, key: &Key) -> bool {
        self.heights
            .iter()
            .zip(key.heights.iter())
            .all(|(lock, key)| lock + key <= 5)
    }
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for key_or_lock in input.split("\n\n") {
        let is_lock = key_or_lock.lines().next().unwrap() == "#####";
        let mut heights = [0; 5];
        for line in key_or_lock.lines().skip(1) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }
        if is_lock {
            locks.push(Lock { heights });
        } else {
            keys.push(Key {
                heights: heights.map(|x| x - 1),
            });
        }
    }

    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);
    let mut fitting = 0;
    for lock in &locks {
        for key in &keys {
            if lock.fits_key(key) {
                fitting += 1;
            }
        }
    }
    Some(fitting)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
