use log::{debug, info};
use std::fs;
use std::collections::HashSet;
use std::ops::AddAssign;
use num::integer::lcm;
use itertools::izip;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct AxisSimulation{
    positions: Vec<isize>,
    velocities: Vec<isize>
}

impl AxisSimulation{
    fn new() -> AxisSimulation {
        AxisSimulation{
            positions: Vec::new(),
            velocities: Vec::new()
        }
    }
    
    fn tick(&mut self) {
        let gravities: Vec<isize> = self.positions
            .iter()
            .map(|moon| {
                self.positions
                    .iter()
                    .fold(0, |accum, other| {
                        accum + (other - moon).signum()
                    })
            })
            .collect();
        
        self.velocities
            .iter_mut()
            .zip(gravities)
            .for_each(|(velocity, gravity)| {
                *velocity += gravity;
            });

        self.positions
            .iter_mut()
            .zip(self.velocities.iter())
            .for_each(|(position, velocity)| {
                *position += velocity;
            });
    }

    fn cycle_size(&self) -> usize {
        let mut state = self.to_owned();
        let mut visited: HashSet<AxisSimulation> = HashSet::new();
        visited.insert(state.clone());
        loop {
            state.tick();
            if visited.replace(state.clone()).is_some() {
                break
            }
        }
        visited.len()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Simulation{
    x_axis: AxisSimulation,
    y_axis: AxisSimulation,
    z_axis: AxisSimulation
}

impl Simulation {
    fn from_string(data: &str) -> Simulation {
        let mut x_axis = AxisSimulation::new();
        let mut y_axis = AxisSimulation::new();
        let mut z_axis = AxisSimulation::new();

        data
            .lines()
            .for_each(|line| {
                let data = &line[1..line.len()-1];
                let values: Vec<isize> = data.split(",").map(|part| {
                    part.split("=").last().unwrap().parse::<isize>().unwrap()
                }).collect();
                x_axis.positions.push(values[0]);
                x_axis.velocities.push(0);
                y_axis.positions.push(values[1]);
                y_axis.velocities.push(0);
                z_axis.positions.push(values[2]);
                z_axis.velocities.push(0);
            });

        Simulation{
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: z_axis
        }
    }

    fn tick(&mut self) {
        self.x_axis.tick();
        self.y_axis.tick();
        self.z_axis.tick();
    }
}

pub fn solve(input_file: &str){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = Simulation::from_string(&contents);

    part1(&input);
    part2(&input);
}

fn part1(input: &Simulation) {
    let mut state = input.to_owned();
    (1..=1000).for_each(|_| state.tick());

    let total_energy: isize = izip!(
        state.x_axis.positions,
        state.x_axis.velocities,
        state.y_axis.positions,
        state.y_axis.velocities,
        state.z_axis.positions,
        state.z_axis.velocities
    ).map(|(px, vx, py, vy, pz, vz)| {
        let potential = px.abs() + py.abs() + pz.abs();
        let kinetic = vx.abs() + vy.abs() + vz.abs();
        potential * kinetic
    }).sum();
    
    println!("Total system energy {}", total_energy);
}

fn part2(state: &Simulation) {
    let x_cycle = state.x_axis.cycle_size();
    let y_cycle = state.y_axis.cycle_size();
    let z_cycle = state.z_axis.cycle_size();
    
    let cycle_size = lcm(x_cycle, lcm(y_cycle, z_cycle));
    println!("Found cycle of size {} steps", cycle_size);
}
