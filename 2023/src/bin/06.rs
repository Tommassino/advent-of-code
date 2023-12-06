advent_of_code::solution!(6);

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl From<&str> for Races {
    fn from(value: &str) -> Self {
        let data: Vec<Vec<u64>> = value
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect()
            })
            .collect();
        let races = data
            .get(0)
            .unwrap()
            .iter()
            .zip(data.get(1).unwrap())
            .map(|(time, distance)| Race {
                time: *time,
                best_distance: *distance,
            })
            .collect();
        Races { races }
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    pub fn winning_button_times(&self) -> u64 {
        let a = -1f64;
        let b = self.time as f64;
        let c = -(self.best_distance as f64);
        let solution_a = (-b - (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);
        let solution_b = (-b + (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);
        let solution_down = (solution_a.min(solution_b) + 1.0).floor() as u64;
        let solution_up = (solution_a.max(solution_b) - 1.0).ceil() as u64;
        (solution_up - solution_down) + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = Races::from(input);
    let result: u64 = races
        .races
        .iter()
        .map(|x| x.winning_button_times())
        .product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let modified_input = input.replace(' ', "").replace(':', ": ");
    let races = Races::from(modified_input.as_str());
    let race = races.races.first().unwrap();
    Some(race.winning_button_times())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
