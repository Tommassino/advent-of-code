use std::env;
use env_logger;

mod day1;
mod day4;
mod day5;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day5::solve(&args[1]);
}
