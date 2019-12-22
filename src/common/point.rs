use std::ops::Add;
use std::ops::Sub;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point{
    pub x: isize,
    pub y: isize
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point{
    pub fn new(x: isize, y: isize) -> Point {
        Point{
            x: x,
            y: y
        }
    }

    pub fn abs(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
