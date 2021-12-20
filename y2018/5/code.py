# Advent of code Year 2018 Day 5 solution
# Author = witzatom
# Date = December 2018

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


def react(polymer: str) -> str:
    def reacts(c1: str, c2: str):
        return c1.lower() == c2.lower() and c1.islower() != c2.islower()

    result = []
    for c in polymer:
        if result and reacts(result[-1], c):
            result.pop()
        else:
            result.append(c)
    return "".join(result)


print("Part One : " + str(len(react(input))))

from string import ascii_lowercase

min_lenght = None
min_polymer = None
for c in ascii_lowercase:
    fixed_polymer = react(input.replace(c, "").replace(c.upper(), ""))
    if min_lenght is None or len(fixed_polymer) < min_lenght:
        min_lenght = len(fixed_polymer)
        min_polymer = fixed_polymer

print("Part Two : " + str(min_lenght))
