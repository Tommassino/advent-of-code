use log::{debug, info};
use std::fs;
use std::char;
use std::time::Instant;
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp::min;
use crate::common::graph::*;
use crate::common::point::*;
use crate::common::grid::*;

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
}

impl Neighbors<Point> for PlutoMap {
    fn neighbors(&self, point: Point) -> Vec<Point> {
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
            self.portal_connections.get(p).map(|x| *x).unwrap_or(*p)
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
    let (shortest, _) = bfs(map.start, map.finish, map).expect("No path found");
    println!("Solution is {}", shortest);
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct State{
    position: Point,
    recursion: isize
}

impl State{
    fn new(position: Point, recursion: isize) -> State {
        State{
            position: position,
            recursion: recursion
        }
    }
}

impl Neighbors<State> for PlutoMap{
    fn neighbors(&self, state: State) -> Vec<State> {
        vec![
            state.position + Point::new(-1, 0),
            state.position + Point::new(1, 0),
            state.position + Point::new(0, -1),
            state.position + Point::new(0, 1),
        ].iter()
        .filter(|p| {
            let color = self.grid.get_unsafe(p.x as usize, p.y as usize);
            color != '#' && color != ' '
        })
        .flat_map(|target| {
            let connection = self.portal_connections.get(target);
            if connection.is_some() {
                //let portal_id = self.portals.get(state.position).unwrap();
                let dist = min(
                    min(target.x, self.grid.width as isize - target.x), 
                    min(target.y, self.grid.height as isize - target.y)
                );
                //debug!("Portal {:?} from edge for {:?}: {}", self.portals.get(target), target, dist);

                let recursion = if dist < 3 {
                    state.recursion - 1
                } else {
                    state.recursion + 1
                };
                if recursion >= 0 && recursion <= 100 {
                    Some(State::new(*connection.unwrap(), recursion))
                } else {
                    None
                }
            } else {
                Some(State::new(*target, state.recursion))
            }
        }).collect()
    }
}

fn part2(map: &PlutoMap) {
    let start = State::new(map.start, 0);
    let finish = State::new(map.finish, 0);
    println!("Routing from {:?} to {:?}", start, finish);
    let (shortest, _) = bfs(start, finish, map).expect("No path found");
    println!("Solution is {}", shortest);
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
        part2(&map);
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

    #[test]
    fn test_recursive_input() {
        let env = Env::new().filter_or("RUST_LOG", "debug");
        init_from_env(env);
        let contents = 
r#"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "#;
            
        let map = PlutoMap::new(&Grid::from_str(&contents).unwrap());
        part2(&map);
    }
    
}