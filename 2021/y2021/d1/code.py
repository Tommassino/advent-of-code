# Advent of code Year 2021 Day 1 solution
# Author = witzatom
# Date = December 2021

from y2021.utils.utils import window


def run(puzzle_input):
    puzzle_input = list(map(int, puzzle_input.split("\n")))
    part_one = sum(first < second for first, second in window(puzzle_input))
    print(f"Part One : {part_one}")

    part_two = sum(
        sum(first_window) < sum(second_window)
        for first_window, second_window in window(window(puzzle_input, n=3))
    )
    print(f"Part Two : {part_two}")
