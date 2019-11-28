use log::{debug, info};
use std::fs;
use std::char;
use std::ascii::AsciiExt;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub fn solve(input_file: &str) {
    let parsed_units = parse(input_file);
    part1(&parsed_units);
    part2(&parsed_units);
}

fn parse(input_file: &str) -> Vec<char> {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file")
        .chars()
        .collect();

    debug!("{:?}", contents);
    contents
}

fn part1(data: &Vec<char>) {
    let reacted = react(data);
    info!("Final polymer length is {}", reacted.len());
}

fn part2(data: &Vec<char>) {
    let mut best_length = data.len();
    let mut best_unit = 'a';
    for candidate in ASCII_LOWER.iter() {
        let mut buffer = data.clone();
        let candidate_higher = candidate.to_ascii_uppercase();
        debug!("removing unit {} and {}", candidate, candidate_higher);
        let mut i = 0;
        while i < buffer.len() {
            if buffer[i] == *candidate || buffer[i] == candidate_higher {
                buffer.remove(i);
            } else {
                i += 1;
            }
        }
        let reacted = react(&buffer);
        debug!("reacted polymer {:?}", reacted);
        if reacted.len() < best_length {
            best_length = reacted.len();
            best_unit = *candidate;
        }
    }
    info!("best elimination is {} for a reacted length of {}", best_unit, best_length);
}

fn react(data: &Vec<char>) -> Vec<char> {
    let mut buffer = data.clone();
    let mut i = 0;
    while i + 1 < buffer.len() {
        let first = buffer[i];
        let second = buffer[i+1];
        if first.to_ascii_uppercase() == second.to_ascii_uppercase() && first != second {
            buffer.remove(i);
            buffer.remove(i);
            debug!("eliminated {} with {} at {}", first, second, i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    buffer
}