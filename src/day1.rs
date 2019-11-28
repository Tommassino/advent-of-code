use log::{debug, info};
use std::fs;
use std::collections::HashSet;

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    let changes = parse(&contents);

    part1(&changes);
    part2(&changes);
}

#[derive(Debug)]
struct FrequencyChange{
    pub direction: i8,
    pub amount: i32
}

fn part1(changes: &Vec<FrequencyChange>) {
    let mut total: i32 = 0;
    for i in changes {
        total = total + (i.direction as i32) * i.amount;
    }
    info!("Final frequency is {}", total);
}

fn part2(changes: &Vec<FrequencyChange>) {
    let mut total: i32 = 0;
    let mut encountered: HashSet<i32> = HashSet::new();
    'outer: loop {
        for i in changes {
            total = total + (i.direction as i32) * i.amount;
            if encountered.contains(&total) {
                break 'outer;
            }
            encountered.insert(total);
        }
    }
    info!("First duplicate frequency is {}", total)
}

fn parse(changes: &str) -> Vec<FrequencyChange> {
    let mut parsed: Vec<FrequencyChange> = Vec::new();
    for token in changes.split("\n"){
        let mut direction: i8 = -1;
        if &token[0..1] == "+" {
            direction = 1;
        }
        let len = &token.len() + 0;
        let amount: i32 = token[1..len].parse::<i32>().unwrap();
        let change = FrequencyChange{
            direction: direction,
            amount: amount
        };
        debug!("{} {} {} {} {:?}", token, &token[1..len], len, direction, change);
        parsed.push(change)
    }
    parsed
}