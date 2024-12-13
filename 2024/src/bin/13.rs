use advent_of_code::helpers::Point2;
advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: Point2<usize>,
    b: Point2<usize>,
    prize: Point2<usize>,
}

impl Machine {
    fn solve(&self) -> Option<Point2<usize>> {
        let (px, py) = (self.prize.x as i64, self.prize.y as i64);
        let (ax, ay) = (self.a.x as i64, self.a.y as i64);
        let (bx, by) = (self.b.x as i64, self.b.y as i64);
        let det = ax * by - ay * bx;
        if det == 0 {
            return None;
        }
        let a_coefficient = px * by - py * bx;
        let b_coefficient = py * ax - px * ay;
        if a_coefficient % det != 0 || b_coefficient % det != 0 {
            return None;
        }
        let a = a_coefficient / det;
        let b = b_coefficient / det;
        Point2::new(a as usize, b as usize).into()
    }
}

struct Input {
    machines: Vec<Machine>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut machines = Vec::new();
        let regex = regex::Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        for capture in regex.captures_iter(value) {
            let a = Point2::new(capture[1].parse().unwrap(), capture[2].parse().unwrap());
            let b = Point2::new(capture[3].parse().unwrap(), capture[4].parse().unwrap());
            let prize = Point2::new(capture[5].parse().unwrap(), capture[6].parse().unwrap());
            machines.push(Machine { a, b, prize });
        }
        Self { machines }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let mut total_cost = 0;
    for machine in input.machines {
        let solution = machine.solve();
        if let Some(solution) = solution {
            total_cost += 3 * solution.x + solution.y;
        }
    }
    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = Input::from(input);
    let mut total_cost = 0;
    for machine in input.machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        let solution = machine.solve();
        if let Some(solution) = solution {
            total_cost += 3 * solution.x + solution.y;
        }
    }
    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
