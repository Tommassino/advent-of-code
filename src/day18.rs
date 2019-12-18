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
    paint: HashMap<(isize, isize), char>
}

impl Canvas {
    fn new() -> Canvas {
        Canvas{
            paint: HashMap::<(isize, isize), char>::new()
        }
    }

    fn paint(&mut self, x: isize, y: isize, color: char) {
        self.paint.insert((x, y), color);
    }

    fn color_at(&self, x: isize, y: isize) -> char {
        *self.paint.get(&(x, y)).unwrap_or(&' ')
    }

    fn color_at_point(&self, point: &Point) -> char {
        self.color_at(point.x, point.y)
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
        let min_x = *self.paint.keys().map(|(x, _)| x).min().unwrap();
        let max_x = *self.paint.keys().map(|(x, _)| x).max().unwrap();
        let min_y = *self.paint.keys().map(|(_, y)| y).min().unwrap();
        let max_y = *self.paint.keys().map(|(_, y)| y).max().unwrap();

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

fn collect_all_keys(canvas: &Canvas) -> (usize, String) {
    let mut points: HashMap<char, Point> = HashMap::new();

    canvas.paint.iter()
        .filter(|(_, value)| value.is_alphabetic() || **value == '@')
        .for_each(|((x, y), value)| {
            points.insert(*value, Point{x: *x, y: *y});
        });

    let keys: Vec<char> = points.keys()
        .filter(|x| x.is_lowercase() || **x == '@')
        .map(|x| x.to_owned()).collect();
    
    let mut paths: HashMap<char, HashMap<char, (usize, String)>> = HashMap::new();

    for from in keys.iter() {
        for to in keys.iter() {
            if from >= to {
                continue;
            }

            let (distance, keys_required) = shortest_path(*from, *to, &points, &keys, input).unwrap();
            debug!("{} -> {}: {} {:?}", from, to, distance, keys_required);

            if !paths.contains_key(from) {
                paths.insert(*from, HashMap::new());
            }
            paths.get_mut(from).unwrap().insert(*to, (distance, keys_required.clone()));
            if !paths.contains_key(to) {
                paths.insert(*to, HashMap::new());
            }
            paths.get_mut(to).unwrap().insert(*from, (distance, keys_required.clone()));
        }
    }
    

    collect_keys(
        '@', 
        &String::from(""), 
        &paths
    )
}

use cached::UnboundCache;

cached_key! {
    COLLECT_KEYS: UnboundCache<String, (usize, String)> = UnboundCache::new();
    Key = { 
        let mut keys: Vec<char> = keys_collected.chars().collect();
        keys.sort();
        format!("{:?}{}", keys, current_location) 
    };
    fn collect_keys(
        current_location: char,
        keys_collected: &String,
        paths: &HashMap<char, HashMap<char, (usize, String)>>
    ) -> (usize, String) = {
        let paths_from = paths.get(&current_location).unwrap();
    
        let keys_left: Vec<char> = paths_from.keys()
            .filter(|k| **k != '@')
            .filter(|k| !keys_collected.chars().any(|c| c == **k))
            .map(|x| *x)
            .collect();
    
        if keys_left.is_empty() {
            return (0, String::from(""));
        }
    
        let (best_length, best_keys) = keys_left
            .iter()
            .flat_map(|key| {
                let (segment_length, keys_required) = paths_from.get(key).unwrap();
                let is_unlocked = keys_required.chars().all(|c| keys_collected.chars().any(|x| x == c));
                if is_unlocked {
                    let mut new_keys = keys_collected.clone();
                    new_keys.push(*key);
                    let (recursive_length, recursive_path) = collect_keys(*key, &new_keys, paths);
                    let mut path = recursive_path.clone();
                    path.insert(0, *key);
                    Some((recursive_length + segment_length, path))
                } else {
                    None
                }
            })
            .min_by_key(|x| x.0)
            .unwrap();
    
        (best_length, best_keys)
    }
}


fn shortest_path(
    from: char, 
    to: char, 
    locations: &HashMap<char, Point>, 
    keys: &Vec<char>, 
    canvas: &Canvas
) -> Result<(usize, String), usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, usize, String)> = VecDeque::new();

    let from_location = locations.get(&from).unwrap().to_owned();
    let to_location = locations.get(&to).unwrap().to_owned();
    queue.push_back((from_location, 0, String::from("")));

    while !queue.is_empty() {
        let (point, distance, keys_required) = queue.pop_back().unwrap();

        let mut candidates = vec![
            point + Point::new(-1, 0),
            point + Point::new(1, 0),
            point + Point::new(0, -1),
            point + Point::new(0, 1),
        ];
        candidates.sort_by_key(|p| {
            - (*p - to_location).abs()
        });
        
        for next_point in candidates.iter() {                
            let color = canvas.color_at_point(next_point);
            if *next_point == to_location {
                return Ok((distance + 1, keys_required));
            }
            let is_passable = color != '#' && color != ' ';
            let was_visited = visited.contains(next_point);
            let is_door = color.is_uppercase() && keys.contains(&color.to_ascii_lowercase());
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

fn part2(input: &Canvas) {
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
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"#.trim();
            
        let input = Canvas::from_str(&contents).unwrap();
        println!("{}", input);

        let part1_time = Instant::now();
        part1(&input);
    }
}