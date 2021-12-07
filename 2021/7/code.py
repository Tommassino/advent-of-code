# Advent of code Year 2021 Day 7 solution
# Author = witzatom
# Date = December 2021
import numpy as np

def run(puzzle_input):
    positions = list(map(int, puzzle_input.split(",")))
    print(f"Part One : {part_one(positions)}")
    print(f"Part Two : {part_two(positions)}")

def part_one(positions):
    target_position = int(np.median(positions))
    return sum(
        abs(x - target_position)
        for x in positions
    )

def part_two(positions):
    positions = np.array(positions)
    def cost(target_position):
        distance = np.abs(positions - target_position)
        cost = int(np.sum(np.multiply(distance, distance + 1) / 2))
        # print(f"Cost at {target_position}: {cost}")
        return cost

    # yes i know the minimum is in a valley in the distance space 
    # but this works and is so much easier to write :D
    target_position = min(range(min(positions), max(positions)), key=cost)
    target_cost = cost(target_position)
    return target_cost
