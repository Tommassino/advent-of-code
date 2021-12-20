# Advent of code Year 2020 Day 12 solution
# Author = witzatom
# Date = December 2020
import numpy as np


def run(puzzle_input):
    commands = [
        (line[0], int(line[1:]))
        for line in puzzle_input.split("\n")
    ]
    print(f"Part One : {part1(commands)}")
    print(f"Part Two : {part2(commands)}")


DIRECTIONS = {
    "N": np.array((-1, 0)),
    "S": np.array((1, 0)),
    "E": np.array((0, 1)),
    "W": np.array((0, -1)),
}


def part1(commands):
    position = np.array((0, 0))
    direction = np.array((0, 1))
    for command, argument in commands:
        if command in DIRECTIONS:
            position = position + DIRECTIONS[command] * argument
        elif command == 'F':
            position = position + direction * argument
        else:
            assert argument <= 360
            turns = argument // 90
            if command == 'L':
                turns = 4 - argument // 90
            for i in range(turns):
                direction[[0, 1]] = direction[[1, 0]]
                direction[1] *= -1
    return np.sum(np.abs(position))


def part2(commands):
    position = np.array((0, 0))
    direction = np.array((-1, 10))
    for command, argument in commands:
        if command in DIRECTIONS:
            direction = direction + DIRECTIONS[command] * argument
        elif command == 'F':
            position = position + direction * argument
        else:
            assert argument <= 360
            turns = argument // 90
            if command == 'L':
                turns = 4 - argument // 90
            for i in range(turns):
                direction[[0, 1]] = direction[[1, 0]]
                direction[1] *= -1
    return np.sum(np.abs(position))
