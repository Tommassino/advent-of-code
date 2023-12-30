use advent_of_code::helpers::{Point2, Point3};
use num::FromPrimitive;
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};
advent_of_code::solution!(24);

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    position: Point3<i64>,
    velocity: Point3<i64>,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (position_str, velocity_str) = value.trim().split_once('@').unwrap();
        let position = position_str
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let velocity = velocity_str
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            position: Point3::new(position[0], position[1], position[2]),
            velocity: Point3::new(velocity[0], velocity[1], velocity[2]),
        }
    }
}

impl Hailstone {
    fn slope2d(&self) -> f64 {
        f64::from_i64(self.velocity.y).unwrap() / f64::from_i64(self.velocity.x).unwrap()
    }

    fn intersect2d(&self) -> f64 {
        let px = f64::from_i64(self.position.x).unwrap();
        let py = f64::from_i64(self.position.y).unwrap();
        let vx = f64::from_i64(self.velocity.x).unwrap();
        let vy = f64::from_i64(self.velocity.y).unwrap();
        py - px / vx * vy
    }

    fn intersection2d(&self, other: &Hailstone) -> Option<(bool, Point2<i64>)> {
        let self_in = self.intersect2d();
        let self_sl = self.slope2d();
        let other_in = other.intersect2d();
        let other_sl = other.slope2d();
        if self_sl == other_sl {
            return None;
        }
        let intersection = Point2::new(
            ((self_in - other_in) / (other_sl - self_sl)) as i64,
            (self_in + self_sl * ((self_in - other_in) / (other_sl - self_sl))) as i64,
        );
        let time_self = (intersection.x - self.position.x) / (self.velocity.x);
        let time_other = (intersection.x - other.position.x) / (other.velocity.x);
        Some((time_self.min(time_other) > 0, intersection))
    }
}

fn intersections(hailstones: &[Hailstone], min: i64, max: i64) -> usize {
    let mut count = 0;
    for (a_idx, &a_hail) in hailstones.iter().enumerate() {
        for &b_hail in hailstones.iter().skip(a_idx + 1) {
            if let Some((in_future, intersection)) = a_hail.intersection2d(&b_hail) {
                let in_area = min <= intersection.x
                    && intersection.x <= max
                    && min <= intersection.y
                    && intersection.y <= max;
                if in_future && in_area {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_rock(hailstones: &[Hailstone]) -> i64 {
    // Solve system of equations based on the following
    // position of hail A: H_A(t) = P + V * t
    // position of hail B: H_B(t) = Q + W * t
    // position of hail C: H_C(t) = R + X * t
    // position of rock:   H_R(x) = S + Y * x
    // equation for rock intersecting A,B,C at 3 different times:
    // H_A(t) = H_R(t):
    //   p_a_x + v_a_x * t = p_r_x + v_r_x * t
    //   p_a_y + v_a_y * t = p_r_y + v_r_y * t
    //   p_a_z + v_a_z * t = p_r_z + v_r_z * t
    // H_B(u) = H_R(u)
    //   p_b_x + v_b_x * u = p_r_x + v_r_x * u
    //   p_b_y + v_b_y * u = p_r_y + v_r_y * u
    //   p_b_z + v_b_z * u = p_r_z + v_r_z * u
    // H_C(v) = H_R(v)
    //   p_c_x + v_c_x * v = p_r_x + v_r_x * v
    //   p_c_y + v_c_y * v = p_r_y + v_r_y * v
    //   p_c_z + v_c_z * v = p_r_z + v_r_z * v
    // solve for p_r_*, v_r_* using z3

    let ctx = Context::new(&Config::new());
    let solver = Solver::new(&ctx);
    let [rx, ry, rz, rdx, rdy, rdz] =
        ["rx", "ry", "rz", "rdx", "rdy", "rdz"].map(|name| Int::new_const(&ctx, name));
    let zero = Int::from_i64(&ctx, 0);
    for hailstone in hailstones.iter() {
        let x = Int::from_i64(&ctx, hailstone.position.x);
        let y = Int::from_i64(&ctx, hailstone.position.y);
        let z = Int::from_i64(&ctx, hailstone.position.z);
        let dx = Int::from_i64(&ctx, hailstone.velocity.x);
        let dy = Int::from_i64(&ctx, hailstone.velocity.y);
        let dz = Int::from_i64(&ctx, hailstone.velocity.z);
        let time = Int::fresh_const(&ctx, "t");
        solver.assert(&time.ge(&zero));
        solver.assert(&((&x + &dx * &time)._eq(&(&rx + &rdx * &time))));
        solver.assert(&((&y + &dy * &time)._eq(&(&ry + &rdy * &time))));
        solver.assert(&((&z + &dz * &time)._eq(&(&rz + &rdz * &time))));
    }
    // println!("{:?}", solver);
    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&rx).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&ry).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&rz).unwrap().as_i64().unwrap();
    x + y + z
}

pub fn part_one(input: &str) -> Option<usize> {
    let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();
    Some(intersections(&hailstones, 200000000000000, 400000000000000))
}

pub fn part_two(input: &str) -> Option<i64> {
    let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();
    Some(find_rock(&hailstones))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();
        let result = intersections(&hailstones, 7, 24);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_line_equation() {
        let mut hailstone = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let (a, b) = (hailstone.intersect2d(), hailstone.slope2d());
        let y = a + b * (hailstone.position.x as f64);
        assert_eq!(y, hailstone.position.y as f64);
        hailstone.position = hailstone.position + hailstone.velocity;
        let y = a + b * (hailstone.position.x as f64);
        assert_eq!(y, hailstone.position.y as f64);
    }

    #[test]
    fn test_intersects() {
        let hailstone_a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let hailstone_b = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        let intersection = hailstone_a.intersection2d(&hailstone_b);
        assert!(intersection.is_some());
        let (time, intersection) = intersection.unwrap();
        assert!(time);
        assert_eq!(intersection.x, 14);
        assert_eq!(intersection.y, 15);
    }

    #[test]
    fn test_intersects_past() {
        let hailstone_a = Hailstone::from("20, 25, 34 @ -2, -2, -4");
        let hailstone_b = Hailstone::from("20, 19, 15 @ 1, -5, -3");
        let Some((time, intersection)) = hailstone_a.intersection2d(&hailstone_b) else {
            panic!("")
        };
        assert!(!time);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
