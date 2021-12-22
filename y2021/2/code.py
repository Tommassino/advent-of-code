# Advent of code Year 2021 Day 2 solution
# Author = witzatom
# Date = December 2021

from itertools import product
from y2021.utils.parsing import lines


def run(puzzle_input):
    parsed_input = parse_input(puzzle_input)
    position = [0, 0]
    for (direction_v, direction_h), amplitude in parsed_input:
        if direction_v != 0:
            position[0] += amplitude * direction_v
        if direction_h != 0:
            position[1] += amplitude * direction_h
    part_one = position[0] * position[1]

    position = [0, 0]
    aim = 0
    for (direction_v, direction_h), amplitude in parsed_input:
        if direction_h != 0:
            position[0] += amplitude * direction_h
            position[1] += amplitude * aim
        if direction_v != 0:
            aim += amplitude * direction_v
    part_two = position[0] * position[1]

    print(f"Part One : {part_one}")
    print(f"Part Two : {part_two}")


def parse_input(puzzle_input):
    return [
        (direction(line.split(" ")[0]), int(line.split(" ")[1]))
        for line in lines(puzzle_input)
    ]


def direction(value):
    if value == "forward":
        return (0, 1)
    if value == "down":
        return (1, 0)
    if value == "up":
        return (-1, 0)
    raise ValueError(value)
