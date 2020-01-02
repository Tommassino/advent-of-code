use log::{debug, info};
use std::fs;
use std::rc::Rc;
use std::iter;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::char;
use std::str::FromStr;

use crate::common::intcode::*;
use crate::common::point::*;
use crate::common::graph::*;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction{
    NORTH,
    WEST,
    EAST,
    SOUTH
}

impl FromStr for Direction{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "- north" => Ok(Direction::NORTH),
            "- south" => Ok(Direction::SOUTH),
            "- east" => Ok(Direction::EAST),
            "- west" => Ok(Direction::WEST),
            _ => Err(format!("unknown direction {}", s))
        }
    }
}

impl Direction{

    fn as_str(&self) -> &'static str {
        match self {
            Direction::NORTH => "north",
            Direction::WEST => "west",
            Direction::EAST => "east",
            Direction::SOUTH => "south"
        }
    }

    fn as_point(&self) -> Point {
        match self {
            Direction::NORTH => Point::new(0, -1),
            Direction::WEST => Point::new(-1, 0),
            Direction::EAST => Point::new(1, 0),
            Direction::SOUTH => Point::new(0, 1)
        }
    }

    fn from_point(point: &Point) -> Direction {
        match (point.x, point.y) {
            (0, -1) => Direction::NORTH,
            (0, 1) => Direction::SOUTH,
            (1, 0) => Direction::EAST,
            (-1, 0) => Direction::WEST,
            _ => panic!("Unknown direction {:?}", point)
        }
    }

    fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            NORTH => SOUTH,
            SOUTH => NORTH,
            WEST => EAST,
            EAST => WEST
        }
    }
}

#[derive(Clone, Debug)]
struct RoomDescription{
    name: String,
    doors: Vec<Direction>,
    items: Vec<String>
}

#[derive(Debug)]
enum ActionResult{
    ItemPickup,
    ItemDrop,
    RoomDescription(RoomDescription),
    //unused Inventory(Vec<String>),
    AnalysisResult(String)
}

impl ActionResult {
    fn from_str(input: &str) -> Result<Self, String> {
        let split: Vec<&str> = input.split("\n\n").collect();
        
        if input.contains("== Pressure-Sensitive Floor =="){
            //let result = input.lines().skip(9).next().unwrap().to_string();
            Ok(ActionResult::AnalysisResult(input.to_string()))
        } else if input.contains("You take") {
            Ok(ActionResult::ItemPickup)
        } else if input.contains("You drop") {
            Ok(ActionResult::ItemDrop)
        } else if split.len() == 4 || split.len() == 5 {
            let doors: Vec<Direction> = split[2].lines().skip(1).map(|x| {
                Direction::from_str(x).unwrap()
            }).collect();
            
            let items: Vec<String> = if split.len() == 5 {
                split[3].lines().skip(1).map(|x| x.split("-").last().unwrap().trim().to_string()).collect()
            } else {
                vec![]
            };
            let room_name = input.lines().skip(3).next().unwrap().to_string();

            Ok(ActionResult::RoomDescription(RoomDescription{
                name: room_name,
                doors: doors,
                items: items
            }))
        } else {
            Err(format!("Could not parse action result: {}", input))
        }
    }
}

struct Robit {
    computer: Computer,
    items: HashSet<String>
}

impl Robit{
    fn new(program: &Program) -> Robit {
        Robit{
            computer: Computer::new(program),
            items: HashSet::new()
        }
    }

    fn movement(&mut self, direction: Direction) -> Result<ActionResult, String> {
        let result = self.act(&format!("{}\n", direction.as_str()));
        result
    }

    fn pickup(&mut self, what: &str) -> Result<ActionResult, String> {
        let result = self.act(&format!("take {}\n", what));
        self.items.insert(what.to_string());
        result
    }

    fn drop_all(&mut self) {   
        while !self.items.is_empty() {
            let to_drop = self.items.iter().next().unwrap().clone();
            self.drop(&to_drop).unwrap();
        }
    }

    fn drop(&mut self, what: &str) -> Result<ActionResult, String> {
        let result = self.act(&format!("drop {}\n", what));
        self.items.remove(&what.to_string());
        result
    }

    fn act(&mut self, command: &str) -> Result<ActionResult, String> {
        debug!("== Action == {}", command);
        let input: Vec<i128> = command.chars().map(|c| c as i128).collect();
        let output = self.computer.run(input);
        let output_str: String = output.iter()
            .map(|i| char::from_u32(*i as u32).unwrap()).collect();
        debug!("{}", output_str);
        ActionResult::from_str(&output_str)
    }
}

#[derive(Debug)]
struct Map {
    connections: HashMap<String, HashMap<String, Direction>>
}

impl Map {
    fn new() -> Map {
        Map{
            connections: HashMap::new()
        }
    }

