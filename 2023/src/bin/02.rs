advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_prefix, game_suffix) = value.split_once(": ").unwrap();
        let (_, game_id) = game_prefix.split_once(' ').unwrap();
        let id = game_id.parse::<u32>().unwrap();
        let rounds = game_suffix
            .split("; ")
            .map(|round| {
                let mut game_round = Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                round.split(", ").for_each(|x| {
                    let (count_str, color) = x.split_once(' ').unwrap();
                    let count = count_str.parse::<u32>().unwrap();
                    match color {
                        "red" => game_round.red = count,
                        "green" => game_round.green = count,
                        "blue" => game_round.blue = count,
                        _ => panic!("Unknown color: {}", color),
                    }
                });
                game_round
            })
            .collect();
        Game { id, rounds }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Game::from)
        .filter(|game| {
            // only 12 red cubes, 13 green cubes, and 14 blue cubes
            !game
                .rounds
                .iter()
                .any(|round| round.red > 12 || round.green > 13 || round.blue > 14)
        })
        .map(|g| g.id)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(Game::from)
        .map(|game| {
            let red_power = game.rounds.iter().map(|round| round.red).max().unwrap();
            let green_power = game.rounds.iter().map(|round| round.green).max().unwrap();
            let blue_power = game.rounds.iter().map(|round| round.blue).max().unwrap();
            red_power * green_power * blue_power
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
