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
pub struct Canvas {
    paint: HashMap<Point, char>
}

impl Canvas {
    fn new() -> Canvas {
        Canvas{
            paint: HashMap::new()
        }
    }

    fn paint(&mut self, x: isize, y: isize, color: char) {
        self.paint.insert(Point::new(x, y), color);
    }

    fn paint_at(&mut self, pos: Point, color: char) {
        self.paint.insert(pos, color);
    }

    fn color_at(&self, x: isize, y: isize) -> char {
        self.color_at_point(&Point::new(x, y))
    }

    fn color_at_point(&self, point: &Point) -> char {
        *self.paint.get(point).unwrap_or(&' ')
    }
}

impl FromStr for Canvas {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut canvas = Canvas{paint: HashMap::new()};
        let mut x = 0;
        let mut y = 0;
        input.chars().for_each(|c|{
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                canvas.paint(x, y, c);
                x += 1;
            }
        });
        Ok(canvas)
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.paint.keys().map(|p| p.x).min().unwrap();
        let max_x = self.paint.keys().map(|p| p.x).max().unwrap();
        let min_y = self.paint.keys().map(|p| p.y).min().unwrap();
        let max_y = self.paint.keys().map(|p| p.y).max().unwrap();

        let repr: String = (min_y..=max_y).map(|y| {
            let line: String = (min_x..=max_x).map(|x| {
                self.color_at(x, y)
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = Canvas::from_str(&contents).unwrap();
    println!("{}", input);

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(input: &Canvas) {
    let (distance, order) = collect_all_keys(input);
    println!("Best path is {} with order {}", distance, order);
}

fn part2(input: &Canvas) {
    let mut canvas = input.clone();
    let robot_position = canvas.paint.iter().find(|(_, c)| **c == '@').unwrap().0.to_owned();
    canvas.paint_at(robot_position, '#');
    canvas.paint_at(robot_position + Point::new(-1, 0), '#');
    canvas.paint_at(robot_position + Point::new(1, 0), '#');
    canvas.paint_at(robot_position + Point::new(0, -1), '#');
    canvas.paint_at(robot_position + Point::new(0, 1), '#');
    canvas.paint_at(robot_position + Point::new(1, 1), '@');
    canvas.paint_at(robot_position + Point::new(-1, 1), '@');
    canvas.paint_at(robot_position + Point::new(1, -1), '@');
    canvas.paint_at(robot_position + Point::new(-1, -1), '@');
    debug!("{}", canvas);
    let (distance, order) = collect_all_keys(&canvas);
    println!("Best path is {} with order {}", distance, order);
}

fn collect_all_keys(canvas: &Canvas) -> (usize, String) {
    
    let mut paths: HashMap<Point, HashMap<Point, (usize, String)>> = HashMap::new();

    let keys: Vec<(Point, char)> = canvas.paint.iter()
        .filter(|(_, value)| value.is_alphabetic() && value.is_lowercase())
        .map(|(p, c)| (*p, *c)).collect();
    
    let robots: Vec<(Point, char)> = canvas.paint.iter()
        .filter(|(_, value)| **value == '@')
        .map(|(p, c)| (*p, *c)).collect();
    
    for (from_position, from) in keys.iter().chain(robots.iter()) {
        for (to_position, to) in keys.iter() {
            if *from != '@' && from >= to {
                continue;
            }

            match shortest_path(*from_position, *to_position, canvas) {
                Ok((distance, keys_required)) => {
                    debug!("{} -> {}: {} {:?}", from, to, distance, keys_required);

                    if !paths.contains_key(from_position) {
                        paths.insert(*from_position, HashMap::new());
                    }
                    paths.get_mut(from_position).unwrap().insert(*to_position, (distance, keys_required.clone()));
                    if *from != '@' {
                        if !paths.contains_key(to_position) {
                            paths.insert(*to_position, HashMap::new());
                        }
                        paths.get_mut(to_position).unwrap().insert(*from_position, (distance, keys_required.clone()));
                    }
                },
                Err(_) => {}
            }
        }
    }
    
    let key_map: HashMap<Point, char> = keys.iter().map(|(p, c)| (*p, *c)).collect();
    let robot_positions: Vec<Point> = robots.iter().map(|(p, _)| *p).collect();

    collect_keys(
        robot_positions, 
        &String::from(""), 
        &paths,
        &key_map
    )
}

use cached::UnboundCache;

cached_key! {
    COLLECT_KEYS: UnboundCache<String, (usize, String)> = UnboundCache::new();
    Key = { 
        let mut keys: Vec<char> = keys_collected.chars().collect();
        keys.sort();
        format!("{:?}{:?}", keys, current_positions) 
    };
    fn collect_keys(
        current_positions: Vec<Point>,
        keys_collected: &String,
        paths: &HashMap<Point, HashMap<Point, (usize, String)>>,
        key_positions: &HashMap<Point, char>
    ) -> (usize, String) = { 
        let maybe_result = current_positions.iter()
            .flat_map(|from_position| {
                let best_for_position = paths
                    .get(from_position)
                    .expect(&format!("No paths known for position {:?}", from_position))
                    .iter()
                    .filter(|(position, _)| {
                        let key_name = key_positions.get(position).unwrap();
                        !keys_collected.chars().any(|c| c == *key_name)
                    })
                    .flat_map(|(to_position, (segment_length, keys_required))| {
                        let is_unlocked = keys_required.chars().all(|c| keys_collected.chars().any(|x| x == c));
                        if is_unlocked {
                            let mut new_positions = current_positions.clone();
                            let from_idx = new_positions.iter().position(|x| x == from_position).unwrap();
                            new_positions.remove(from_idx);
                            new_positions.push(*to_position);

                            let mut new_keys = keys_collected.clone();
                            let key_name = key_positions.get(to_position).unwrap();
                            new_keys.push(*key_name);

                            let (recursive_length, recursive_path) = collect_keys(new_positions, &new_keys, paths, key_positions);
                            let mut path = recursive_path.clone();
                            path.insert(0, *key_name);
                            Some((recursive_length + segment_length, path))
                        } else {
                            None
                        }
                    })
                    .min_by_key(|x| x.0);

                    best_for_position
            })
            .min_by_key(|x| x.0);
    
        match maybe_result {
            Some(result) => result,
            None => (0, String::from(""))
        }
    }
}


fn shortest_path(
    from: Point, 
    to: Point, 
    canvas: &Canvas
) -> Result<(usize, String), usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, usize, String)> = VecDeque::new();

    queue.push_back((from, 0, String::from("")));

    while !queue.is_empty() {
        let (point, distance, keys_required) = queue.pop_back().unwrap();

        let mut candidates = vec![
            point + Point::new(-1, 0),
            point + Point::new(1, 0),
            point + Point::new(0, -1),
            point + Point::new(0, 1),
        ];
        candidates.sort_by_key(|p| {
            - (*p - to).abs()
        });
        
        for next_point in candidates.iter() {                
            let color = canvas.color_at_point(next_point);
            if *next_point == to {
                return Ok((distance + 1, keys_required));
            }
            let is_passable = color != '#' && color != ' ';
            let was_visited = visited.contains(next_point);
            let is_door = color.is_uppercase() && color.is_alphabetic();
            if is_passable && !was_visited {
                let next_keys_required = 
                if is_door {
                    let mut tmp = keys_required.clone();
                    tmp.push(color.to_ascii_lowercase());
                    tmp
                } else {
                    keys_required.clone()
                };
                visited.insert(next_point.clone());
                queue.push_back((next_point.clone(), distance + 1, next_keys_required));
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
    fn name() {
        let env = Env::new().filter_or("RUST_LOG", "debug");
        init_from_env(env);
        let contents = r#"
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############"#.trim();
            
        let mut input = Canvas::from_str(&contents).unwrap();
        part2(&mut input);
    }
}