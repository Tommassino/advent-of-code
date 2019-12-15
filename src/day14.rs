use log::{debug, info};
use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::AddAssign;
use num::integer::lcm;
use itertools::izip;
use std::cmp::min;

#[derive(Debug, Clone)]
struct Reaction{
    inputs: Vec<(usize, String)>,
    output: (usize, String)
}

#[derive(Debug, Clone)]
struct Refinery{
    reactions: HashMap<String, Reaction>
}

impl Refinery{
    fn from_string(contents: &str) -> Refinery {
        let mut reactions: HashMap<String, Reaction> = HashMap::new();

        contents.lines().for_each(|line| {
            let mut split = line.split(" => ");
            let inputs: Vec<(usize, String)> = split.next().unwrap().split(", ").map(|elem| {
                let split: Vec<&str> = elem.split(" ").collect();
                let count = split[0].parse::<usize>().unwrap();
                let element = String::from(split[1]);
                (count, element)
            }).collect();
            let output_parts: Vec<&str> = split.next().unwrap().split(" ").collect();
            let output_count = output_parts[0].parse::<usize>().unwrap();
            let output_element = String::from(output_parts[1]);

            if reactions.contains_key(&output_element) {
                panic!("reaction for {} already exists: {:?}", output_element, reactions.get(&output_element));
            }

            let reaction = Reaction{
                inputs: inputs,
                output: (output_count, output_element.clone())
            };
            reactions.insert(output_element, reaction);
        });

        Refinery{
            reactions: reactions
        }
    }

    fn reactions_for(&self, output: &String) -> &Reaction {
        self.reactions.get(output).unwrap()
    }

    fn produce_fuel(&self, count: usize) -> usize {
        let mut production_queue: VecDeque<(usize, String)> = VecDeque::new();
        let mut byproducts: HashMap<String, usize> = HashMap::new();
        production_queue.push_back((count, String::from("FUEL")));
        let mut ore_required = 0;
    
        while !production_queue.is_empty() {
            let (queued, element) = production_queue.pop_front().unwrap();
            
            let to_produce = match byproducts.get_mut(&element) {
                Some(existing_byproduct) => {
                    let to_use = min(*existing_byproduct, queued);
                    *existing_byproduct -= to_use;
                    debug!("Used {} existing byproduct of {}", to_use, element);
                    queued - to_use
                },
                None => queued
            };
    
            if to_produce == 0 {
                continue;
            }
    
            let reaction = self.reactions_for(&element);
            let reaction_count = (to_produce + reaction.output.0 - 1) / reaction.output.0;
            let byproduct = reaction_count * reaction.output.0 - to_produce;
            debug!(
                "Producing {}x{} with reaction {:?} {} times with {} byproduct", 
                element, 
                to_produce,  
                reaction, 
                reaction_count, 
                byproduct
            );
            assert!(reaction_count > 0, "Reaction count was zero");
    
            reaction.inputs.iter().for_each(|(count, input_element)| {
                if input_element != "ORE" {
                    production_queue.push_back((count * reaction_count, input_element.clone()));
                } else {
                    ore_required += count * reaction_count;
                }
            });
            if byproduct > 0 {
                let current = byproducts.get(&element).unwrap_or(&0);
                byproducts.insert(element, byproduct + current);
            }
        }

        ore_required
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = Refinery::from_string(&contents);
    debug!("{:?}", input);

    part1(&input);
    part2(&input);
}

fn part1(refinery: &Refinery) {
    let ore_required = refinery.produce_fuel(1);
    println!("It took {} ORE to produce FUEL", ore_required);
}

fn part2(refinery: &Refinery) {
    let max_ore = 1000000000000usize;
    let max_ore_required = refinery.produce_fuel(1);
    //upper bound for ore required
    let mut to_produce = 1;

    loop {
        let ore_required = refinery.produce_fuel(to_produce);
        let batches = max_ore / ore_required;
        let single_batches = (max_ore - ore_required * batches) / max_ore_required;
        if batches <= 1 && single_batches == 0 {
            break
        }
        to_produce = batches * to_produce + single_batches;
    }
    println!("Its possible to produce {} FUEL with {} ORE", to_produce, max_ore);
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger;

    #[test]
    fn simple() {
        env_logger::init();
        let input = r#"
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#.trim();
        let refinery = Refinery::from_string(input);
        part1(&refinery);
    }

    #[test]
    fn more() {
        env_logger::init();
        let input = r#"
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#.trim();
        let refinery = Refinery::from_string(input);
        part1(&refinery);
        part2(&refinery);
    }
}