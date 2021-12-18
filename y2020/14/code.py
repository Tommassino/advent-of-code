# Advent of code Year 2020 Day 14 solution
# Author = witzatom
# Date = December 2020
from collections import defaultdict
from itertools import product


def run(puzzle_input):
    print(f"Part One : {part1(puzzle_input)}")
    print(f"Part Two : {part2(puzzle_input)}")


def part1(puzzle_input: str):
    mask_and = 0b111111111111111111111111111111111111
    mask_or = 0
    memory = {}
    for line in puzzle_input.split("\n"):
        if line.startswith("mask = "):
            mask_str = line.replace("mask = ", "")
            mask_and = int(mask_str.replace("X", "1"), 2)
            mask_or = int(mask_str.replace("X", "0"), 2)
        elif line.startswith("mem["):
            split = line.replace("mem[", "").split("] = ")
            memory[int(split[0])] = (int(split[1]) & mask_and) | mask_or
    return sum(memory.values())


def part2(puzzle_input: str):
    mask_numbers = defaultdict(list)
    memory = {}

    for line in puzzle_input.split("\n"):
        if line.startswith("mask = "):
            mask_str = line.replace("mask = ", "")
            mask_numbers = defaultdict(list)
            for idx, c in enumerate(mask_str):
                mask_numbers[c].append(idx)
            del mask_numbers["0"]
        elif line.startswith("mem["):
            split = line.replace("mem[", "").split("] = ")
            address = list("{0:b}".format(int(split[0])))
            address = ['0'] * (36 - len(address)) + address
            for idx in mask_numbers['1']:
                address[idx] = '1'
            assignment = int(split[1])
            for combination in product(['0', '1'], repeat=len(mask_numbers['X'])):
                for idx, value in zip(mask_numbers['X'], combination):
                    address[idx] = value
                address_int = int("".join(address), 2)
                memory[address_int] = assignment
    return sum(memory.values())
