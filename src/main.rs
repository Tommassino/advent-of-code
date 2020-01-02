#[macro_use] extern crate cached;

use std::env;
use env_logger;
use std::time::Instant;

mod common;
mod day23;
use day23::solve;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    solve(&args[1]);
}
