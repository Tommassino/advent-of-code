use std::collections::VecDeque;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BotResource {
    Clay = 0,
    Ore = 1,
    Obsidian = 2,
    Geode = 3
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    idx: usize,
    bot_costs: [[usize; 3]; 4],
    max_bots: [usize; 4]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct FactoryState {
    resources: [usize; 4],
    bots: [usize; 4],
    time_remaining: usize
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
            .expect("");
        let captures = pattern.captures(s).expect("");
        let idx = captures.get(1)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let ore_bot_ore_cost = captures.get(2)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let clay_bot_ore_cost = captures.get(3)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let obsidian_bot_ore_cost = captures.get(4)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let obsidian_bot_clay_cost = captures.get(5)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let geode_bot_ore_cost = captures.get(6)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();
        let geode_bot_obsidian_cost = captures.get(7)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap();


        Ok(Blueprint {
            idx,
            // clay, ore, obsidian
            bot_costs: [
                [0, clay_bot_ore_cost, 0],
                [0, ore_bot_ore_cost, 0],
                [obsidian_bot_clay_cost, obsidian_bot_ore_cost, 0],
                [0, geode_bot_ore_cost, geode_bot_obsidian_cost]
            ],
            max_bots: [
                obsidian_bot_clay_cost,
                clay_bot_ore_cost.max(ore_bot_ore_cost).max(obsidian_bot_ore_cost).max(geode_bot_ore_cost),
                geode_bot_obsidian_cost,
                usize::MAX
            ]
        })
    }
}

impl FactoryState {
    pub fn new(time_remaining: usize) -> FactoryState {
        FactoryState {
            resources: [0, 0, 0, 0],
            bots: [0, 1, 0, 0],
            time_remaining
        }
    }

    pub fn produce(&mut self, time: usize) {
        self.bots.iter().enumerate().for_each(|(idx, num)| {
            self.resources[idx] += num * time;
        });
        self.time_remaining -= time;
    }

    pub fn time_to_construct(&self, bot_type: BotResource, blueprint: &Blueprint) -> usize {
        let costs = blueprint.bot_costs[bot_type as usize];
        let result = costs.iter().enumerate().map(|(resource, cost)|{
            let current_stock = self.resources[resource];
            let production = self.bots[resource];
            if current_stock >= *cost {
                0
            } else if production == 0 {
                usize::MAX
            } else {
                let x = cost - current_stock;
                x / production + usize::from(x % production != 0)
            }
        }).max().unwrap();
        // println!("Time to build {:?} with factory {:?} and cost {:?}: {:?}", bot_type, self, costs, result);
        result
    }

    pub fn should_build(&self, bot_type: BotResource, blueprint: &Blueprint) -> bool {
        // if we produce more than the cost of every bot per tick, there is no point in building this bot
        if bot_type != BotResource::Geode {
            self.bots[bot_type as usize] < blueprint.max_bots[bot_type as usize]
        } else {
            true
        }
    }

    pub fn build(&mut self, bot_type: BotResource, blueprint: &Blueprint) {
        let costs = blueprint.bot_costs[bot_type as usize];
        // println!("Building {:?} with resources {:?} and cost {:?}", bot_type, self.resources, costs);
        costs.iter().enumerate().for_each(|(idx, cost)| {
            self.resources[idx] -= cost;
        });
        self.bots[bot_type as usize] += 1;
    }
}


impl Blueprint {
    pub fn optimize_factory(&self, time_limit: usize) -> usize {
        let mut stack: VecDeque<FactoryState> = VecDeque::new();
        stack.push_back(FactoryState::new(time_limit));
        let mut best_production = 0;
        while let Some(state) = stack.pop_front() {
            // in each step decide on a bot to construct and wait for resource
            for bot_type in [BotResource::Geode, BotResource::Obsidian, BotResource::Clay, BotResource::Ore] {
                let time_to_construct = state.time_to_construct(bot_type, self);
                let should_construct = state.should_build(bot_type, self);
                // if bot_type == BotResource::GEODE {
                //     println!("{:?}: {:?} {:?}", state, time_to_construct, should_construct);
                // }
                if should_construct && time_to_construct < state.time_remaining {
                    let mut bot_state = state;
                    //need to take at least 1 minute to construct a bot
                    bot_state.produce(time_to_construct + 1);
                    bot_state.build(bot_type, self);
                    // println!("Created bot {:?} with state {:?}", bot_type, bot_state);
                    if bot_state.time_remaining > 1 {
                        stack.push_back(bot_state);
                    } else {
                        best_production = best_production.max(
                            bot_state.time_remaining * bot_state.bots[BotResource::Geode as usize] +
                                bot_state.resources[BotResource::Geode as usize]
                        )
                    }
                }
            }
            // this is when no bot can be built, just wait till completion
            best_production = best_production.max(
                state.time_remaining * state.bots[BotResource::Geode as usize] +
                    state.resources[BotResource::Geode as usize]
            );
            // println!("Stack size {:?}", stack.len());
        }
        best_production
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input.lines()
        .map(|x| Blueprint::from_str(x).expect(""))
        .map(|blueprint| {
            blueprint.optimize_factory(24) * blueprint.idx
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input.lines()
        .map(|x| Blueprint::from_str(x).expect(""))
        .take(3)
        .map(|blueprint| {
            blueprint.optimize_factory(32)
        })
        .product();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19, None);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19, None);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_should_build() {
        let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.")
            .expect("");
        let mut factory = FactoryState::new(24);
        for bot_type in [BotResource::Obsidian, BotResource::Clay, BotResource::Ore] {
            factory.bots[bot_type as usize] = blueprint.bot_costs.iter()
                .map(|costs| costs[bot_type as usize]).max().unwrap();
            let should_build = factory.should_build(bot_type, &blueprint);
            println!("{:?}, {:?}, {:?}", factory, blueprint, should_build);
            assert_eq!(should_build, false);
        }
    }

    #[test]
    fn test_time_to_construct() {
        let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.")
            .expect("");
        let mut factory = FactoryState::new(24);
        assert_eq!(factory.time_to_construct(BotResource::Ore, &blueprint), 4);
        factory.bots[BotResource::Ore as usize] += 1;
        assert_eq!(factory.time_to_construct(BotResource::Ore, &blueprint), 2);
        factory.produce(1);
        assert_eq!(factory.time_to_construct(BotResource::Ore, &blueprint), 1);
        factory.produce(1);
        assert_eq!(factory.time_to_construct(BotResource::Ore, &blueprint), 0);
        factory.produce(1);
        assert_eq!(factory.time_to_construct(BotResource::Ore, &blueprint), 0);
        assert_eq!(factory.time_to_construct(BotResource::Geode, &blueprint), usize::MAX);
    }
}
