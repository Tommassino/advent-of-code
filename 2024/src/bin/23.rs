use itertools::Itertools;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(23);

/*
\--- Day 23: LAN Party ---
----------

As The Historians wander around a secure area at Easter Bunny HQ, you come across posters for a [LAN party](https://en.wikipedia.org/wiki/LAN_party) scheduled for today! Maybe you can find it; you connect to a nearby [datalink port](/2016/day/9) and download a map of the local network (your puzzle input).

The network map provides a list of every *connection between two computers*. For example:

```
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn

```

Each line of text in the network map represents a single connection; the line `kh-tc` represents a connection between the computer named `kh` and the computer named `tc`. Connections aren't directional; `tc-kh` would mean exactly the same thing.

LAN parties typically involve multiplayer games, so maybe you can locate it by finding groups of connected computers. Start by looking for *sets of three computers* where each computer in the set is connected to the other two computers.

In this example, there are `12` such sets of three inter-connected computers:

```
aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq

```

If the Chief Historian is here, *and* he's at the LAN party, it would be best to know that right away. You're pretty sure his computer's name starts with `t`, so consider only sets of three computers where at least one computer's name starts with `t`. That narrows the list down to `*7*` sets of three inter-connected computers:

```
co,de,ta
co,ka,ta
de,ka,ta
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn

```

Find all the sets of three inter-connected computers. *How many contain at least one computer with a name that starts with `t`?*


\--- Part Two ---
----------

There are still way too many results to go through them all. You'll have to find the LAN party another way and go there yourself.

Since it doesn't seem like any employees are around, you figure they must all be at the LAN party. If that's true, the LAN party will be the *largest set of computers that are all connected to each other*. That is, for each computer at the LAN party, that computer will have a connection to every other computer at the LAN party.

In the above example, the largest set of computers that are all connected to each other is made up of `co`, `de`, `ka`, and `ta`. Each computer in this set has a connection to every other computer in the set:

```
ka-co
ta-co
de-co
ta-ka
de-ta
ka-de

```

The LAN party posters say that the *password* to get into the LAN party is the name of every computer at the LAN party, sorted alphabetically, then joined together with commas. (The people running the LAN party are clearly a bunch of nerds.) In this example, the password would be `*co,de,ka,ta*`.

*What is the password to get into the LAN party?*

 */

#[derive(Debug)]
struct Network {
    connections: HashMap<String, HashSet<String>>,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut connections = HashMap::new();
        for line in value.lines() {
            let mut parts = line.split('-');
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            connections
                .entry(a.clone())
                .or_insert(HashSet::new())
                .insert(b.clone());
            connections
                .entry(b.clone())
                .or_insert(HashSet::new())
                .insert(a.clone());
        }
        Self { connections }
    }
}

impl Network {
    fn connected(&self, a: &str, b: &str) -> bool {
        self.connections[a].contains(b)
    }

    fn find_groups(&self) -> Vec<Vec<String>> {
        let mut groups = vec![];
        for (a, connections) in &self.connections {
            for b in connections {
                for c in connections {
                    if b != c && self.connected(b, c) {
                        groups.push(vec![a.clone(), b.clone(), c.clone()]);
                    }
                }
            }
        }
        groups
    }

    fn find_largest_group(&self) -> Vec<String> {
        // use bron kerbosh
        let r = HashSet::new();
        let p = self.connections.keys().cloned().collect();
        let x = HashSet::new();
        self.bron_kerbosh(&r, &p, &x).into_iter().collect()
    }

    fn bron_kerbosh(
        &self,
        r: &HashSet<String>,
        p: &HashSet<String>,
        x: &HashSet<String>,
    ) -> HashSet<String> {
        if p.is_empty() && x.is_empty() {
            // found a clique
            return r.clone();
        }
        //choose pivot
        let u = p.union(x).next().unwrap().clone();
        let mut largest_result = HashSet::new();
        let mut p = p.clone();
        let mut x = x.clone();
        for v in p.clone() {
            if self.connected(&u, &v) {
                continue;
            }
            let mut r_v = r.clone();
            r_v.insert(v.clone());
            let recursive_result = self.bron_kerbosh(
                &r_v,
                &p.intersection(&self.connections[&v]).cloned().collect(),
                &x.intersection(&self.connections[&v]).cloned().collect(),
            );
            if recursive_result.len() > largest_result.len() {
                largest_result = recursive_result;
            }
            p.remove(&v);
            x.insert(v);
        }
        largest_result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let network = Network::from(input);
    let groups = network.find_groups();
    let count = groups
        .iter()
        .filter(|group| group.iter().any(|name| name.starts_with('t')))
        .count()
        / 6;
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let network = Network::from(input);
    let group = network.find_largest_group();
    let password = group.iter().sorted().join(",");
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
