use log::{debug, info};
use std::fs;

fn fuel_required(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn fuel_required_recursive(mass: &i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel < 9 { //rest is handled by wishing really hard
        fuel
    } else {
        fuel + fuel_required_recursive(&fuel)
    }
}

pub fn solve(input_file: &str){
    let masses = parse(&input_file);

    debug!("{:?}", masses);

    part1(&masses);
    part2(&masses);
}

fn parse(input_file: &str) -> Vec<i32> {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    
    contents.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn part1(masses: &Vec<i32>) {
    let total: i32 = masses.iter().map(fuel_required).sum();
    info!("Total fuel required {}", total);
}

fn part2(masses: &Vec<i32>) {
    let total: i32 = masses.iter().map(fuel_required_recursive).sum();
    info!("Total fuel required {}", total);
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(&12), 2);
        assert_eq!(fuel_required(&14), 2);
        assert_eq!(fuel_required(&1969), 654);
        assert_eq!(fuel_required(&100756), 33583);
    }

    #[test]
    fn test_fuel_required_recursive() {
        assert_eq!(fuel_required_recursive(&14), 2);
        assert_eq!(fuel_required_recursive(&1969), 966);
        assert_eq!(fuel_required_recursive(&100756), 50346);
    }
}
