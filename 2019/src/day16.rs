
use log::{debug, info};
use std::fs;
use std::char;
use std::fmt;
use std::fmt::Display;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Signal{
    parts: Vec<isize>
}

impl Signal{
    fn from_string(input: &str) -> Signal{
        let parts: Vec<isize> = input.chars().map(|x| x.to_digit(10).unwrap() as isize).collect();

        Signal{
            parts: parts
        }
    }

    fn fft(&mut self, base_pattern: &Vec<isize>) {
        let new_parts: Vec<isize> = (0..self.parts.len()).map(|idx| {
            let result: isize = self.parts.iter().enumerate().map(|(jdx, x)| {
                let index = ((jdx + 1) / (idx + 1)) % base_pattern.len();
                (x * base_pattern[index]) % 10
            }).sum();
            result.abs() % 10
        }).collect();

        self.parts.clear();
        self.parts.extend(new_parts.iter());
    }
}

impl Display for Signal{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.parts.iter().map(|x| char::from_digit(*x as u32, 10).unwrap()).collect();
        write!(f, "{}", s)
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = Signal::from_string(&contents);
    debug!("{:?}", input);

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(input: &Signal) {
    let base_pattern = vec![0, 1, 0, -1];
    let mut signal = input.clone();
    for _ in 0..100 {
        signal.fft(&base_pattern);
    }
    println!("{}", signal);
}

fn part2(input: &Signal) {
    let len = input.parts.len();
    let offset: usize = input.parts
        .iter().take(7).fold(0usize, |acc, x|{
            acc * 10 + *x as usize
        });
    println!("Offset {}", offset);
    println!("Input sequence length {}", len);
    println!("Large sequence length {}", len * 10000);

    let mut parts: Vec<isize> = input.parts
        .iter().cloned()
        .cycle()
        .take(len * 10000)
        .skip(offset)
        .collect();
    
    for _ in 0..100 {
        let mut total: isize = parts.iter().sum();

        parts.iter_mut().for_each(|x| {
            let previous = total;
            total -= *x;
            *x = previous % 10;
        });
    }

    let answer: String = parts.iter()
        .take(8)
        .map(|x| char::from_digit(*x as u32, 10).unwrap())
        .collect();
    
    println!("{}", answer);
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger;

    #[test]
    fn test() {
        let mut signal = Signal::from_string("12345678");
        println!("{}", signal);
        let base_pattern = vec![0, 1, 0, -1];
        signal.fft(&base_pattern);
        println!("{}", signal);
        signal.fft(&base_pattern);
        println!("{}", signal);
    }

    #[test]
    fn test2() {
        let input = Signal::from_string("80871224585914546619083218645595");
        let base_pattern = vec![0, 1, 0, -1];
        let mut signal = input.clone();
        for _ in 0..100 {
            signal.fft(&base_pattern);
        }
        println!("{}", signal);
    }

    #[test]
    fn test3() {
        let input = Signal::from_string("03036732577212944063491565474664");
        part2(&input);
    }
}