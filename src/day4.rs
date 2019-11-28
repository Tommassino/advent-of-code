use log::{debug, info};
use std::fs;
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;
use rulinalg::utils;
use std::ops::Add;

#[derive(Clone, Debug)]
struct GuardInfo {
    pub guard_id: u16,
    pub sleep: Vec<u32>
}

impl<'a, 'b> Add<&'b GuardInfo> for &'a GuardInfo {
    type Output = GuardInfo;

    fn add(self, other: &'b GuardInfo) -> GuardInfo {
        let mut added = vec![0; 60];
        for i in 0..60{
            added[i] = self.sleep[i] + other.sleep[i];
        }

        GuardInfo {
            guard_id: self.guard_id,
            sleep: added,
        }
    }
}


pub fn solve(input_file: &str) {
    let parsed_actions = parse(input_file);
    part1(&parsed_actions);
    part2(&parsed_actions);
}

fn parse(input_file: &str) -> Vec<GuardInfo> {
    let action_regexes: HashMap<u8, Regex> = vec![
        (0u8, Regex::new(r"^\s*\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] Guard #(\d+) begins shift\s*$").unwrap()),
        (1u8, Regex::new(r"^\s*\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] falls asleep\s*$").unwrap()),
        (2u8, Regex::new(r"^\s*\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] wakes up\s*$").unwrap())
    ].into_iter().collect();
    
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut last_guard = Option::<u16>::None;
    let mut last_asleep = Option::<u16>::None;
    let mut sleep = vec![0u32; 60];
    let mut shifts = Vec::<GuardInfo>::new();
    
    for line in contents.split("\n"){
        for (action, action_regex) in action_regexes.iter() {
            if action_regex.is_match(line) {
                let groups = action_regex.captures(line).unwrap();
                let action_time = groups[1].parse::<u16>().unwrap();
                match action {
                    0 => {
                        if last_guard.is_some() {
                            let shift = GuardInfo{
                                guard_id: last_guard.unwrap(),
                                sleep: sleep.clone()
                            };
                            debug!("{:?}", &shift);
                            shifts.push(shift);
                            for i in 0..60 {
                                sleep[i] = 0;
                            }
                        }
                        last_guard = Some(groups[2].parse::<u16>().unwrap());
                    }
                    1 => last_asleep = Some(action_time),
                    2 => {
                        for i in last_asleep.unwrap()..action_time {
                            sleep[i as usize] = 1u32;
                        }
                    }
                    _ => {}
                }
                break;
            }
        }
    }
    
    if last_guard.is_some() {
        let shift = GuardInfo{
            guard_id: last_guard.unwrap(),
            sleep: sleep
        };
        debug!("{:?}", &shift);
        shifts.push(shift);
    }

    shifts.sort_by_key(|x| x.guard_id);
    let mut totals = Vec::<GuardInfo>::new();
    for (guard, group) in &shifts.iter().group_by(|shift| shift.guard_id) {
        let guard_shifts = group.collect::<Vec<&GuardInfo>>();
        debug!("{} {:?}", guard, guard_shifts);
        
        let mut guard_total = guard_shifts[0].clone();
        for x in guard_shifts[1..].iter() {
            guard_total = guard_total.add(x);
        }
        totals.push(guard_total)
    }
    totals
}

fn part1(guards: &Vec<GuardInfo>) {
    let mut guard_id = Option::<u16>::None;
    let mut minutes_asleep = Option::<u32>::None;
    let mut best_minute = Option::<usize>::None;

    for guard_info in guards.iter() {
        let max = utils::argmax(&guard_info.sleep);
        let total: u32 = (&guard_info.sleep).into_iter().sum();

        if total > minutes_asleep.unwrap_or(0) {
            guard_id = Some(guard_info.guard_id);
            minutes_asleep = Some(total);
            best_minute = Some(max.0);
        }
    }

    info!("Best guard {:?}, total minutes asleep {:?}, best minute {:?}", guard_id, minutes_asleep, best_minute);
    info!("Answer is {}", guard_id.unwrap() as u32 * best_minute.unwrap() as u32);
}

fn part2(guards: &Vec<GuardInfo>) {
    let mut guard_id = Option::<u16>::None;
    let mut repeated_minute = Option::<u32>::None;
    let mut best_minute = Option::<usize>::None;

    for guard_info in guards.iter() {
        let max = utils::argmax(&guard_info.sleep);

        if max.1 > repeated_minute.unwrap_or(0) {
            guard_id = Some(guard_info.guard_id);
            repeated_minute = Some(max.1);
            best_minute = Some(max.0);
        }
    }

    info!("Best guard {:?}, asleep on best minute times {:?}, best minute {:?}", guard_id, repeated_minute, best_minute);
    info!("Answer is {}", guard_id.unwrap() as u32 * best_minute.unwrap() as u32);
}