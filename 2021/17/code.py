# Advent of code Year 2021 Day 17 solution
# Author = witzatom
# Date = December 2021
# from __future__ import annotations
import re
from typing import NamedTuple
import math
from dataclasses import dataclass
from utils import timed


@dataclass
class Point2:
    x: int
    y: int

    def add(self, other):
        self.x += other.x
        self.y += other.y


class TargetArea(NamedTuple):
    x_min: int
    x_max: int
    y_min: int
    y_max: int

    def isin(self, other) -> bool:
        if other.x < self.x_min or self.x_max < other.x:
            return False
        if other.y < self.y_min or self.y_max < other.y:
            return False
        return True


@timed
def run(puzzle_input):
    target_area = parse(puzzle_input)
    velocities = find_velocities(target_area)
    print(f"Part One : {max(velocities.values())}")
    print(f"Part Two : {len(velocities.keys())}")


def parse(puzzle_input):
    pattern = r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)"
    target = tuple(map(int, re.match(pattern, puzzle_input).groups()))
    return TargetArea(*target)


def launch(velocity: Point2, target_area: TargetArea):
    position = Point2(0, 0)
    top_y = position.y
    while position.x <= target_area.x_max and position.y >= target_area.y_min:
        position.add(velocity)
        top_y = max(top_y, position.y)

        if target_area.isin(position):
            return top_y

        velocity.y -= 1  # gravity
        velocity.x = max(0, velocity.x - 1)  # drag
    return -1


def find_velocities(target_area: TargetArea):
    min_x_v = round(math.sqrt(target_area.x_min * 2))
    return dict(
        filter(
            lambda x: x[1] >= 0,
            (
                ((x, y), launch(Point2(x, y), target_area))
                for x in range(min_x_v, target_area.x_max + 1)
                for y in range(target_area.y_min, -target_area.y_min)
            ),
        )
    )
