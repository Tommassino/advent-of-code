use advent_of_code::helpers::Point2;
use std::cmp::min;
use std::collections::HashMap;
advent_of_code::solution!(21);

type Code = Vec<char>;

struct Input {
    codes: Vec<Code>,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        Self {
            codes: input.lines().map(|s| s.chars().collect()).collect(),
        }
    }
}

impl Input {
    fn code_complexity(&self, max_depth: usize) -> u64 {
        let num_keypad = Keypad::numeric();
        let dir_keypad = Keypad::directional_keypad();
        let mut move_memory = HashMap::new();
        let mut stack_memory = HashMap::new();
        let mut complexity_sum = 0;
        for target_code in &self.codes {
            let numeric_codes = num_keypad.presses_for('A', target_code, &mut move_memory);
            // println!("Numeric codes: {:?}", numeric_codes);
            let best_keypresses = numeric_codes
                .iter()
                .map(|code| {
                    dir_keypad.press_code_len(code, max_depth, &mut move_memory, &mut stack_memory)
                })
                .min()
                .unwrap();
            let code_numeric_part = target_code
                .iter()
                .filter(|c| c.is_numeric())
                .collect::<String>();
            let complexity = best_keypresses * code_numeric_part.parse::<u64>().unwrap();
            // println!(
            //     "Code: {:?}, Complexity: {} = {} * {}",
            //     target_code,
            //     complexity,
            //     best_keypresses,
            //     code_numeric_part.parse::<usize>().unwrap()
            // );
            complexity_sum += complexity;
        }
        complexity_sum
    }
}

struct Keypad {
    buttons: HashMap<char, Point2<i32>>,
    forbidden: Point2<i32>,
}

impl Keypad {
    fn numeric() -> Self {
        Self {
            buttons: vec![
                ('7', Point2::new(0, 0)),
                ('8', Point2::new(1, 0)),
                ('9', Point2::new(2, 0)),
                ('4', Point2::new(0, 1)),
                ('5', Point2::new(1, 1)),
                ('6', Point2::new(2, 1)),
                ('1', Point2::new(0, 2)),
                ('2', Point2::new(1, 2)),
                ('3', Point2::new(2, 2)),
                ('0', Point2::new(1, 3)),
                ('A', Point2::new(2, 3)),
            ]
            .into_iter()
            .collect(),
            forbidden: Point2::new(0, 3),
        }
    }

    fn directional_keypad() -> Self {
        Self {
            buttons: vec![
                ('^', Point2::new(1, 0)),
                ('A', Point2::new(2, 0)),
                ('<', Point2::new(0, 1)),
                ('v', Point2::new(1, 1)),
                ('>', Point2::new(2, 1)),
            ]
            .into_iter()
            .collect(),
            forbidden: Point2::new(0, 0),
        }
    }

    fn find_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        //find all the shortest paths from start to end
        if start == end {
            return vec![vec![]];
        }
        let start_point = *self.buttons.get(&start).unwrap();
        let end_point = *self.buttons.get(&end).unwrap();
        let mut frontier = vec![(start_point, vec![])];
        let mut paths = Vec::new();
        while paths.is_empty() {
            let mut new_frontier = Vec::new();
            for (point, path) in frontier.iter() {
                let point = *point;
                if point == end_point {
                    paths.push(path.clone());
                    continue;
                }
                let mut moves = Vec::new();
                let vector = end_point - point;
                match vector.x {
                    x if x > 0 => moves.push('>'),
                    x if x < 0 => moves.push('<'),
                    _ => {}
                };
                match vector.y {
                    y if y > 0 => moves.push('v'),
                    y if y < 0 => moves.push('^'),
                    _ => {}
                };
                for m in moves {
                    let new_point = match m {
                        '>' => point + Point2::new(1, 0),
                        '<' => point + Point2::new(-1, 0),
                        'v' => point + Point2::new(0, 1),
                        '^' => point + Point2::new(0, -1),
                        _ => panic!("Invalid move"),
                    };
                    if new_point != self.forbidden {
                        let mut new_path = path.clone();
                        new_path.push(m);
                        new_frontier.push((new_point, new_path));
                    }
                }
            }
            frontier = new_frontier;
        }
        paths
    }

    fn presses_for(
        &self,
        start_position: char,
        keys: &Code,
        memory: &mut HashMap<(char, char), Vec<Code>>,
    ) -> Vec<Code> {
        // always assume the initial position is the activate key
        let mut presses = vec![vec![]];
        let mut position = start_position;
        for key in keys.iter() {
            // move to the next key
            let paths = memory
                .entry((position, *key))
                .or_insert_with(|| self.find_paths(position, *key));
            let mut next_presses = Vec::new();
            for current_path in presses.iter() {
                for path in paths.iter() {
                    let mut new_path = current_path.clone();
                    new_path.extend(path);
                    new_path.push('A');
                    next_presses.push(new_path);
                }
            }
            presses = next_presses;
            position = *key;
        }
        presses
    }

    fn press_code_len(
        &self,
        code: &Code,
        depth: usize,
        moves_memory: &mut HashMap<(char, char), Vec<Code>>,
        stack_memory: &mut HashMap<(Code, usize), u64>,
    ) -> u64 {
        // this assumes we always start at A
        // receives a code to type and a position in the stack
        // it resolves what it needs to be typed on this keypad to type the code
        // and then sends this to the next depth
        if let Some(cached_result) = stack_memory.get(&(code.clone(), depth)) {
            return *cached_result;
        }
        let codes = self.presses_for('A', code, moves_memory);
        if depth == 1 {
            return codes.iter().map(|s| s.len() as u64).min().unwrap();
        }
        // println!("Code: {}, Depth: {}", code, depth);
        let mut best_length = u64::MAX;
        for next_code in codes {
            let sub_codes = Keypad::split_by_activation(&next_code);
            // println!("Depth: {}, Code: {}, Subcodes: {:?}", depth, next_code, sub_codes);
            let mut total_cost = 0;
            for sub_code in sub_codes {
                total_cost += self.press_code_len(&sub_code, depth - 1, moves_memory, stack_memory);
            }
            best_length = min(best_length, total_cost);
        }
        stack_memory.insert((code.clone(), depth), best_length);
        best_length
    }

    fn split_by_activation(code: &Code) -> Vec<Code> {
        let mut result = Vec::new();
        result.push(Vec::new());
        for &char in code.iter() {
            result.last_mut().unwrap().push(char);
            if char == 'A' {
                result.push(Vec::new());
            }
        }
        result.pop();
        result
            .into_iter()
            .map(|v| v.into_iter().collect())
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = Input::from(input);
    Some(input.code_complexity(2))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = Input::from(input);
    Some(input.code_complexity(25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
