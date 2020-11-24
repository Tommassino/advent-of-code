# Advent of code Year 2018 Day 1 solution
# Author = witzatom
# Date = December 2018
from itertools import cycle

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()

numbers = [int(i) for i in input.split("\n")]
solution = sum(numbers)

print("Part One : " + str(solution))


def first_duplicate(numbers):
    frequency = 0
    seen = {frequency}
    for i in cycle(numbers):
        frequency += i
        if frequency in seen:
            return frequency
        seen.add(frequency)


print("Part Two : " + str(first_duplicate(numbers)))
