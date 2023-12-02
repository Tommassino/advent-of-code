# Advent of code Year 2020 Day 3 solution
# Author = witzatom
# Date = December 2020
import numpy as np


def run(puzzle_input):
    toboggan = np.array([
        [c == '#' for c in line]
        for line in puzzle_input.split("\n")
    ])

    def tree_collisions(vector, toboggan):
        trajectory = np.cumsum(
            np.tile(vector[:, np.newaxis], int(np.ceil(toboggan.shape[0] / vector[0])) - 1),
            axis=1
        )
        assert trajectory[0, -1] + vector[0] >= toboggan.shape[0]
        trajectory[1, :] = np.mod(trajectory[1, :], toboggan.shape[1])
        return np.sum(toboggan[trajectory[0], trajectory[1]])

    collisions = [
        tree_collisions(vector, toboggan)
        for vector in [
            np.array([1, 1]),
            np.array([1, 3]),
            np.array([1, 5]),
            np.array([1, 7]),
            np.array([2, 1])
        ]
    ]

    print(f"Part One : {collisions[1]}")
    print(f"Part Two : {np.prod(collisions, dtype=np.int64)}")
