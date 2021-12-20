# Advent of code Year 2020 Day 6 solution
# Author = witzatom
# Date = December 2020
import itertools
from functools import reduce


def run(puzzle_input: str):
    groups = [
        parse_group(group)
        for group in puzzle_input.split("\n\n")
    ]

    print(f"Part One : {part1(groups)}")
    print(f"Part Two : {part2(groups)}")


def parse_group(group: str):
    return [
        list(line)
        for line in group.split("\n")
    ]


def part1(groups):
    return sum(
        len(set(itertools.chain(*group)))
        for group in groups
    )


def part2(groups):
    return sum([
        len(reduce(
            lambda a, b: a.intersection(b),
            [set(x) for x in group]
        ))
        for group in groups
    ])
