# Advent of code Year 2018 Day 10 solution
# Author = witzatom
# Date = December 2018
from __future__ import annotations
from dataclasses import dataclass
import numpy as np
from typing import List
import re
from itertools import count, groupby


def solve():
    with open((__file__.rstrip("code.py") + "christmas.txt"), 'r') as input_file:
        input = input_file.read()

    star_chart = StarChart.from_string(input)
    for time in count(0):
        print(time)
        if star_chart.detect_word():
            print("Part One : \n" + str(star_chart))
            print("Part Two : " + str(time))
            exit(1)
        star_chart.tick()


@dataclass(frozen=False)
class Star:
    position: np.array
    velocity: np.array


class StarChart:
    def __init__(self, stars: List[Star]):
        self.stars = stars

    @classmethod
    def from_string(cls, data) -> StarChart:
        star_re = re.compile(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
        stars = []
        for line in data.split("\n"):
            star_match = star_re.match(line)
            position = np.array(star_match.groups()[:2]).astype(int)
            velocity = np.array(star_match.groups()[2:]).astype(int)
            stars.append(Star(position, velocity))
        return StarChart(stars)

    def __repr__(self):
        min_x = min(star.position[0] for star in self.stars)
        max_x = max(star.position[0] for star in self.stars)
        min_y = min(star.position[1] for star in self.stars)
        max_y = max(star.position[1] for star in self.stars)
        positions = set(
            tuple(star.position)
            for star in self.stars
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

    def tick(self):
        for star in self.stars:
            star.position = star.position + star.velocity

    def detect_word(self):
        x_positions = list(
            star.position[0]
            for star in self.stars
        )
        min_y = min(star.position[1] for star in self.stars)
        max_y = max(star.position[1] for star in self.stars)
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


letters = {
    "V": [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (2, 6),
        (3, 3),
        (3, 4),
        (3, 5),
        (4, 0),
        (4, 1),
        (4, 2),

    ],
    "A": [
        (0, 4),
        (0, 5),
        (0, 6),
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 0),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 3),
        (4, 4),
        (4, 5),
        (4, 6),
    ],
    "N": [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (1, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (3, 5),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
        (4, 6),

    ],
    "O": [
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (1, 0),
        (1, 6),
        (2, 0),
        (2, 6),
        (3, 0),
        (3, 6),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
    ],
    "C": [
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (1, 0),
        (1, 6),
        (2, 0),
        (2, 6),
        (3, 0),
        (3, 6),
        (4, 1),
        (4, 5),

    ],
    "E": [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (1, 0),
        (1, 3),
        (1, 6),
        (2, 0),
        (2, 3),
        (2, 6),
        (3, 0),
        (3, 3),
        (3, 6),
        (4, 0),
        (4, 6),

    ],
    "S": [
        (0, 1),
        (0, 2),
        (0, 5),
        (1, 0),
        (1, 3),
        (1, 6),
        (2, 0),
        (2, 3),
        (2, 6),
        (3, 0),
        (3, 3),
        (3, 6),
        (4, 1),
        (4, 4),
        (4, 5),
    ],
    "L": [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (1, 6),
        (2, 6),
        (3, 6),
        (4, 6),
    ]
}

if __name__ == "__main__":
    writing1 = "VESELE"
    writing = "VANOCE"
    stars = [
                Star(np.array([position[0] + 7 * idx, position[1]]), np.array([0, 0]))
                for idx, letter in enumerate(writing1)
                for position in letters[letter]
            ] + [
                Star(np.array([position[0] + 7 * idx, position[1] + 9]), np.array([0, 0]))
                for idx, letter in enumerate(writing)
                for position in letters[letter]
            ]
    import random

    for star in stars:
        star.velocity[0] = random.randint(-2, 2)
        star.velocity[1] = random.randint(-2, 2)

    chart = StarChart(stars)
    print(chart)
    for i in range(15):
        chart.tick()

    print(chart)
    with open((__file__.rstrip("code.py") + "christmas.txt"), 'w') as input_file:
        for star in chart.stars:
            input_file.write(
                f"position=< {star.position[0]},  {star.position[1]}> velocity=< {-star.velocity[0]},  {-star.velocity[1]}>"
            )
            input_file.write("\n")
