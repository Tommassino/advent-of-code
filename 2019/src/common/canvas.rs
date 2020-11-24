use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::convert::Infallible;


#[derive(Debug)]
pub struct Canvas {
    paint: HashMap<(isize, isize), char>
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas{
            paint: HashMap::<(isize, isize), char>::new()
        }
    }

    pub fn paint(&mut self, x: isize, y: isize, color: char) {
        self.paint.insert((x, y), color);
    }

    pub fn color_at(&self, x: isize, y: isize) -> char {
        *self.paint.get(&(x, y)).unwrap_or(&' ')
    }
}

impl FromStr for Canvas {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut canvas = Canvas{paint: HashMap::new()};
        let mut x = 0;
        let mut y = 0;
        input.chars().for_each(|c|{
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                canvas.paint(x, y, c);
                x += 1;
            }
        });
        Ok(canvas)
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = *self.paint.keys().map(|(x, _)| x).min().unwrap();
        let max_x = *self.paint.keys().map(|(x, _)| x).max().unwrap();
        let min_y = *self.paint.keys().map(|(_, y)| y).min().unwrap();
        let max_y = *self.paint.keys().map(|(_, y)| y).max().unwrap();

        let repr: String = (min_y..=max_y).map(|y| {
            let line: String = (min_x..=max_x).map(|x| {
                self.color_at(x, y)
            }).collect();
            format!("{}\n", line)
        }).collect();
        write!(f, "{}", repr)
    }
}