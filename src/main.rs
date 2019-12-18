use std::env;
use env_logger;
use std::time::Instant;

mod day17;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day17::solve(&args[1]);
}