    fn move_to(&mut self, robit: &mut Robit, from: String, to: String) {
        debug!("{} -> {}", from, to);
        debug!("{:?}", self.connections);
        let route = bfs_route(from.clone(), to, self).unwrap();

        let mut position = from;
        for step in route.iter().skip(1) {
            let direction = self.connections.get(&position).unwrap().get(step).unwrap();
            match robit.movement(*direction) {
                Ok(ActionResult::RoomDescription(_)) => {},
                result => panic!("{:?}", result)
            }
            position = step.clone();
            
        }
    }

    fn connect(&mut self, from: String, to: String, direction: Direction) {
        if !self.connections.contains_key(&from) {
            self.connections.insert(from.clone(), HashMap::new());
        }
        self.connections.get_mut(&from).unwrap().insert(to, direction);
    }

    fn explore(&mut self, robit: &mut Robit, item_blacklist: &HashSet<String>) {
        let mut position = String::from("== Hull Breach ==");

        let mut visited: HashSet<(String, Direction)> = HashSet::new();
        let mut to_visit: VecDeque<(String, Direction)> = VecDeque::new();

        match robit.act(&"") {
            Ok(ActionResult::RoomDescription(description)) => {
                to_visit.extend(description.doors.iter().map(|door| {
                    (position.clone(), door.clone())
                }));
            },
            result => panic!("{:?}", result)
        };        

        while !to_visit.is_empty() {
            let (next_room, next_direction) = to_visit.pop_front().unwrap();
            debug!("Going to {:?} and taking door {:?}", next_room, next_direction);
            debug!("State {:?}", self);
            debug!("To visit {:?}", to_visit);
            self.move_to(robit, position.clone(), next_room.clone());
            let room_info = robit.movement(next_direction);

            match room_info {
                Ok(ActionResult::RoomDescription(room_info)) => {
                    self.connect(next_room.clone(), room_info.name.clone(), next_direction);
                    self.connect(room_info.name.clone(), next_room.clone(), next_direction.reverse());

                    //do not add doors from checkpoint entrance
                    if room_info.name != "== Security Checkpoint ==" {
                        to_visit.extend(room_info.doors.iter()
                            .map(|door| {
                                (room_info.name.clone(), door.clone())
                            })
                            .filter(|node| !visited.contains(node))
                        );
                    }
        
                    for item in room_info.items {
                        if !item_blacklist.contains(&item) {
                            robit.pickup(&item).unwrap();
                        }
                    }
                    position = room_info.name;
                },
                _ => panic!("Could not move from {:?} in direction {:?}", next_room, next_direction)
            }
            visited.insert((next_room, next_direction));
        }
        self.move_to(robit, position, String::from("== Security Checkpoint =="));
    }
}

impl Neighbors<String> for Map {
    fn neighbors(&self, point: String) -> Vec<String> {
        self.connections.get(&point).map(|connected_rooms| {
            connected_rooms.keys().map(|k| k.to_owned()).collect()
        }).unwrap_or(vec![])
    }
}

pub fn solve(input_file: &str){
    use std::time::Instant;

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input: Program = Program::from_str(&contents);

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn play(bot: &Program) {
    let mut robit = Robit::new(bot);
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let result = robit.act(&format!("{}\n", line.unwrap()));
        println!("{:?}", result);
    }
}

fn part1(bot: &Program) {
    //play(bot);
    let mut map = Map::new();
    let mut blacklist: HashSet<String> = HashSet::new();
    blacklist.insert(String::from("escape pod"));
    blacklist.insert(String::from("photons"));
    blacklist.insert(String::from("molten lava"));
    blacklist.insert(String::from("infinite loop"));
    blacklist.insert(String::from("giant electromagnet"));
    let mut robit = Robit::new(bot);

    map.explore(&mut robit, &blacklist);

    let all_items: Vec<String> = robit.items.iter().map(|x| x.to_owned()).collect();
    println!("{:?}", all_items);
    robit.drop_all();

    use itertools::Itertools;

    (1..=all_items.len())
        .flat_map(|i| all_items.iter().combinations(i))
        .find(|combination| {
            combination.iter().for_each(|item| {
                robit.pickup(item).unwrap();
            });
            match robit.movement(Direction::EAST) {
                Ok(ActionResult::AnalysisResult(analysis)) => {
                    if analysis.contains("Droids on this ship are lighter") || analysis.contains("Droids on this ship are heavier") {
                        robit.drop_all();
                        false
                    } else {
                        println!("Win! {} {:?}", analysis, combination);
                        true
                    }
                },
                _ => panic!()
            }
        });
}

fn part2(bot: &Program) {
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger::*;

    #[test]
    fn test_mod_exponent() {
    }
}
