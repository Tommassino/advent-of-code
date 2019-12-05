use std::env;
use env_logger;
use std::time::Instant;

mod day5;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let now = Instant::now();
    day5::solve(&args[1]);
    println!("Took {} millis", now.elapsed().as_millis());
}
