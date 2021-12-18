# Advent of code Year 2020 Day 24 solution
# Author = witzatom
# Date = December 2020
from collections import Counter
import re
import numpy as np
from functools import lru_cache


def run(puzzle_input):
    coordinates = [
        parse_coordinate(line)
        for line in puzzle_input.split("\n")
    ]
    print(f"Part One : {part1(coordinates)}")
    print(f"Part Two : {part2(coordinates)}")


coordinate_split = re.compile(r"(se|sw|ne|nw|e|w)")

moves = {
    # the coordinate system is
    # 1: axis to SW <-> NE
    # 2: axis to SE <-> NW
    'se': np.array([0, -1]),
    'ne': np.array([1, 0]),

    'sw': np.array([-1, 0]),
    'nw': np.array([0, 1]),

    'w': np.array([-1, 1]),  # ~ SW + NW
    'e': np.array([1, -1]),  # ~ SE + NE
}


def parse_coordinate(line):
    coordinate_raw = np.sum(
        [
            moves[move]
            for move in coordinate_split.split(line)
            if move
        ], axis=0
    )
    return tuple(coordinate_raw)


class Floor:
    def __init__(self, black_coordinates):
        self.black_coordinates = black_coordinates

    @staticmethod
    @lru_cache(10000)
    def neighbors(coordinate):
        (q, r) = coordinate
        return [
            (move[0] + q, move[1] + r)
            for direction, move in moves.items()
        ]

    def flip_tiles(self, times: int):
        def is_black(coordinate, activation):
            if coordinate in self.black_coordinates:
                return activation == 1 or activation == 2
            else:
                return activation == 2

        for i in range(times):
            # print(f"Day {i}: {len(self.black_coordinates)}")
            activations = Counter(
                neighbor
                for coordinate in self.black_coordinates
                for neighbor in self.neighbors(coordinate)
            )
            new_blacks = set(
                coordinate
                for coordinate, activation in activations.items()
                if is_black(coordinate, activation)
            )
            self.black_coordinates = new_blacks


def part1(coordinates):
    hex_grid = Counter(coordinates)
    return sum(x % 2 for x in hex_grid.values())


def part2(coordinates):
    hex_grid = Counter(coordinates)
    black_coordinates = set(
        coordinate
        for coordinate, flips in hex_grid.items()
        if flips % 2 == 1
    )
    floor_plan = Floor(black_coordinates)
    floor_plan.flip_tiles(100)
    return len(floor_plan.black_coordinates)
