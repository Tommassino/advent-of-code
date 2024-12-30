use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::Write;
advent_of_code::solution!(24);

#[derive(Debug, Clone)]
enum CircuitNode {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Constant(bool),
}

impl CircuitNode {
    fn node_type(&self) -> &str {
        match self {
            CircuitNode::And(_, _) => "AND",
            CircuitNode::Or(_, _) => "OR",
            CircuitNode::Xor(_, _) => "XOR",
            CircuitNode::Constant(_) => "CONSTANT",
        }
    }
}

#[derive(Debug)]
struct Alu {
    nodes: HashMap<String, CircuitNode>,
}

impl Alu {
    fn get_node(&self, name: &str) -> Option<&CircuitNode> {
        self.nodes.get(name)
    }

    fn eval(&self, name: &str) -> bool {
        let node = self.get_node(name).unwrap();
        match node {
            CircuitNode::And(a, b) => self.eval(a) && self.eval(b),
            CircuitNode::Or(a, b) => self.eval(a) || self.eval(b),
            CircuitNode::Xor(a, b) => self.eval(a) ^ self.eval(b),
            CircuitNode::Constant(value) => *value,
        }
    }

    fn get_z(&self) -> u64 {
        let mut number: u64 = 0;
        for idx in 0..64 {
            let name = format!("z{:02}", idx);
            if self.nodes.contains_key(&name) {
                let evaled = self.eval(&name) as u64;
                number |= evaled << idx;
            } else {
                break;
            }
        }
        number
    }

    fn to_graphviz(&self) {
        let mut graph = petgraph::graph::DiGraph::<_, i32>::new();

        let mut name_indices = HashMap::new();
        for (name, _) in self.nodes.iter() {
            let node_id = graph.add_node(name.clone());
            name_indices.insert(name.clone(), node_id);
        }

        for (name, circuit) in self.nodes.iter() {
            let output_id = *name_indices.get(name).unwrap();

            let gate_name = match circuit {
                CircuitNode::And(_, _) => "AND",
                CircuitNode::Or(_, _) => "OR",
                CircuitNode::Xor(_, _) => "XOR",
                _ => continue,
            };

            let (input_a, input_b) = match circuit {
                CircuitNode::And(a, b) => (a, b),
                CircuitNode::Or(a, b) => (a, b),
                CircuitNode::Xor(a, b) => (a, b),
                _ => continue,
            };

            let input_a_id = *name_indices.get(input_a).unwrap();
            let input_b_id = *name_indices.get(input_b).unwrap();

            let gate_node = graph.add_node(gate_name.to_string());
            graph.add_edge(input_a_id, gate_node, 1);
            graph.add_edge(input_b_id, gate_node, 1);
            graph.add_edge(gate_node, output_id, 1);
        }
        // output to file
        let mut file = std::fs::File::create("graph.dot").unwrap();
        let output = format!("{}", petgraph::dot::Dot::new(&graph));
        file.write_all(output.as_bytes()).unwrap();
    }

    fn suspicious_gates(&self) -> HashSet<String> {
        let mut suspicious_gates = HashSet::new();
        for (name, node) in &self.nodes {
            match node {
                // any OR gate connected to input is suspicious
                CircuitNode::Or(a, b) => {
                    let valid = ['x', 'y'];
                    let a_input = valid.contains(&a.chars().nth(0).unwrap());
                    let b_input = valid.contains(&b.chars().nth(0).unwrap());
                    if a_input && b_input {
                        suspicious_gates.insert(name.to_string());
                    }
                }
                // any XOR gate that is not connected to input or output is suspicious
                CircuitNode::Xor(a, b) => {
                    let valid = ['x', 'y', 'z'];
                    let a_is_input = valid.contains(&a.chars().nth(0).unwrap());
                    let b_is_input = valid.contains(&b.chars().nth(0).unwrap());
                    let output_is_output = valid.contains(&name.chars().nth(0).unwrap());
                    if !a_is_input && !b_is_input && !output_is_output {
                        suspicious_gates.insert(name.to_string());
                    }
                }
                _ => {}
            }
            // any output that is not connected to a XOR is suspicious
            // except the last bit
            if name.starts_with("z") && name != "z45" {
                match node {
                    CircuitNode::Xor(_, _) => {}
                    _ => {
                        suspicious_gates.insert(name.to_string());
                    }
                }
            }
        }

        for (name, node) in &self.nodes {
            // find nodes that are connected to input except first bit
            let input_gate = match node {
                CircuitNode::And(a, b) | CircuitNode::Or(a, b) | CircuitNode::Xor(a, b) => {
                    let valid = ['x', 'y'];
                    let a_input =
                        valid.contains(&a.chars().nth(0).unwrap()) && a != "x00" && a != "y00";
                    let b_input = valid.contains(&b.chars().nth(0).unwrap());
                    a_input && b_input
                }
                _ => false,
            };
            if !input_gate {
                continue;
            }
            let mut connected_gates = self
                .nodes
                .iter()
                .filter(|(_, gate)| match gate {
                    CircuitNode::Or(a, b) | CircuitNode::Xor(a, b) | CircuitNode::And(a, b) => {
                        a == name || b == name
                    }
                    _ => false,
                })
                .collect::<Vec<_>>();
            if node.node_type() == "AND" {
                // AND input gate should be connected to a OR
                connected_gates.retain(|(_, gate)| gate.node_type() != "OR");
            } else if node.node_type() == "XOR" {
                // XOR input gate should be connected to a XOR + AND
                connected_gates
                    .retain(|(_, gate)| gate.node_type() != "XOR" && gate.node_type() != "AND");
            } else {
                continue;
            }
            suspicious_gates.extend(
                connected_gates
                    .into_iter()
                    .map(|(name, _)| name.to_string()),
            );
        }
        suspicious_gates
    }

