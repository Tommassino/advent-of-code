use std::char;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::convert::Infallible;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize
}

impl Grid {
    pub fn set(&mut self, x: usize, y: usize, color: char) {
        self.data[y][x] = color;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).map(|a| a.get(x).map(|c| *c)).flatten()
    }

    pub fn get_unsafe(&self, x: usize, y: usize) -> char {
        self.data.get(y).map(|a| a.get(x).map(|c| *c)).flatten().unwrap_or(' ')
    }

    pub fn coordinates(&self) -> Vec<(usize, usize)> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| (x, y))).collect()
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let width = data.iter().map(|a| a.len()).max().unwrap();
        let height = data.len();
        Ok(Grid{
            data: data,
            width: width,
            height: height
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: String = self.data.iter().map(|line| {
            let line_str: String = line.iter().collect();
            format!("{}\n", line_str)
        }).collect();
        write!(f, "{}", repr)
    }
}
