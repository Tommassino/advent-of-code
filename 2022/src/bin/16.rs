use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

use petgraph::{Directed, Graph};
use petgraph::adj::DefaultIx;
use petgraph::algo::floyd_warshall;
use petgraph::graph::NodeIndex;
use regex::Regex;

#[derive(Debug)]
struct CaveSystem {
    flow_rates: HashMap<u16, u32>,
    distances: HashMap<(u16, u16), u32>
}

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut graph: Graph<(), (), Directed> = Graph::new();
        let mut nodes: HashMap<u16, NodeIndex<DefaultIx>> = HashMap::new();
        let mut flow_rates: HashMap<u16, u32> = HashMap::new();

        let pattern = Regex::new(r"Valve ([A-Z]+).*rate=(\d+);.*valves? ([A-Z, ]+)")
            .unwrap();

        input.lines().for_each(|line| {
            let captures = pattern.captures(line).unwrap();
            let valve: String = String::from(captures.get(1).unwrap().as_str());
            let valve_id = CaveSystem::valve_id(&valve);
            let flow_rate = captures.get(2)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .unwrap();
            let tunnels: Vec<String> = captures.get(3).unwrap()
                .as_str().split(", ").map(String::from).collect();
            // println!("Valve {:?}, flow_rate {:?}, tunnels {:?}", valve, flow_rate, tunnels);
            flow_rates.insert(valve_id, flow_rate);

            if !nodes.contains_key(&valve_id) {
                nodes.insert(valve_id, graph.add_node(()));
            }
            let valve_node = *nodes.get(&valve_id).unwrap();

            tunnels.iter().for_each(|x| {
                let x_id = CaveSystem::valve_id(x);
                if !nodes.contains_key(&x_id) {
                    nodes.insert(x_id, graph.add_node(()));
                }
                let tunnel_node = *nodes.get(&x_id).unwrap();
                graph.add_edge(valve_node, tunnel_node, ());
            })
        });

        let res = floyd_warshall(
            &graph,
            |_| 1,
        ).expect("");
        let mut distances: HashMap<(u16, u16), u32> = HashMap::new();
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
            distances
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct IterState{
    position: u16,
    visited: BTreeSet<u16>,
    time: u32,
    released_pressure: u32
}

impl CaveSystem {
    pub fn valve_id(node_name: &str) -> u16 {
        let bytes = node_name.as_bytes();
        let first = (bytes[0] as u32 - 'A' as u32) as u16;
        let second = (bytes[1] as u32 - 'A' as u32) as u16;
        first << 8 | second
    }

    pub fn open_valves(&self, max_time: u32) -> u32 {
        let state = IterState{
            position: CaveSystem::valve_id("AA"),
            visited: Default::default(),
            time: max_time,
            released_pressure: 0
        };
        let mut cache: HashMap<IterState, u32> = HashMap::new();
        self.turn_valves(&state, &mut cache)
    }

    pub fn open_valves_double(&self, max_time: u32) -> u32 {
        let state = IterState{
            position: CaveSystem::valve_id("AA"),
            visited: Default::default(),
            time: max_time,
            released_pressure: 0
        };
        let mut cache: HashMap<IterState, u32> = HashMap::new();
        self.turn_valves_double(&state, max_time, &mut cache)
    }

    pub fn turn_valves(&self, state: &IterState, cache: &mut HashMap<IterState, u32>) -> u32 {
        if let Some(result) = cache.get(state) {
            return *result;
        }
        let result = self.next(&state)
            .map(|next_state| self.turn_valves(&next_state, cache))
            .max()
            .unwrap_or(state.released_pressure);
        cache.insert(state.clone(), result);
        result
    }

    pub fn turn_valves_double(
        &self,
        state: &IterState,
        max_time: u32,
        cache: &mut HashMap<IterState, u32>
    ) -> u32 {
        if let Some(&result) = cache.get(state) {
            return result;
        }
        if state.time <= 1 {
            return state.released_pressure;
        }
        if state.time == 10 {
            println!("{:?}", state);
        }
        // either we stop here and let the elephant take over
        let mut elephant = state.clone();
        elephant.position = CaveSystem::valve_id("AA");
        elephant.time = max_time;
        let mut elephant_cache: HashMap<IterState, u32> = HashMap::new();
        let elephant_pressure = self.turn_valves(&elephant, &mut elephant_cache);
        // or we continue recursively
        let recursive_max = self.next(&state)
            .map(|next_state| self.turn_valves_double(&next_state, max_time, cache))
            .max()
            .unwrap_or(state.released_pressure);
        let result = elephant_pressure.max(recursive_max);
        cache.insert(state.clone(), result);
        result
    }

    pub fn next<'a>(
        &'a self,
        state: &'a IterState
    ) -> impl Iterator<Item = IterState> + 'a {
        self.distances
            .iter()
            .filter_map(move |((from, to), distance)| {
                let flow_rate = self.flow_rates[to];
                if state.position == *from && !state.visited.contains(&to) && state.time > *distance && flow_rate > 0 {
                    let mut next_state = state.clone();
                    next_state.position = *to;
                    next_state.time -= distance + 1;
                    next_state.visited.insert(*to);
                    next_state.released_pressure += flow_rate * next_state.time;
                    Some(next_state)
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ActorState {
    position: u16,
    time_left: i64
}

pub fn part_one(input: &str) -> Option<u32> {
    let cave_system = CaveSystem::from_str(input).expect("");
    Some(cave_system.open_valves(30))
}

pub fn part_two(input: &str) -> Option<u32> {
    let cave_system = CaveSystem::from_str(input).expect("");
    Some(cave_system.open_valves_double(26))
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

    #[test]
    fn test_recursion_floor() {
        let input = advent_of_code::read_file("examples", 16, None);
        let cave_system = CaveSystem::from_str(&input).expect("");
        let state = IterState{
            position: CaveSystem::valve_id("DD"),
            visited: vec!["BB", "DD", "EE", "HH", "JJ"].iter()
                .map(|x| CaveSystem::valve_id(x))
                .collect(),
            time: 8,
            released_pressure: 0
        };
        let mut cache: HashMap<IterState, u32> = HashMap::new();
        assert_eq!(cave_system.turn_valves(&state, &mut cache), 12);
    }
}
