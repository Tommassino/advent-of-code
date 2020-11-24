use log::{debug, info};
use std::str::FromStr;
use std::fs;
use std::collections::HashSet;

use crate::common::grid::*;

pub fn solve(input_file: &str){
    use std::time::Instant;

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input: Grid = Grid::from_str(&contents).unwrap();

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn tick(previous: &Grid) -> Grid {
    let mut next = previous.clone();
    for (x, y) in previous.coordinates() {
        let infested_neighbors = vec![
            (-1, 0isize),
            (0isize, -1),
            (1, 0),
            (0, 1)
        ].iter().filter(|(c_x, c_y)| {
            let nx = x as isize + c_x;
            let ny = y as isize + c_y;
            nx >= 0 && ny >= 0 &&
            previous.get(nx as usize, ny as usize) == Some('#')
        }).count();

        match previous.get(x, y).unwrap() {
            '.' => 
                if infested_neighbors == 1 || infested_neighbors == 2 {
                    next.set(x, y, '#');
                },
            '#' => 
                if infested_neighbors != 1 {
                    next.set(x, y, '.');
                },
            c => panic!("Unexpected character! {}", c)
        }
    }
    next
}

fn part1(grid: &Grid) {
    let mut seen_grids: HashSet<Grid> = HashSet::new();
    seen_grids.insert(grid.clone());
    let mut last_grid: Grid = grid.clone();

    let final_grid = loop {
        let next = tick(&last_grid);
        if let Some(duplicate) = seen_grids.replace(next.clone()) {
            break duplicate;
        }
        last_grid = next;
    };

    let result: u64 = final_grid.coordinates().iter().map(|(x, y)|{
        let offset = (y * final_grid.width + x) as u32;
        if final_grid.get(*x, *y) == Some('#') {
            2u64.pow(offset)
        } else {
            0
        }
    }).sum();

    println!("{}", result);
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct RecursiveCoordinate{
    x: isize,
    y: isize,
    dimension: isize
}

impl RecursiveCoordinate {
    fn new(x: isize, y: isize, dimension: isize) -> RecursiveCoordinate {
        RecursiveCoordinate{
            x: x,
            y: y,
            dimension: dimension
        }
    }

    fn neighbors(&self, width: isize, height: isize) -> Vec<RecursiveCoordinate> {
        let center_x = width / 2;
        let center_y = height / 2;
        vec![
            (-1, 0isize),
            (0isize, -1),
            (1, 0),
            (0, 1)
        ].iter().flat_map(|(dx, dy)| {
            let nx = self.x + dx;
            let ny = self.y + dy;
            
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                vec![RecursiveCoordinate::new(center_x + dx, center_y + dy, self.dimension - 1)]
            } else if nx == center_x && ny == center_y {
                match (dx, dy) {
                    (-1, 0) => (0..height).map(|y| RecursiveCoordinate::new(4, y, self.dimension + 1)).collect(),
                    (1, 0) => (0..height).map(|y| RecursiveCoordinate::new(0, y, self.dimension + 1)).collect(),
                    (0, -1) => (0..height).map(|x| RecursiveCoordinate::new(x, 4, self.dimension + 1)).collect(),
                    (0, 1) => (0..height).map(|x| RecursiveCoordinate::new(x, 0, self.dimension + 1)).collect(),
                    _ => panic!("Unknown direction")
                }
            } else {
                vec![RecursiveCoordinate::new(nx, ny, self.dimension)]
            }
        }).collect()
    }
}

#[derive(Clone, Debug)]
struct ErisDimensionalBugs {
    bugs: HashSet<RecursiveCoordinate>,
    width: isize,
    height: isize
}

impl ErisDimensionalBugs {
    fn new(initial_grid: &Grid) -> ErisDimensionalBugs {
        let bugs: HashSet<RecursiveCoordinate> = initial_grid.coordinates().iter()
            .filter(|(x, y)| {
                initial_grid.get(*x, *y) == Some('#')
            })
            .map(|(x, y)| RecursiveCoordinate::new(*x as isize, *y as isize, 0isize))
            .collect();
        
        ErisDimensionalBugs{
            bugs: bugs,
            width: initial_grid.width as isize,
            height: initial_grid.height as isize
        }
    }

    fn tick(&self) -> ErisDimensionalBugs {
        let mut new_bugs: HashSet<RecursiveCoordinate> = HashSet::new();

        let coordinates_to_check: HashSet<RecursiveCoordinate> = self.bugs.iter()
            .flat_map(|bug| {
                let mut neighbors = bug.neighbors(self.width, self.height);
                neighbors.push(bug.clone());
                neighbors
            })
            .collect();
        
        for coordinate in coordinates_to_check {
            let infested_neighbors = coordinate
                .neighbors(self.width, self.height).iter()
                .filter(|neighbor| self.bugs.contains(neighbor))
                .count();
            
            if self.bugs.contains(&coordinate) && infested_neighbors == 1 {
                new_bugs.insert(coordinate);
            } else if !self.bugs.contains(&coordinate) && (infested_neighbors == 1 || infested_neighbors == 2) {
                new_bugs.insert(coordinate);
            }
        }

        ErisDimensionalBugs{
            bugs: new_bugs,
            width: self.width,
            height: self.height
        }
    }
}

fn part2(grid: &Grid) {
    let dimension_grid = ErisDimensionalBugs::new(&grid);
    let final_state = (0..200).fold(dimension_grid, |state, _| {
        state.tick()
    });
    println!("{}", final_state.bugs.len());
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger::*;

    #[test]
    fn test_equality() {
        let contents = r#"
....#
#..#.
#..##
..#..
#....
        "#.trim();
        let grid = Grid::from_str(contents).unwrap();
        let mut grid2 = grid.clone();
        grid2.set(0, 0, '#');
        assert_ne!(grid, grid2);
        grid2.set(0, 0, '.');
        assert_eq!(grid, grid2);
        part1(&grid);
    }

    #[test]
    fn test_neighbors() {
        let tile_19 = RecursiveCoordinate::new(3, 3, 0);
        assert_eq!(4, tile_19.neighbors(5, 5).len());
        let tile_g = RecursiveCoordinate::new(1, 1, 1);
        assert_eq!(4, tile_g.neighbors(5, 5).len());
        let tile_14 = RecursiveCoordinate::new(3, 2, 0);
        assert_eq!(8, tile_14.neighbors(5, 5).len());
        let tile_n = RecursiveCoordinate::new(3, 2, 1);
        assert_eq!(8, tile_n.neighbors(5, 5).len());
    }

    #[test]
    fn test_example(){
        let contents = r#"
....#
#..#.
#.?##
..#..
#....
        "#.trim();
        let grid = Grid::from_str(contents).unwrap();
        let dimension_grid = ErisDimensionalBugs::new(&grid);
        let final_state = (0..10).fold(dimension_grid, |state, _| {
            state.tick()
        });
        assert_eq!(final_state.bugs.len(), 99);
    }
}
