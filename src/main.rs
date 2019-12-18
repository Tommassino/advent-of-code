#[macro_use] extern crate cached;

use std::env;
use env_logger;
use std::time::Instant;

mod day18;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    day18::solve(&args[1]);
}
