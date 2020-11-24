use log::{debug, info};
use std::str::FromStr;
use std::fs;

enum Shuffle{
    Deal,
    Cut(i128),
    DealIncrement(i128)
}

impl FromStr for Shuffle{
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Shuffle, Self::Err> {
        if input.contains("cut ") {
            let number: String = input.chars().skip(4).collect();
            let count = number.parse::<i128>().unwrap();
            Ok(Shuffle::Cut(count))
        } else if input.contains("deal with increment ") {
            let number: String = input.chars().skip(20).collect();
            let count = number.parse::<i128>().unwrap();
            Ok(Shuffle::DealIncrement(count))
        } else if input.contains("deal into new stack") {
            Ok(Shuffle::Deal)
        } else {
            Err("Shuffle not known")
        }
    }
}

pub fn solve(input_file: &str){
    use std::time::Instant;

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input: Vec<Shuffle> = contents.lines().map(|x| Shuffle::from_str(x).unwrap()).collect();

    let part1_time = Instant::now();
    part1(&input);
    println!("Part 1 took {} millis", part1_time.elapsed().as_millis());
    let part2_time = Instant::now();
    part2(&input);
    println!("Part 2 took {} millis", part2_time.elapsed().as_millis());
}

fn part1(commands: &Vec<Shuffle>) {
    let n = 10007;
    let result: i128 = commands.iter().fold(2019, |position, command| {
        let tick = match command {
            Shuffle::Deal => n - 1 - position,
            Shuffle::Cut(count) => (position - count) % n,
            Shuffle::DealIncrement(count) => (position * count) % n
        };
        //debug!("{:?}", tick);
        tick
    });
    println!("{}", result);
}

fn part2(commands: &Vec<Shuffle>) {
    let m = 119315717514047i128;
    let combination = Combine::new(&commands, m);

    let x = 2020;
    let n = 101741582076661i128;
    let result = combination.apply_times(x, n, m);
    println!("{}", result);
}

struct Combine(i128, i128);

impl Combine{
    fn new(commands: &Vec<Shuffle>, modulo: i128) -> Combine {
        commands.iter().rev().fold(Combine(1, 0), |Combine(a, b), command| {
            let next = match command {
                Shuffle::Cut(count) => (a, b + count),
                Shuffle::DealIncrement(count) => {
                    let inv = modular_pow(*count, modulo - 2, modulo);
                    (a * inv, b * inv)
                }
                Shuffle::Deal => (-a, - b - 1),
            };
            //debug!("{:?} -> {}", tick, ((x * tick.0 + tick.1) % n + n) % n);
            Combine(next.0 % modulo, next.1 % modulo)
        })
    }

    fn apply_times(&self, x: i128, times: i128, modulo: i128) -> i128 {
        let i1 = (modular_pow(self.0, times, modulo) * x) % modulo;
        let i2 = (modular_pow(self.0, times, modulo) - 1) % modulo;
        let i3 = self.1 * i2 % modulo;
        let i4 = modular_pow(self.0 - 1, modulo - 2, modulo);
        ((i1 + i3 * i4) % modulo + modulo) % modulo
    }
}

/*
function modular_pow(base, exponent, modulus) is
    if modulus = 1 then
        return 0
    Assert :: (modulus - 1) * (modulus - 1) does not overflow base
    result := 1
    base := base mod modulus
    while exponent > 0 do
        if (exponent mod 2 == 1) then
            result := (result * base) mod modulus
        exponent := exponent >> 1
        base := (base * base) mod modulus
    return result

*/
fn modular_pow(base: i128, exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0i128;
    }
    assert_eq!((modulus - 1).checked_mul(modulus - 1).is_some(), true);
    //assert that (modulus - 1) * (modulus - 1) does not overflow
    let mut result: i128 = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;
    use env_logger::*;

    #[test]
    fn test_mod_exponent() {
        assert_eq!(modular_pow(4, 13, 497), 445);
    }

    #[test]
    fn test_simple() {
        let contents = r#"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
        "#.trim();
        let commands: Vec<Shuffle> = contents.lines().map(|x| Shuffle::from_str(x).unwrap()).collect();
        let n = 10;
    }
}
