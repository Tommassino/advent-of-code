use std::env;
use env_logger;

mod day1;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day1::solve(&args[1]);
}
