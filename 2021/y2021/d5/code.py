# Advent of code Year 2021 Day 5 solution
# Author = witzatom
# Date = December 2021
import re
import numpy as np
from typing import NamedTuple


class Vent(NamedTuple):
    x_1: int
    y_1: int
    x_2: int
    y_2: int

    def is_horizontal(self):
        return self.y_1 == self.y_2

    def is_vertical(self):
        return self.x_1 == self.x_2

    def line_coordinates(self):
        vector = np.sign([self.x_2 - self.x_1, self.y_2 - self.y_1])
        x_coordinates = [self.x_1]
        y_coordinates = [self.y_1]
        while x_coordinates[-1] != self.x_2 or y_coordinates[-1] != self.y_2:
            x_coordinates.append(x_coordinates[-1] + vector[0])
            y_coordinates.append(y_coordinates[-1] + vector[1])
        return x_coordinates, y_coordinates


def run(puzzle_input):
    vents = parse_input(puzzle_input)
    dangerous_spots = part_one(vents)
    very_dangerous_spots = part_two(vents)

    print(f"Part One : {dangerous_spots}")
    print(f"Part Two : {very_dangerous_spots}")


def part_two(vents):
    max_x = max(max(vent.x_1, vent.x_2) for vent in vents) + 1
    max_y = max(max(vent.y_1, vent.y_2) for vent in vents) + 1
    grid = np.zeros((max_x, max_y), int)
    for vent in vents:
        # print(vent, vent.line_coordinates())
        grid[vent.line_coordinates()] += 1
    # print(grid.T)
    return np.sum(grid > 1)


def part_one(vents):
    max_x = max(max(vent.x_1, vent.x_2) for vent in vents) + 1
    max_y = max(max(vent.y_1, vent.y_2) for vent in vents) + 1
    grid = np.zeros((max_x, max_y), int)

    for vent in vents:
        if vent.is_horizontal():
            start_x = min(vent.x_1, vent.x_2)
            end_x = max(vent.x_1, vent.x_2) + 1
            grid[start_x:end_x, vent.y_2] += 1
        if vent.is_vertical():
            start_y = min(vent.y_1, vent.y_2)
            end_y = max(vent.y_1, vent.y_2) + 1
            grid[vent.x_1, start_y:end_y] += 1
    return np.sum(grid > 1)


def parse_input(puzzle_input):
    pattern = re.compile(r"(\d+),(\d+) -> (\d+),(\d+)")

    def parse_line(line):
        x_1, y_1, x_2, y_2 = pattern.match(line).groups()
        return Vent(int(x_1), int(y_1), int(x_2), int(y_2))

    vents = [parse_line(line) for line in puzzle_input.split("\n")]
    return vents
