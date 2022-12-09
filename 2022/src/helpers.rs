/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Point2{
    type Output = Point2;

    fn add(self, rhs: Self) -> Self::Output {
        Point2{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Point2{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point2{
    type Output = Point2;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign for Point2{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
