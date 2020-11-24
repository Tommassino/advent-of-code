use log::{debug, info};
use std::fs;
use std::rc::Rc;
use std::iter;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::common::intcode::*;

#[derive(Debug, Clone)]
struct Packet {
    address: usize,
    values: Vec<i128>
}

struct Network {
    computers: Vec<Computer>
}

impl Network {
    fn new(program: &Program, computer_count: usize) -> Network {
        let computers: Vec<Computer> = iter::repeat_with(|| {
            Computer::new(&program)
        }).take(computer_count).collect();

        Network {
            computers: computers
        }
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

fn part1(nic: &Program) {
    let mut network = Network::new(nic, 50);
    
    
    let mut queue: VecDeque<Packet> = (0..network.computers.len())
        .map(|idx| {
            Packet {
                address: idx,
                values: vec![idx as i128]
            }
        })
        .collect();

    let result = 'outer: loop {
        while let Some(packet) = queue.pop_front() {
            let computer = &mut network.computers[packet.address];
            let output = computer.run(packet.values);
            debug!("{:?}", output);
            
            for chunk in output.chunks(3) {
                let packet = Packet{
                    address: chunk[0] as usize,
                    values: vec![chunk[1], chunk[2]]
                };
                debug!("Received packet {:?}", packet);
                if packet.address == 255 {
                    break 'outer packet;
                }
                queue.push_back(packet);
            }
        }

        info!("Network idle, pushing -1");
        //this means all computers are halted waiting for input
        queue
            .extend((0..network.computers.len())
            .map(|dest| Packet { address: dest, values: vec![-1] }));
    };
    
    println!("{:?}", result);
}

fn part2(nic: &Program) {
    let mut network = Network::new(nic, 50);
    
    
    let mut queue: VecDeque<Packet> = (0..network.computers.len())
        .map(|idx| {
            Packet {
                address: idx,
                values: vec![idx as i128]
            }
        })
        .collect();
    let mut seen_values: HashSet<i128> = HashSet::new();
    let mut nat: Option<Packet> = None;

    let result = 'outer: loop {
        while let Some(packet) = queue.pop_front() {
            let computer = &mut network.computers[packet.address];
            let output = computer.run(packet.values);
            debug!("{:?}", output);
            
            for chunk in output.chunks(3) {
                let packet = Packet{
                    address: chunk[0] as usize,
                    values: vec![chunk[1], chunk[2]]
                };
                debug!("Received packet {:?}", packet);
                if packet.address == 255 {
                    nat = Some(packet);
                } else {
                    queue.push_back(packet);
                }
            }
        }

        match nat.clone() {
            Some(to_send) => {
                info!("Network idle, pushing {:?}", to_send);
                if let Some(y) = seen_values.replace(to_send.values[1]) {
                    break 'outer y;
                }
                queue.push_back(Packet{
                    address: 0,
                    values: to_send.values
                });
            },
            None => {
                queue
                    .extend((0..network.computers.len())
                    .map(|dest| Packet { address: dest, values: vec![-1] }));
            }
        }
    };
    
    println!("{:?}", result);
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger::*;

    #[test]
    fn test_mod_exponent() {
    }
}
