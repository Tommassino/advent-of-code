use log::{debug, info};
use std::fs;
use std::collections::*;
use queues::*;

#[derive(Debug)]
struct StarChart{
    orbits: HashMap<String, HashSet<String>>
}

impl StarChart{

    fn orbits_of(&self, planet: String) -> Option<&HashSet<String>> {
        self.orbits.get(&planet)
    }

    fn parent_of(&self, planet: String) -> Option<String> {
        for (parent, children) in self.orbits.iter() {
            if children.contains(&planet) {
                return Some(parent.to_owned())
            }
        }
        None
    }

    fn path_to(&self, planet: String) -> Vec<String> {
        let mut path: Vec<String> = vec![];
        let mut current = planet;
        loop {
            let parent = self.parent_of(current);
            if parent.is_none() {
                break;
            } else {
                current = parent.unwrap();
                path.push(current.clone());
            }
        }
        path.reverse();
        path
    }

    fn from_file(input_file: &str) -> StarChart {
        let contents = fs::read_to_string(input_file)
            .expect("Something went wrong reading the file");
        
        let mut orbits = HashMap::<String, HashSet<String>>::new();

        contents.split("\n").for_each(|x| {
            let bodies: Vec<&str> = x.split(")").take(2).collect();
            let body1 = bodies[0].to_owned();
            let body2 = bodies[1].to_owned();
            if !orbits.contains_key(&body1) {
                let mut planets = HashSet::new();
                planets.insert(body2);
                orbits.insert(body1, planets);
            } else {
                let body_orbits = orbits.get_mut(bodies[0]).unwrap();
                body_orbits.insert(body2);
            }
        });

        StarChart{
            orbits: orbits
        }
    }
}

pub fn solve(input_file: &str){
    let input = StarChart::from_file(&input_file);
    debug!("{:?}", input);

    part1(&input);
    part2(&input);
}

fn part1(input: &StarChart) {
    let mut distances = HashMap::<String, usize>::new();
    let root = String::from("COM");
    let mut buffer = queue![root.to_owned()];
    distances.insert(root.to_owned(), 0);

    while buffer.size() > 0 {
        let parent = buffer.remove().expect("buffer empty!");
        let distance = distances.get(&parent).unwrap() + 1;

        input
            .orbits_of(parent)
            .map(|x| x.iter().for_each(|to_add| {
                buffer.add(to_add.to_owned()).expect("queue full??");
                distances.insert(to_add.to_owned(), distance);
            }));
    }
    debug!("{:?}", distances);
    let total: usize = distances.values().sum();
    info!("Total orbital distances {}", total);
}

fn part2(input: &StarChart) {
    let santa_path = input.path_to(String::from("SAN"));
    let you_path = input.path_to(String::from("YOU"));
    debug!("{:?}", santa_path);
    debug!("{:?}", you_path);
    let common_path_length: usize = santa_path.iter()
        .zip(you_path.iter())
        .take_while(|(s, y)| s == y)
        .count();
    let path_to_santa = santa_path.len() + you_path.len() - 2 * common_path_length;
    info!("Number of orbital transfers to get to santa {}", path_to_santa);
}

#[cfg(test)]
mod tests{
    use super::*;
}
