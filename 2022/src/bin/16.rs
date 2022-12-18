use std::collections::HashMap;
use std::str::FromStr;

use petgraph::{Directed, Graph};
use petgraph::adj::DefaultIx;
use petgraph::algo::floyd_warshall;
use petgraph::graph::NodeIndex;
use regex::Regex;

#[derive(Debug)]
struct CaveSystem {
    flow_rates: HashMap<String, u32>,
    distances: HashMap<(String, String), u32>,
}

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut graph: Graph<(), (), Directed> = Graph::new();
        let mut nodes: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();
        let mut flow_rates: HashMap<String, u32> = HashMap::new();

        let pattern = Regex::new(r"Valve ([A-Z]+).*rate=(\d+);.*valves? ([A-Z, ]+)")
            .unwrap();

        input.lines().for_each(|line| {
            let captures = pattern.captures(line).unwrap();
            let valve: String = String::from(captures.get(1).unwrap().as_str());
            let flow_rate = captures.get(2)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .unwrap();
            let tunnels: Vec<String> = captures.get(3).unwrap()
                .as_str().split(", ").map(String::from).collect();
            // println!("Valve {:?}, flow_rate {:?}, tunnels {:?}", valve, flow_rate, tunnels);
            flow_rates.insert(valve.clone(), flow_rate);

            if !nodes.contains_key(&valve) {
                nodes.insert(valve.clone(), graph.add_node(()));
            }
            let valve_node = *nodes.get(&valve).unwrap();

            tunnels.iter().for_each(|x| {
                if !nodes.contains_key(x) {
                    nodes.insert(x.clone(), graph.add_node(()));
                }
                let tunnel_node = *nodes.get(&x.clone()).unwrap();
                graph.add_edge(valve_node, tunnel_node, ());
            })
        });

        let res = floyd_warshall(
            &graph,
            |_| 1,
        ).expect("");
        let mut distances: HashMap<(String, String), u32> = HashMap::new();
        nodes.iter().for_each(|from| {
            nodes.iter().for_each(|to| {
                if let Some(distance) = res.get(&(*from.1, *to.1)) {
                    distances.insert(
                        (from.0.clone(), to.0.clone()),
                        *distance as u32,
                    );
                }
            });
        });

        Ok(CaveSystem {
            flow_rates,
            distances,
        })
    }
}

impl CaveSystem {
    pub fn turn_on_valves(&self, time_remaining: i64) -> (i64, Vec<String>) {
        let mut path: Vec<String> = Vec::new();
        self.recurse(
            String::from("AA"),
            &mut path,
            time_remaining
        )
    }

    fn recurse(
        &self,
        node: String,
        path: &mut Vec<String>,
        remaining_time: i64,
    ) -> (i64, Vec<String>) {
        // println!("Arrived to node {:?} at minute {} after visiting {:?}", node, 30 - remaining_time, path);
        if remaining_time <= 0 {
            return (0, path.clone());
        }

        let current_flow_rate = *self.flow_rates.get(&node).unwrap();
        let (pressure_gain, time_in_node) =
            if current_flow_rate > 0 {
                // we expect the best path to be
                // "DD" at minute 2 to gain 560 pressure
                // "BB" at minute 5 to gain 325 pressure
                // "JJ" at minute 9 to gain 441 pressure
                // "HH" at minute 17 to gain 286 pressure
                // "EE" at minute 21 to gain 27 pressure
                // "CC" at minute 24 to gain 12 pressure
                // println!(
                //     "Turning on valve in node {:?} at minute {} to gain {:?} pressure",
                //     node,
                //     30 - remaining_time + 1,
                //     current_flow_rate as i64 * (remaining_time - 1)
                // );
                (current_flow_rate as i64 * (remaining_time - 1), 1)
            } else {
                (0, 0)
            };

        let mut best = pressure_gain;
        let mut best_path = path.clone();
        for (next_node, &flow_rate) in self.flow_rates.iter() {
            if flow_rate == 0 || path.contains(next_node) {
                continue;
            }
            let distance = *self.distances.get(
                &(node.clone(), next_node.clone())
            ).unwrap() as i64;
            path.push(next_node.clone());
            let rec_result = self.recurse(
                next_node.clone(),
                path,
                remaining_time - time_in_node - distance,
            );
            if best < rec_result.0 + pressure_gain {
                best = rec_result.0 + pressure_gain;
                best_path = rec_result.1.clone();
            }
            path.pop();
            // println!("Current best from path {:?}: {:?}", path, rec_result);
        }
        (best, best_path)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let cave_system = CaveSystem::from_str(input).expect("");
    let (pressure_gain, best_path) = cave_system.turn_on_valves(30);
    println!("Best path is: {:?}", best_path);
    Some(pressure_gain)
}

pub fn part_two(input: &str) -> Option<i64> {
    let _cave_system = CaveSystem::from_str(input).expect("");
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16, None);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16, None);
        assert_eq!(part_two(&input), Some(1707));
    }
}
