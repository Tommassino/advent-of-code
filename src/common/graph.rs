use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::min;
use std::hash::Hash;
use std::fmt::Debug;

pub trait Neighbors<S>
where S : Eq + Hash + Clone {
    fn neighbors(&self, state: S) -> Vec<S>;
}

pub fn bfs<G, S>(
    from: S, 
    to: S, 
    map: &G
) -> Result<(usize, S), usize> 
where S : Clone + Eq + Hash + Debug,
G : Neighbors<S>
{
    let mut visited: HashSet<S> = HashSet::new();
    let mut queue: VecDeque<(S, usize)> = VecDeque::new();

    queue.push_back((from, 0));

    while !queue.is_empty() {
        let (point, distance) = queue.pop_front().unwrap();
        
        for next_point in map.neighbors(point) {
            if next_point == to {
                return Ok((distance + 1, next_point));
            }
            if !visited.contains(&next_point) {
                visited.insert(next_point.clone());
                queue.push_back((next_point.clone(), distance + 1));
            }
        }
    }

    Err(0)
}
