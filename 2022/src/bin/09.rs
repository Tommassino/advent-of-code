use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;

use image::{Frame, ImageResult, Rgba, RgbaImage};
use image::gif::{GifEncoder};
use imageproc::drawing::{
    draw_filled_circle_mut, draw_filled_rect_mut, draw_line_segment_mut,
};
use imageproc::rect::Rect;
use itertools::{Itertools};

use advent_of_code::helpers::Point2;

pub fn get_direction(dir: &str) -> Point2<i32> {
    match dir {
        "R" => Point2 { x: 1, y: 0 },
        "L" => Point2 { x: -1, y: 0 },
        "U" => Point2 { x: 0, y: -1 },
        "D" => Point2 { x: 0, y: 1 },
        _ => panic!()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tail_positions: HashSet<Point2<i32>> = HashSet::new();
    let mut head = Point2 { x: 0, y: 0 };
    let mut tail = Point2 { x: 0, y: 0 };
    input.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split(' ').next_tuple().unwrap();
        let amount = amount_str.parse::<u32>().unwrap();
        let direction = get_direction(direction_str);
        for _ in 0..amount {
            head += direction;
            let vector = head - tail;
            if max(vector.x.abs(), vector.y.abs()) > 1 {
                let adjustment = Point2 {
                    x: if vector.x != 0 { vector.x / vector.x.abs() } else { 0 },
                    y: if vector.y != 0 { vector.y / vector.y.abs() } else { 0 },
                };
                // println!("Moving tail from {:?} towards head at {:?} by {:?}", tail, head, adjustment);
                tail += adjustment;
            }
            tail_positions.insert(tail);
        }
    });
    Some(tail_positions.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tail_positions: HashSet<Point2<i32>> = HashSet::new();
    let mut rope_knots: Vec<Point2<i32>> = (0..10).map(|_| Point2 { x: 0, y: 0 }).collect();
    input.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split(' ').next_tuple().unwrap();
        let amount = amount_str.parse::<u32>().unwrap();
        let direction = get_direction(direction_str);
        for _ in 0..amount {
            // move the head
            rope_knots[0] += direction;
            // move each knot
            for idx in 1..rope_knots.len() {
                let vector = rope_knots[idx - 1] - rope_knots[idx];
                if max(vector.x.abs(), vector.y.abs()) > 1 {
                    let adjustment = Point2 {
                        x: if vector.x != 0 { vector.x / vector.x.abs() } else { 0 },
                        y: if vector.y != 0 { vector.y / vector.y.abs() } else { 0 },
                    };
                    rope_knots[idx] += adjustment;
                }
            }
            tail_positions.insert(*rope_knots.last().unwrap());
        }
    });
    Some(tail_positions.len())
}

#[derive(Debug)]
struct Day9Animation {
    rope_positions: Vec<Vec<Point2<i32>>>,
    block_size: u32,
    knot_size: (u32, u32),
    image_size: (u32, u32),
    margin: (u32, u32),
}

impl Day9Animation {
    pub fn parse(input: &str) -> Day9Animation {
        let mut rope_positions: Vec<Vec<Point2<i32>>> = Vec::new();
        let mut rope_knots: Vec<Point2<i32>> = (0..10).map(|_| Point2 { x: 0, y: 0 }).collect();
        input.lines().for_each(|line| {
            let (direction_str, amount_str) = line.split(' ').next_tuple().unwrap();
            let amount = amount_str.parse::<u32>().unwrap();
            let direction = get_direction(direction_str);
            for _ in 0..amount {
                // move the head
                rope_knots[0] += direction;
                // move each knot
                for idx in 1..rope_knots.len() {
                    let vector = rope_knots[idx - 1] - rope_knots[idx];
                    if max(vector.x.abs(), vector.y.abs()) > 1 {
                        let adjustment = Point2 {
                            x: if vector.x != 0 { vector.x / vector.x.abs() } else { 0 },
                            y: if vector.y != 0 { vector.y / vector.y.abs() } else { 0 },
                        };
                        rope_knots[idx] += adjustment;
                    }
                }
                rope_positions.push(rope_knots.clone());
            }
        });

        // the animation places the knot head at the images midpoint
        let (width, height) = rope_positions.iter()
            .map(
                |knots| {
                    let head = knots.first().unwrap();
                    let offsets: Vec<Point2<i32>> = knots.iter().skip(1).map(|p| *p - *head).collect();
                    let (min_x, max_x) = offsets.iter().map(|p| p.x)
                        .minmax().into_option().unwrap();
                    let (min_y, max_y) = offsets.iter().map(|p| p.y)
                        .minmax().into_option().unwrap();
                    (
                        (max_x - min_x) as u32 * 2,
                        (max_y - min_y) as u32 * 2
                    )
                }
            ).max().unwrap();
        let block_size = 10u32;
        let margin = (4u32, 4u32);
        let image_size = (
            (width + margin.0 * 2) * block_size,
            (height + margin.1 * 2) * block_size
        );

        Day9Animation {
            rope_positions,
            block_size,
            knot_size: (width, height),
            image_size,
            margin,
        }
    }

