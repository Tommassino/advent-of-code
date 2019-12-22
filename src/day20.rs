use log::{debug, info};
use std::fs;
use std::char;
use std::fmt;
use std::fmt::Display;
use std::time::Instant;
use std::convert::Infallible;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::min;
use std::usize;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point{
    x: isize,
    y: isize
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
    fn new(x: isize, y: isize) -> Point {
        Point{
            x: x,
            y: y
        }
    }

    fn abs(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}


#[derive(Clone, Debug)]
pub struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl Grid {
    fn set(&mut self, x: usize, y: usize, color: char) {
        self.data[y][x] = color;
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).map(|a| a.get(x).map(|c| *c)).flatten()
    }

    fn get_unsafe(&self, x: usize, y: usize) -> char {
        self.data.get(y).map(|a| a.get(x).map(|c| *c)).flatten().unwrap_or(' ')
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

#[derive(Debug, Clone)]
struct PlutoMap{
    grid: Grid,
    portal_connections: HashMap<Point, Point>,
    portals: HashMap<Point, String>,
    start: Point,
    finish: Point
}

impl PlutoMap{
    fn new(grid: &Grid) -> PlutoMap {
        let mut portal_locations: HashMap<Point, String> = HashMap::new();
        let mut portals: HashMap<String, (Point, Point)> = HashMap::new();
        let mut portal_connections: HashMap<Point, Point> = HashMap::new();

        fn update(
            portal: String, 
            location: Point,
            exit_location: Point,
            portals: &mut HashMap<String, (Point, Point)>, 
            portal_locations: &mut HashMap<Point, String>, 
            portal_connections: &mut HashMap<Point, Point>
        ) {
            portal_locations.insert(location.clone(), portal.clone());
            portals.insert(portal, (location.clone(), exit_location.clone())).map(|(paired_loc, paired_exit)| {
                portal_connections.insert(location.clone(), paired_exit.clone());
                portal_connections.insert(paired_loc.clone(), exit_location.clone());
            });
        }

        for x in 0..grid.width{
            for y in 0..grid.height{
                if grid.get_unsafe(x, y).is_alphabetic() {
                    if grid.get_unsafe(x, y + 1).is_alphabetic() { //check below
                        let mut portal_vec = vec![grid.get(x, y).unwrap(), grid.get(x, y + 1).unwrap()];
                        portal_vec.sort();
                        let portal_str = portal_vec.iter().collect();

                        let (portal_location, exit_location) = if grid.get_unsafe(x, y + 2) == '.' {
                            (Point::new(x as isize, y as isize + 1), Point::new(x as isize, y as isize + 2))
                        } else {
                            (Point::new(x as isize, y as isize), Point::new(x as isize, y as isize - 1))
                        };
                        update(portal_str, portal_location, exit_location, &mut portals, &mut portal_locations, &mut portal_connections);
                    } else if grid.get_unsafe(x + 1, y).is_alphabetic() { //check to the right
                        let mut portal_vec = vec![grid.get(x, y).unwrap(), grid.get(x + 1, y).unwrap()];
                        portal_vec.sort();
                        let portal_str = portal_vec.iter().collect();

                        let (portal_location, exit_location) = if grid.get_unsafe(x + 2, y) == '.' {
                            (Point::new(x as isize + 1, y as isize), Point::new(x as isize + 2, y as isize))
                        } else {
                            (Point::new(x as isize, y as isize), Point::new(x as isize - 1, y as isize))
                        };
                        update(portal_str, portal_location, exit_location, &mut portals, &mut portal_locations, &mut portal_connections);
                    }
                }
            }
        }

        let (_, start) = portals.get(&String::from("AA")).unwrap();
        let (_, end) = portals.get(&String::from("ZZ")).unwrap();

        PlutoMap{
            grid: grid.clone(),
            portal_connections: portal_connections,
            portals: portal_locations,
            start: *start,
            finish: *end
        }
    }

    fn neighbors(&self, point: Point) -> Vec<(Point, Option<String>)> {
        vec![
            point + Point::new(-1, 0),
            point + Point::new(1, 0),
            point + Point::new(0, -1),
            point + Point::new(0, 1),
        ].iter()
        .filter(|p| {
            let color = self.grid.get_unsafe(p.x as usize, p.y as usize);
            color != '#' && color != ' '
        })
        .map(|p| {
            self.portal_connections.get(p).map(|x| {
                (*x, self.portals.get(p).map(|t| t.to_owned()))
            }).unwrap_or((
                *p, 
                None,
                //Some(format!("[{},{}]", p.x, p.y))
            ))
        }).collect()
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = PlutoMap::new(&Grid::from_str(&contents).unwrap());

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(map: &PlutoMap) {
    let (shortest, trace) = shortest_path(map.start, map.finish, map).expect("No path found");
    println!("Solution is {} with trace {}", shortest, trace);
}

fn part2(input: &PlutoMap) {
}



fn shortest_path(
    from: Point, 
    to: Point, 
    map: &PlutoMap
) -> Result<(usize, String), usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, usize, String)> = VecDeque::new();

    queue.push_back((from, 0, String::from("")));

    while !queue.is_empty() {
        let (point, distance, trace) = queue.pop_front().unwrap();
        
        for (next_point, maybe_trace) in map.neighbors(point) {
            if next_point == to {
                return Ok((distance + 1, trace));
            }
            if !visited.contains(&next_point) {
                let next_trace = maybe_trace.map(|t| format!("{}{}", trace.clone(), t)).unwrap_or(trace.clone());
                visited.insert(next_point.clone());
                queue.push_back((next_point.clone(), distance + 1, next_trace));
            }
        }
    }

    Err(0)
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger::*;

    #[test]
    fn test_small_input() {
        let env = Env::new().filter_or("RUST_LOG", "debug");
        init_from_env(env);
        let contents = r#"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "#;
            
        let map = PlutoMap::new(&Grid::from_str(&contents).unwrap());
        part1(&map);
    }



    #[test]
    fn test_larger_input() {
        let env = Env::new().filter_or("RUST_LOG", "debug");
        init_from_env(env);
        let contents = 
r#"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "#;
            
        let map = PlutoMap::new(&Grid::from_str(&contents).unwrap());
        part1(&map);
    }
}