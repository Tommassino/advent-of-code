#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<usize>>,
}

impl Grid {
    pub fn full(value: usize, width: usize, height: usize) -> Grid {
        let data: Vec<Vec<usize>> = (0..height).map(|_| {
            (0..width).map(|_| value).collect()
        }).collect();
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn iter(&self, x: isize, y: isize, direction: [isize; 2]) -> GridIterator {
        GridIterator {
            x,
            y,
            width: self.width as isize,
            height: self.height as isize,
            direction,
            grid: self,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct GridIterator<'a> {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    direction: [isize; 2],
    grid: &'a Grid,
}

impl<'a> GridIterator<'a> {
    fn value(&self) -> usize {
        self.grid.data[self.y as usize][self.x as usize]
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = GridIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.direction[0];
        self.y += self.direction[1];
        if self.x < 0 || self.y < 0 || self.x >= self.width || self.y >= self.height {
            None
        } else {
            Some(*self)
        }
    }
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let data: Vec<Vec<usize>> = input.lines().map(|line| {
            line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect()
        }).collect();
        let height = data.len();
        let width = data[0].len();
        Grid {
            width,
            height,
            data,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let mut visibility: Grid = Grid::full(0, grid.width, grid.height);

    for row_col in [true, false] {
        for normal_reverse in [1, -1] {
            let idx_max = if row_col { grid.width } else { grid.height };
            for idx in 0..idx_max {
                let idx_start: isize =
                    if normal_reverse > 0 { -1 } else if row_col { grid.height as isize } else { grid.width as isize };

                let x = if row_col { idx as isize } else { idx_start };
                let y = if row_col { idx_start } else { idx as isize };
                let direction = [
                    if row_col { 0 } else { normal_reverse },
                    if row_col { normal_reverse } else { 0 },
                ];
                let mut cummax: i8 = -1;
                for item in grid.iter(x, y, direction) {
                    if item.value() as i8 > cummax {
                        cummax = item.value() as i8;
                        visibility.data[item.y as usize][item.x as usize] = 1;
                    }
                }
            }
        }
    }

    let visible_count: u32 = visibility.data.iter()
        .map(|x| x.iter().map(|x| *x as u32).sum::<u32>())
        .sum();
    // println!("{:?}", visibility);
    Some(visible_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);
    let mut visibility: Grid = Grid::full(1, grid.width, grid.height);

    for row_col in [true, false] {
        for normal_reverse in [1, -1] {
            let idx_max = if row_col { grid.width } else { grid.height };
            for idx in 0..idx_max {
                let idx_start: isize =
                    if normal_reverse > 0 { -1 } else if row_col { grid.height as isize } else { grid.width as isize };

                let x = if row_col { idx as isize } else { idx_start };
                let y = if row_col { idx_start } else { idx as isize };
                let direction = [
                    if row_col { 0 } else { normal_reverse },
                    if row_col { normal_reverse } else { 0 },
                ];
                let mut stack: Vec<usize> = Vec::new();
                for item in grid.iter(x, y, direction) {
                    let outlook = stack.iter().rev().enumerate()
                        .find(|(_, x)| **x >= item.value())
                        .map(|(x, _)| x + 1).unwrap_or(stack.len());
                    //TODO this could clean up the stack so that it does not grow so much
                    // or maybe have like a hashmap from tree height to visibility so that
                    // its static sized
                    stack.push(item.value());
                    visibility.data[item.y as usize][item.x as usize] *= outlook;
                }
            }
        }
    }
    // println!("{:?}", visibility);
    let best_visibility = visibility.data.iter()
        .flat_map(|x| x.iter().max()).max().unwrap();
    Some(*best_visibility)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
