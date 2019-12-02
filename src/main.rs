use std::env;
use env_logger;

mod day2;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day2::solve(&args[1]);
}
