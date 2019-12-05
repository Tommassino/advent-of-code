use std::env;
use env_logger;

mod day4;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day4::solve(&args[1]);
}
