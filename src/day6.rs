use log::{debug, info};
use std::fs;
use std::collections::*;
use std::collections::hash_map::Entry;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Planet<'s> {
    name: &'s str,
    parent: Option<&'s str>,
    children: Vec<&'s str>
}

#[derive(Clone, Debug)]
struct StarChart<'s> {
    planets: HashMap<&'s str, Planet<'s>>
}

impl<'s> StarChart<'s> {

    fn get_planet(&self, planet: &'s str) -> Option<&'s Planet> {
        self.planets.get(planet)
    }

    fn parent_of(&self, planet: &'s str) -> Option<&'s str> {
        self.planets.get(planet).map(|p| p.parent).unwrap_or(None)
    }

    fn from_string(contents: &str) -> StarChart {
        let mut planets = HashMap::<&str, Planet>::new();

        contents
            .lines()
            .for_each(|x| {
                let bodies: Vec<&str> = x.split(")").take(2).collect();

                let planet = bodies[0];
                let orbiter = bodies[1];
                
                match planets.entry(planet) {
                    Entry::Occupied(slot) => {
                        slot.into_mut().children.push(orbiter);
                    }
                    Entry::Vacant(slot) => {
                        slot.insert(Planet{
                            name: planet,
                            parent: None,
                            children: vec![orbiter]
                        });
                    }
                }
                
                match planets.entry(orbiter) {
                    Entry::Occupied(slot) => {
                        slot.into_mut().parent = Some(planet);
                    }
                    Entry::Vacant(slot) => {
                        slot.insert(Planet{
                            name: orbiter,
                            parent: Some(planet),
                            children: Vec::new()
                        });
                    }
                }
            });

        StarChart{
            planets: planets
        }
    }
}

pub fn solve(input_file: &str){
    let contents: String = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = StarChart::from_string(&contents);

    part1(&input);
    part2(&input);
}

fn part1(input: &StarChart) {
    fn walk<'s>(
        star_chart: &StarChart,
        current_planet: &'s str,
        current_depth: usize
    ) -> usize {
        let children_total = star_chart
            .get_planet(current_planet)
            .map(|x| x
                .children.iter()
                .map(|child| walk(star_chart, child, current_depth + 1))
                .sum()
            )
            .unwrap_or(0);

        current_depth + children_total
    }

    let total = walk(input, "COM", 0);

    info!("Total orbital distances {}", total);
}

fn part2(input: &StarChart) {
    fn path_to<'s>(
        input: &'s StarChart, 
        planet: &'s str
    ) -> Vec<&'s str> {
        let mut path: Vec<&'s str> = vec![];
        let mut current = planet;

        loop {
            if let Some(parent) = input.parent_of(current) {
                current = parent;
                path.push(current.clone());
            } else {
                break;
            }
        }
        
        path.reverse();
        path
    }

    let santa_path = path_to(input, "SAN");
    let you_path = path_to(input, "YOU");
    
    let common_path_length: usize = santa_path.iter()
        .zip(you_path.iter())
        .take_while(|(s, y)| s == y)
        .count();
    let path_to_santa = santa_path.len() + you_path.len() - 2 * common_path_length;

    info!("Number of orbital transfers to get to santa {}", path_to_santa);
}
