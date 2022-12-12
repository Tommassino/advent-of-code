/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 {
            x,
            y
        }
    }
}

impl<T: Add<Output=T>> Add for Point2<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: AddAssign> AddAssign for Point2<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output=T>> Sub for Point2<T>{
    type Output = Self<>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: SubAssign> SubAssign for Point2<T>{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<f32, Output=T>> Mul<f32> for Point2<T>{
    type Output = Point2<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
