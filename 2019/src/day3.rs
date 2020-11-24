use log::{debug, info, trace};
use std::fs;
use std::ops::AddAssign;
use std::iter;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point{
    x: isize,
    y: isize
}

impl Point{
    fn manhattan(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug)]
struct Wire{
    points: Vec<Point>
}


impl Wire {
    fn from_str(def: &str) -> Wire {
        let points = def.split(",").flat_map(|step| {
            let direction = step.chars().next().unwrap();
            let amount = step[1..].parse::<usize>().unwrap();
            match direction {
                'U' => iter::repeat(Point{x: 0, y: 1}).take(amount),
                'D' => iter::repeat(Point{x: 0, y: -1}).take(amount),
                'R' => iter::repeat(Point{x: 1, y: 0}).take(amount),
                'L' => iter::repeat(Point{x: -1, y: 0}).take(amount),
                _ => panic!("Unknown direction encountered {}", direction)
            }
        }).scan(Point{x: 0, y: 0}, |pos, step| {
            pos.add_assign(step);
            Some(pos.clone())
        }).collect::<Vec<Point>>();
        
        Wire{
            points: points
        }
    }

    fn from_file(input_file: &str) -> Vec<Wire> {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        let result = contents.lines().map(Wire::from_str).collect();
        result
    }
}

pub fn solve(input_file: &str){
    let wires = Wire::from_file(&input_file);

    part1(&wires[0], &wires[1]);
    part2(&wires[0], &wires[1]);
}

fn part1(wire1: &Wire, wire2: &Wire) {
    let first: HashSet<Point> = HashSet::from_iter(wire1.points.iter().cloned());
    let second: HashSet<Point> = HashSet::from_iter(wire2.points.iter().cloned());
    let intersections = first.intersection(&second);
    debug!("{:?}", &intersections);
    let origin = Point{x:0, y:0};
    let closest = intersections.min_by_key(|x| x.manhattan(&origin)).unwrap();
    let distance = origin.manhattan(closest);
    trace!("{:?}", wire1);
    trace!("{:?}", wire2);
    info!("distance {} {:?}", distance, closest);
}

fn part2(wire1: &Wire, wire2: &Wire) {
    let first: HashSet<Point> = HashSet::from_iter(wire1.points.iter().cloned());
    let second: HashSet<Point> = HashSet::from_iter(wire2.points.iter().cloned());
    let intersections = first.intersection(&second);

    let distance_to_best = intersections.map(|intersection| {
        let distance1 = wire1.points.iter().position(|x| x == intersection).unwrap() + 1;
        let distance2 = wire2.points.iter().position(|x| x == intersection).unwrap() + 1;
        debug!("Distance to intersection {:?}: {}+{}", intersection, distance1, distance2);
        distance1 + distance2
    }).min().unwrap();
    info!("distance {}", distance_to_best);
}