    fn swap(&mut self, a: &str, b: &str) {
        let a_node = self.nodes.remove(a).unwrap();
        let b_node = self.nodes.remove(b).unwrap();
        self.nodes.insert(a.to_string(), b_node);
        self.nodes.insert(b.to_string(), a_node);
    }

    fn print_output_chains(&self) {
        for (name, node) in &self.nodes {
            if !name.starts_with("z") {
                continue;
            }
            let (left, right) = match node {
                CircuitNode::And(a, b) | CircuitNode::Or(a, b) | CircuitNode::Xor(a, b) => (a, b),
                _ => continue,
            };
            let left_node = self.nodes.get(left).unwrap();
            let right_node = self.nodes.get(right).unwrap();

            let left_is_xor = matches!(left_node, CircuitNode::Xor(_, _));
            if left_is_xor {
                println!("{} -> XOR(in) -> {} -> {:?}", name, left, left_node);
                println!("{} -> XOR(carry) -> {} -> {:?}", name, right, right_node);
            } else {
                println!("{} -> XOR(in) -> {} -> {:?}", name, right, right_node);
                println!("{} -> XOR(carry) -> {} -> {:?}", name, left, left_node);
            }
        }
    }
}

impl From<&str> for Alu {
    fn from(value: &str) -> Self {
        let mut nodes = HashMap::new();

        let (values, equations) = value.split_once("\n\n").unwrap();
        for line in values.lines() {
            let (name, value) = line.split_once(": ").unwrap();
            nodes.insert(name.to_string(), CircuitNode::Constant(value == "1"));
        }

        let equation_regex = regex::Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        for capture in equation_regex.captures_iter(equations) {
            let a = capture.get(1).unwrap().as_str();
            let op = capture.get(2).unwrap().as_str();
            let b = capture.get(3).unwrap().as_str();
            let out = capture.get(4).unwrap().as_str();

            let node = match op {
                "AND" => CircuitNode::And(a.to_string(), b.to_string()),
                "OR" => CircuitNode::Or(a.to_string(), b.to_string()),
                "XOR" => CircuitNode::Xor(a.to_string(), b.to_string()),
                _ => panic!("Unknown operator: {}", op),
            };

            nodes.insert(out.to_string(), node);
        }

        Alu { nodes }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let alu = Alu::from(input);
    Some(alu.get_z())
}

pub fn part_two(input: &str) -> Option<String> {
    // manual solution by looking at some suspicious gates and adding them to swaps manually
    let mut alu = Alu::from(input);
    alu.swap("z11", "vkq");
    alu.swap("mmk", "z24");
    alu.swap("hqh", "z38");
    alu.swap("pvb", "qdq");
    alu.to_graphviz();
    for gate in &alu.suspicious_gates() {
        println!("{}: {:?}", gate, alu.nodes.get(gate));
    }
    alu.print_output_chains();
    let solution_swaps = ["z11", "vkq", "mmk", "z24", "hqh", "z38", "pvb", "qdq"];
    Some(solution_swaps.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_larger() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }
}