    pub fn map_point(&self, point: Point2<i32>, head: Point2<i32>) -> (f32, f32) {
        let translated = point - head;
        (
            (translated.x * self.block_size as i32 + self.image_size.0 as i32 / 2) as f32,
            (translated.y * self.block_size as i32 + self.image_size.1 as i32 / 2) as f32
        )
    }

    pub fn frame(&self, frame_idx: usize) -> Frame {
        let red = Rgba([255u8, 0u8, 0u8, 255u8]);
        let green = Rgba([0u8, 255u8, 0u8, 255u8]);
        let grey = Rgba([100u8, 100u8, 100u8, 255u8]);
        let black = Rgba([0u8, 0u8, 0u8, 255u8]);

        let mut image = RgbaImage::new(
            self.image_size.0,
            self.image_size.1,
        );
        // black background
        draw_filled_rect_mut(
            &mut image,
            Rect::at(0,0).of_size(self.image_size.0, self.image_size.1),
            black
        );
        let frame_positions = &self.rope_positions[frame_idx];
        let head = frame_positions[0];
        // draw left-right grid
        for y in -(self.margin.1 as i32)..=(self.knot_size.1 + self.margin.1) as i32 {
            if (y + (head.y % 5).abs()) % 5 != 0 {
                continue
            }
            let left_point = Point2 {
                x: head.x - self.knot_size.0 as i32 / 2 - self.margin.0 as i32,
                y: head.y - self.knot_size.1 as i32 / 2 + y,
            };
            let right_point = Point2 {
                x: head.x + self.knot_size.0 as i32 / 2 + self.margin.0 as i32,
                y: head.y - self.knot_size.1 as i32 / 2 + y,
            };
            draw_line_segment_mut(
                &mut image,
                self.map_point(left_point, head),
                self.map_point(right_point, head),
                grey,
            )
        }
        // draw top-down grid
        for x in -(self.margin.0 as i32)..=(self.knot_size.0 + self.margin.0) as i32 {
            if (x + (head.x % 5).abs()) % 5 != 0 {
                continue
            }
            let top_point = Point2 {
                x: head.x - self.knot_size.0 as i32 / 2 + x,
                y: head.y - self.knot_size.1 as i32 / 2 - self.margin.1 as i32,
            };
            let bottom_point = Point2 {
                x: head.x - self.knot_size.0 as i32 / 2 + x,
                y: head.y + self.knot_size.1 as i32 / 2 + self.margin.1 as i32,
            };
            draw_line_segment_mut(
                &mut image,
                self.map_point(top_point, head),
                self.map_point(bottom_point, head),
                grey,
            )
        }
        // draw lines between knots
        self.rope_positions[frame_idx].windows(2).for_each(|points| {
            draw_line_segment_mut(
                &mut image,
                self.map_point(points[0], head),
                self.map_point(points[1], head),
                green,
            );
        });
        // draw knots as circles
        self.rope_positions[frame_idx].iter().for_each(|point| {
            let center = self.map_point(*point, head);
            draw_filled_circle_mut(
                &mut image,
                (center.0 as i32, center.1 as i32),
                self.block_size as i32 / 4,
                red
            );
        });
        Frame::new(image)
    }

    pub fn to_gif(&self, out_file: &str, frames_per_iter: usize, max_iter: usize) -> ImageResult<()>{
        let file_out = File::create(out_file)?;
        let mut encoder = GifEncoder::new(file_out);
        for frame_idx in 0..min(max_iter, self.rope_positions.len()) {
            println!("Frame {}/{}", frame_idx, self.rope_positions.len());
            let frame = self.frame(frame_idx);
            for _ in 0..frames_per_iter {
                encoder.encode_frame(frame.clone())?;
            }
        }
        Ok(())
    }
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 9, None);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    let animation = Day9Animation::parse(input);
    animation.to_gif("out.gif", 3, 500).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9, None);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9, None);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_larger() {
        let input = advent_of_code::read_file("examples", 9, Some("larger"));
        assert_eq!(part_two(&input), Some(36));
    }
}
