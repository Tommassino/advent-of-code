/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use num::{CheckedAdd, CheckedSub, One, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Point2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Point2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<T, Output = T>> Mul<T> for Point2<T>
where
    T: Mul<T> + Copy,
{
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Point2<T>
where
    T: CheckedSub<Output = T> + CheckedAdd<Output = T> + Copy + Zero + One + PartialOrd,
{
    pub fn neighbors_checked(&self, width: T, height: T) -> Vec<Point2<T>> {
        [
            (Some(self.x), self.y.checked_sub(&T::one())),
            (Some(self.x), self.y.checked_add(&T::one())),
            (self.x.checked_sub(&T::one()), Some(self.y)),
            (self.x.checked_add(&T::one()), Some(self.y)),
        ]
        .iter()
        .filter_map(|(x, y)| {
            x.filter(|&x| x >= T::zero() && x < width).and_then(|x| {
                y.filter(|&y| y >= T::zero() && y < height)
                    .map(|y| Point2::new(x, y))
            })
        })
        .collect()
    }

    pub fn neighbors(&self) -> Vec<Point2<T>> {
        [
            (Some(self.x), self.y.checked_sub(&T::one())),
            (Some(self.x), self.y.checked_add(&T::one())),
            (self.x.checked_sub(&T::one()), Some(self.y)),
            (self.x.checked_add(&T::one()), Some(self.y)),
        ]
        .iter()
        .filter_map(|(x, y)| x.and_then(|x| y.map(|y| Point2::new(x, y))))
        .collect()
    }

    pub fn neighbors_with_diagonal(&self) -> Vec<Point2<T>> {
        [
            (Some(self.x), self.y.checked_sub(&T::one())),
            (Some(self.x), self.y.checked_add(&T::one())),
            (self.x.checked_sub(&T::one()), Some(self.y)),
            (self.x.checked_add(&T::one()), Some(self.y)),
            (self.x.checked_sub(&T::one()), self.y.checked_sub(&T::one())),
            (self.x.checked_add(&T::one()), self.y.checked_sub(&T::one())),
            (self.x.checked_sub(&T::one()), self.y.checked_add(&T::one())),
            (self.x.checked_add(&T::one()), self.y.checked_add(&T::one())),
        ]
        .iter()
        .filter_map(|(x, y)| x.and_then(|x| y.map(|y| Point2::new(x, y))))
        .collect()
    }
}

impl Point2<i32> {
    pub fn neighbor(&self, direction: Direction) -> Point2<i32> {
        match direction {
            Direction::North => Point2::new(self.x, self.y - 1),
            Direction::South => Point2::new(self.x, self.y + 1),
            Direction::East => Point2::new(self.x + 1, self.y),
            Direction::West => Point2::new(self.x - 1, self.y),
            Direction::NorthEast => Point2::new(self.x + 1, self.y - 1),
            Direction::NorthWest => Point2::new(self.x - 1, self.y - 1),
            Direction::SouthEast => Point2::new(self.x + 1, self.y + 1),
            Direction::SouthWest => Point2::new(self.x - 1, self.y + 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Point3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Point3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul<f32, Output = T>> Mul<f32> for Point3<T> {
    type Output = Point3<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

mod tests {
    #[test]
    fn test_neighbors() {
        use super::Point2;
        let point = Point2::new(1, 1);
        let neighbors = point.neighbors_checked(3, 3);
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Point2::new(0, 1)));
        assert!(neighbors.contains(&Point2::new(1, 0)));
        assert!(neighbors.contains(&Point2::new(2, 1)));
        assert!(neighbors.contains(&Point2::new(1, 2)));
    }
    #[test]
    fn test_neighbors_diagonals() {
        use super::Point2;
        let point = Point2::new(1, 1);
        let neighbors = point.neighbors_diagonals(3, 3);
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&Point2::new(0, 0)));
        assert!(neighbors.contains(&Point2::new(0, 1)));
        assert!(neighbors.contains(&Point2::new(0, 2)));
        assert!(neighbors.contains(&Point2::new(1, 0)));
        assert!(neighbors.contains(&Point2::new(1, 2)));
        assert!(neighbors.contains(&Point2::new(2, 0)));
        assert!(neighbors.contains(&Point2::new(2, 1)));
        assert!(neighbors.contains(&Point2::new(2, 2)));
    }
}
