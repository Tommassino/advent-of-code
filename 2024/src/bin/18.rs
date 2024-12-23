use advent_of_code::helpers::Point2;
use std::collections::HashSet;
advent_of_code::solution!(18);
/*
\--- Day 18: RAM Run ---
----------

You and The Historians look a lot more pixelated than you remember. You're [inside a computer](/2017/day/2) at the North Pole!

Just as you're about to check out your surroundings, a program runs up to you. "This region of memory isn't safe! The User misunderstood what a [pushdown automaton](https://en.wikipedia.org/wiki/Pushdown_automaton) is and their algorithm is pushing whole *bytes* down on top of us! Run!"

The algorithm is fast - it's going to cause a byte to fall into your memory space once every [nanosecond](https://www.youtube.com/watch?v=9eyFDBPk4Yw)! Fortunately, you're *faster*, and by quickly scanning the algorithm, you create a *list of which bytes will fall* (your puzzle input) in the order they'll land in your memory space.

Your memory space is a two-dimensional grid with coordinates that range from `0` to `70` both horizontally and vertically. However, for the sake of example, suppose you're on a smaller grid with coordinates that range from `0` to `6` and the following list of incoming byte positions:

```
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0

```

Each byte position is given as an `X,Y` coordinate, where `X` is the distance from the left edge of your memory space and `Y` is the distance from the top edge of your memory space.

You and The Historians are currently in the top left corner of the memory space (at `0,0`) and need to reach the exit in the bottom right corner (at `70,70` in your memory space, but at `6,6` in this example). You'll need to simulate the falling bytes to plan out where it will be safe to run; for now, simulate just the first few bytes falling into your memory space.

As bytes fall into your memory space, they make that coordinate *corrupted*. Corrupted memory coordinates cannot be entered by you or The Historians, so you'll need to plan your route carefully. You also cannot leave the boundaries of the memory space; your only hope is to reach the exit.

In the above example, if you were to draw the memory space after the first `12` bytes have fallen (using `.` for safe and `#` for corrupted), it would look like this:

```
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....

```

You can take steps up, down, left, or right. After just 12 bytes have corrupted locations in your memory space, the shortest path from the top left corner to the exit would take `*22*` steps. Here (marked with `O`) is one such path:

```
OO.#OOO
.O#OO#O
.OOO#OO
...#OO#
..#OO#.
.#.O#..
#.#OOOO

```

Simulate the first kilobyte (`1024` bytes) falling onto your memory space. Afterward, *what is the minimum number of steps needed to reach the exit?*

Your puzzle answer was `286`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

The Historians aren't as used to moving around in this pixelated universe as you are. You're afraid they're not going to be fast enough to make it to the exit before the path is completely blocked.

To determine how fast everyone needs to go, you need to determine *the first byte that will cut off the path to the exit*.

In the above example, after the byte at `1,1` falls, there is still a path to the exit:

```
O..#OOO
O##OO#O
O#OO#OO
OOO#OO#
###OO##
.##O###
#.#OOOO

```

However, after adding the very next byte (at `6,1`), there is no longer a path to the exit:

```
...#...
.##..##
.#..#..
...#..#
###..##
.##.###
#.#....

```

So, in this example, the coordinates of the first byte that prevents the exit from being reachable are `*6,1*`.

Simulate more of the bytes that are about to corrupt your memory space. *What are the coordinates of the first byte that will prevent the exit from being reachable from your starting position?* (Provide the answer as two integers separated by a comma with no other characters.)

 */

#[derive(Debug)]
struct Input {
    bytes: Vec<Point2<i32>>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let bytes = value
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                Point2::new(x, y)
            })
            .collect();
        Input { bytes }
    }
}

impl Input {
    fn path_find(&self, at_time: usize, start: Point2<i32>, exit: Point2<i32>) -> Option<usize> {
        // implement a floodfill
        let mut visited = HashSet::new();
        let mut frontier = HashSet::new();
        frontier.insert(start);
        visited.insert(start);
        let corrupted: HashSet<Point2<i32>> = self.bytes.iter().take(at_time).copied().collect();
        let mut steps = 0;
        while !frontier.is_empty() {
            let mut next = HashSet::new();
            for position in frontier {
                if position == exit {
                    return Some(steps);
                }
                for neighbor in position.neighbors_checked(exit.x + 1, exit.y + 1) {
                    if !corrupted.contains(&neighbor) && !visited.contains(&neighbor) {
                        next.insert(neighbor);
                        visited.insert(position);
                    }
                }
            }
            steps += 1;
            frontier = next;
        }
        None
    }

    fn unreachable_byte(&self, start: Point2<i32>, exit: Point2<i32>) -> Option<Point2<i32>> {
        // binary search
        let mut low = 0;
        let mut high = self.bytes.len();
        while low < high {
            let mid = (low + high) / 2;
            if self.path_find(mid, start, exit).is_some() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        self.bytes.get(low - 1).copied()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from(input);
    let start = Point2::new(0, 0);
    let exit = Point2::new(70, 70);
    input.path_find(1024, start, exit)
}

pub fn part_two(input: &str) -> Option<Point2<i32>> {
    let input = Input::from(input);
    let start = Point2::new(0, 0);
    let exit = Point2::new(70, 70);
    input.unreachable_byte(start, exit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_str: &str = &advent_of_code::template::read_file("examples", DAY);
        let input = Input::from(input_str);
        let start = Point2::new(0, 0);
        let exit = Point2::new(6, 6);
        let result = input.path_find(12, start, exit);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
