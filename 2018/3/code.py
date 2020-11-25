# Advent of code Year 2018 Day 3 solution
# Author = witzatom
# Date = December 2018

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()

from dataclasses import dataclass


@dataclass
class Claim:
    id: int
    x: int
    y: int
    width: int
    height: int

    def right(self):
        return self.x + self.width

    def bottom(self):
        return self.y + self.height

    def overlaps(self, other):
        return not (
                self.right() <= other.x or
                self.bottom() <= other.y or
                self.x >= other.right() or
                self.y >= other.bottom()
        )


import re

claim_pattern = re.compile("#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")


def from_string(line: str) -> Claim:
    match = claim_pattern.match(line)
    if not match:
        raise ValueError(line)
    return Claim(
        int(match.group(1)),
        int(match.group(2)),
        int(match.group(3)),
        int(match.group(4)),
        int(match.group(5))
    )


claims = [
    from_string(line)
    for line in input.split("\n")
]

conflicted_coordinates = set()
claimed_coordinates = set()
for claim in claims:
    for x in range(claim.x, claim.right()):
        for y in range(claim.y, claim.bottom()):
            coordinate = (x, y)
            if coordinate in claimed_coordinates:
                conflicted_coordinates.add(coordinate)
            claimed_coordinates.add(coordinate)

print("Part One : " + str(len(conflicted_coordinates)))

for claim1 in claims:
    overlaps = False
    for claim2 in claims:
        if claim1 == claim2:
            continue
        overlaps = overlaps or claim1.overlaps(claim2)
    if not overlaps:
        print("Part Two : " + str(claim1.id))
