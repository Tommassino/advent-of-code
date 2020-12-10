# Advent of code Year 2020 Day 9 solution
# Author = witzatom
# Date = December 2020
from itertools import combinations

from utils import window


def run(puzzle_input):
    numbers = [
        int(line) for line in puzzle_input.split("\n")
    ]
    result_1 = part1(numbers)
    print(f"Part One : {result_1}")
    print(f"Part Two : {part2(numbers, result_1)}")


def part1(numbers, preamble_size=25):
    for check in window(numbers, n=preamble_size + 1):
        to_check = check[-1]
        is_valid = any(
            to_check == sum(combination)
            for combination in combinations(check[:-1], 2)
        )
        if not is_valid:
            return to_check


def part2(numbers, checksum):
    start = 0
    end = 1
    while end <= len(numbers):
        check = sum(numbers[start:end])
        if check < checksum:
            end += 1
        elif check > checksum:
            start += 1
        else:
            return min(numbers[start:end]) + max(numbers[start:end])
