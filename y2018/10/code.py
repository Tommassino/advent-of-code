# Advent of code Year 2018 Day 10 solution
# Author = witzatom
# Date = December 2018

from dataclasses import dataclass
import numpy as np
from typing import List
import re
from itertools import count, groupby

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


@dataclass(frozen=False)
class Star:
    position: np.array
    velocity: np.array


def from_string(data) -> List[Star]:
    star_re = re.compile(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
    stars = []
    for line in data.split("\n"):
        star_match = star_re.match(line)
        position = np.array(star_match.groups()[:2]).astype(int)
        velocity = np.array(star_match.groups()[2:]).astype(int)
        stars.append(Star(position, velocity))
    return stars


def star_repr(stars):
    min_x = min(star.position[0] for star in stars)
    max_x = max(star.position[0] for star in stars)
    min_y = min(star.position[1] for star in stars)
    max_y = max(star.position[1] for star in stars)
    positions = set(
        tuple(star.position)
        for star in stars
    )
    repr = []
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if (x, y) in positions:
                repr.append("*")
            else:
                repr.append(" ")
        repr.append("\n")
    return "".join(repr)


def detect_word(stars):
    x_positions = list(
        star.position[0]
        for star in stars
    )
    min_y = min(star.position[1] for star in stars)
    max_y = max(star.position[1] for star in stars)
    height = max_y - min_y + 1
    vertical_counts = {
        x: len(list(vals))
        for x, vals in
        groupby(sorted(list(x_positions)))
    }
    max_vertical = max(vertical_counts.values())
    # print(height, vertical_counts)
    # print(max_vertical / height)
    return max_vertical / height > 0.8


stars = from_string(input)
for time in count(0):
    if detect_word(stars):
        print("Part One : \n" + str(star_repr(stars)))
        print("Part Two : " + str(time))
        exit(1)
    for star in stars:
        star.position = star.position + star.velocity
