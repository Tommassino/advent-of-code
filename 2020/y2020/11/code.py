# Advent of code Year 2020 Day 11 solution
# Author = witzatom
# Date = December 2020

from itertools import count

import numpy as np

from y2021.utils import timed


def run(puzzle_input):
    grid = Grid(puzzle_input, True)
    print(f"Part One : {part1(grid)}")
    grid = Grid(puzzle_input, False)
    print(f"Part Two : {part2(grid)}")


class Grid:
    SPACE = 0
    SEAT = 1
    OCCUPIED = 2

    DIRECTIONS = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ]

    def __init__(self, puzzle_input, padding):
        self.data = np.array([
            list(line.replace(".", str(self.SPACE)).replace("L", str(self.SEAT)).replace("#", str(self.OCCUPIED)))
            for line in puzzle_input.split("\n")
        ], dtype="int")

        if padding:
            padded = np.full((self.data.shape[0] + 2, self.data.shape[1] + 2), 0)
            padded[1:-1, 1:-1] = self.data
            self.data = padded

    def __repr__(self):
        return "\n".join([
            "".join([
                '.' if c == self.SPACE else "#" if c == self.OCCUPIED else "L"
                for c in line
            ])
            for line in self.data
        ])

    def windows(self, window_size):
        for i in range(0, self.data.shape[0] - window_size + 1):
            for j in range(0, self.data.shape[1] - window_size + 1):
                yield self.data[i: i + window_size, j: j + window_size].reshape((-1, window_size))

    def raytrace(self, x, y, velocity_x, velocity_y):
        while True:
            x += velocity_x
            y += velocity_y
            if x < 0 or y < 0:
                return
            if x >= self.data.shape[1] or y >= self.data.shape[0]:
                return
            yield self.data[y, x]

    def neighbors(self, x, y):
        n = 0
        for velocity_x, velocity_y in self.DIRECTIONS:
            maybe_first_seat = next(
                (i for i in self.raytrace(x, y, velocity_x, velocity_y) if i > 0),
                None
            )
            if maybe_first_seat == 2:
                n += 1
        return n


@timed
def part1(grid: Grid):
    for i in count(0):
        changes, grid = tick_part1(grid)
        # print(i, changes)
        if changes == 0:
            seat_count = np.sum(grid.data == 2)
            return seat_count


def tick_part1(grid: Grid):
    result = []
    changed = 0
    for window_data in grid.windows(3):
        neighbors = np.sum(window_data == 2)
        spot = window_data[1, 1]
        if neighbors == 0 and spot == 1:
            new_value = 2
            changed += 1
        elif neighbors >= 5 and spot == 2:
            new_value = 1
            changed += 1
        else:
            new_value = spot
        result.append(new_value)
    result = np.array(result).reshape((grid.data.shape[0] - 2, grid.data.shape[1] - 2))
    grid.data[1:-1, 1:-1] = result
    return changed, grid


@timed
def part2(grid: Grid):
    for i in count(0):
        changes, grid = tick_part2(grid)
        # print(i, changes)
        if changes == 0:
            seat_count = np.sum(grid.data == 2)
            return seat_count


def tick_part2(grid: Grid):
    result = []
    changed = 0

    for y in range(grid.data.shape[0]):
        for x in range(grid.data.shape[1]):
            neighbor_count = grid.neighbors(x, y)
            spot = grid.data[y, x]
            if neighbor_count == 0 and spot == 1:
                new_value = 2
                changed += 1
            elif neighbor_count >= 5 and spot == 2:
                new_value = 1
                changed += 1
            else:
                new_value = spot
            result.append(new_value)
    grid.data = np.array(result).reshape((grid.data.shape[0], grid.data.shape[1]))
    return changed, grid
