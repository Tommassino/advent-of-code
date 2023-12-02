# Advent of code Year 2020 Day 10 solution
# Author = witzatom
# Date = December 2020
from collections import defaultdict

import numpy as np


def run(puzzle_input):
    numbers = [
        int(number) for number in puzzle_input.split("\n")
    ]
    print(f"Part One : {part1(numbers)}")
    print(f"Part Two : {part2(numbers)}")


def part1(numbers):
    counts = np.bincount(np.diff(sorted(numbers)))
    return (counts[1] + 1) * (counts[3] + 1)


def part2(numbers):
    end_number = max(numbers) + 3
    numbers = sorted(numbers) + [end_number]
    path_lengths = defaultdict(int)
    path_lengths[0] = 1
    for num in numbers:
        for jump in range(1, 4):
            path_lengths[num] += path_lengths[num - jump]
    return path_lengths[end_number]
