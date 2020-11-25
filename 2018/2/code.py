# Advent of code Year 2018 Day 2 solution
# Author = witzatom
# Date = December 2018
from collections import Counter
import numpy as np

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


def counts(line):
    counter = Counter(Counter(line).values())
    return counter.get(2, 0) >= 1, counter.get(3, 0) >= 1


counters = np.sum([
    counts(line)
    for line in input.split("\n")
], axis=0)

print("Part One : " + str(counters[0] * counters[1]))


def factorize(line):
    for i in range(len(line)):
        yield i, line[:i] + line[i + 1:]


commons = Counter([
    element
    for line in input.split("\n")
    for element in factorize(line)
])
(_, match), _ = max(commons.items(), key=lambda x: x[1])

print("Part Two : " + str(match))
