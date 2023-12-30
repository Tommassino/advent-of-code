advent_of_code::solution!(25);
use graphrs::algorithms::community::louvain::louvain_partitions;
use graphrs::{Edge, Graph, GraphSpecs};

fn parse_input(input: &str) -> Graph<&str, ()> {
    let mut graph = Graph::<&str, ()>::new(GraphSpecs::undirected_create_missing());
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap().strip_suffix(':').unwrap();
        for to in parts {
            graph.add_edge(Edge::new(from, to)).unwrap();
        }
    }
    graph
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_input(input);
    let partitions = louvain_partitions(&graph, false, Some(0f64), Some(5f64), None).unwrap();
    assert_eq!(partitions.len(), 1);
    assert_eq!(partitions[0].len(), 2);
    let result = partitions[0].iter().map(|x| x.len()).product();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
