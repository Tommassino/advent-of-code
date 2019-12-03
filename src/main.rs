use std::env;
use env_logger;

mod day3;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day3::solve(&args[1]);
}
