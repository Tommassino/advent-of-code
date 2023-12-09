advent_of_code::solution!(9);

#[derive(Debug)]
struct OasisReading {
    readings: Vec<i32>,
}

impl From<&str> for OasisReading {
    fn from(value: &str) -> Self {
        let readings = value
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        OasisReading { readings }
    }
}

impl OasisReading {
    fn predict(&self) -> i32 {
        OasisReading::_predict(&self.readings)
    }

    fn predict_previous(&self) -> i32 {
        let readings = self.readings.iter().copied().rev().collect();
        OasisReading::_predict(&readings)
    }

    fn _predict(seq: &Vec<i32>) -> i32 {
        if !seq.iter().any(|x| *x != 0) {
            0
        } else {
            let mut next_seq = Vec::new();
            let mut iter = seq.iter();
            let mut prev = iter.next().unwrap();
            for x in iter {
                next_seq.push(x - prev);
                prev = x;
            }
            seq[seq.len() - 1] + OasisReading::_predict(&next_seq)
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let oasis_readings = input
        .lines()
        .map(OasisReading::from)
        .collect::<Vec<OasisReading>>();
    let result = oasis_readings.iter().map(|x| x.predict()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let oasis_readings = input
        .lines()
        .map(OasisReading::from)
        .collect::<Vec<OasisReading>>();
    let result = oasis_readings.iter().map(|x| x.predict_previous()).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_example() {
        let readings = OasisReading::from("10  13  16  21  30  45");
        assert_eq!(readings.predict_previous(), 5)
    }
}
