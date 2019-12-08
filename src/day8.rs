use log::{debug, info};
use std::fs;
use std::{str, char};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Vec<u32>>
}

impl Image {
    fn pixel(&self, x: usize, y: usize) -> u32 {
        self.layers
            .iter()
            .map(|layer| {
                layer[y * self.width + x]
            })
            .find(|&color| color != 2)
            .unwrap_or(2)
    }

    fn from_string(contents: &str, width: usize, height: usize) -> Image {
        let pixels_per_layer = width * height;

        let layers = contents
            .as_bytes()
            .chunks(pixels_per_layer)
            .map(|chunk| {
                chunk.iter()
                    .map(|x| (*x as char).to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        Image{
            width: width,
            height: height,
            layers: layers
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: String = (0..self.height).map(|y| {
            let line: String = (0..self.width).map(|x| {
                //char::from_digit(self.pixel(x, y), 10).unwrap()
                let value = self.pixel(x, y);
                if value == 1 {
                    '█'
                } else {
                    ' '
                }
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}

pub fn solve(input_file: &str){
    let contents: String = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let input = Image::from_string(&contents, 25, 6);

    part1(&input);
    part2(&input);
}

fn part1(input: &Image) {
    let best_layer = input.layers.iter().min_by_key(|layer| {
        layer.iter().filter(|&x| *x == 0).count()
    }).unwrap();
    let one_digits = best_layer.iter().filter(|&x| *x == 1).count() as usize;
    let two_digits = best_layer.iter().filter(|&x| *x == 2).count() as usize;
    debug!("{:?}", best_layer);
    info!("1 * 2 : {}", one_digits * two_digits)
}

fn part2(input: &Image) {
    info!("\n{}", input);
}
