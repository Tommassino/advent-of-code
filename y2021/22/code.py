# Advent of code Year 2021 Day 22 solution
# Author = witzatom
# Date = December 2021
import numpy as np
import re
from typing import List, NamedTuple
from tqdm.auto import tqdm
from y2021.utils import timed


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


class ToggleCube(NamedTuple):
    sign: int
    min_x: int
    max_x: int
    min_y: int
    max_y: int
    min_z: int
    max_z: int

    def count(self):
        return (
            self.sign
            * (self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)
        )

    def clip(self, clip_min, clip_max):
        if self.max_x < clip_min or self.min_x > clip_max:
            return None
        if self.max_y < clip_min or self.min_y > clip_max:
            return None
        if self.max_z < clip_min or self.min_z > clip_max:
            return None
        return ToggleCube(
            self.sign,
            max(self.min_x, clip_min),
            min(self.max_x, clip_max),
            max(self.min_y, clip_min),
            min(self.max_y, clip_max),
            max(self.min_z, clip_min),
            min(self.max_z, clip_max),
        )

    def intersect(self, other):
        """

        >>> ToggleCube(1, 10, 12, 10, 12, 10, 12).intersect(ToggleCube(1, 11, 13, 11, 13, 11, 13))
        ToggleCube(sign=-1, min_x=11, max_x=12, min_y=11, max_y=12, min_z=11, max_z=12)

        >>> ToggleCube(1, 10, 12, 8, 12, 10, 12).intersect(ToggleCube(-1, 9, 11, 9, 11, 9, 11))
        ToggleCube(sign=-1, min_x=10, max_x=11, min_y=9, max_y=11, min_z=10, max_z=11)

        >>> ToggleCube(-1, 10, 12, 8, 12, 10, 12).intersect(ToggleCube(-1, 9, 11, 9, 11, 9, 11))
        ToggleCube(sign=1, min_x=10, max_x=11, min_y=9, max_y=11, min_z=10, max_z=11)
        """
        sign = -1
        if self.sign == other.sign:
            sign = self.sign * -1
        else:
            sign = other.sign
        cube = ToggleCube(
            sign,
            max(self.min_x, other.min_x),
            min(self.max_x, other.max_x),
            max(self.min_y, other.min_y),
            min(self.max_y, other.max_y),
            max(self.min_z, other.min_z),
            min(self.max_z, other.max_z),
        )
        if (
            cube.min_x <= cube.max_x
            and cube.min_y <= cube.max_y
            and cube.min_z <= cube.max_z
        ):
            return cube
        return None


example = """on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"""

example_large = """on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15"""


def reboot(steps: List[ToggleCube]):
    """
    >>> sum(x.count() for x in reboot(parse(example_large)))
    590784
    >>> sum(x.count() for x in reboot(parse(example)))
    39
    """
    cubes = []
    for step in tqdm(steps):
        # print(f"Step {step}")
        to_add = []
        if step.sign == 1:
            # print(">", step)
            to_add.append(step)
        for cube in cubes:
            intersection = cube.intersect(step)
            if intersection is not None:
                # print(">", intersection)
                to_add.append(intersection)
        cubes.extend(to_add)
    return cubes


def parse(puzzle_input: str) -> List[ToggleCube]:
    step = re.compile(r".* x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
    return [
        ToggleCube(
            -1 if line.split(" ")[0] == "off" else 1,
            *tuple(map(int, step.match(line).groups())),
        )
        for line in puzzle_input.splitlines()
    ]


@timed
def part_one(puzzle_input: str):
    steps = [
        step.clip(-50, 50)
        for step in parse(puzzle_input)
        if step.clip(-50, 50) is not None
    ]
    cubes = reboot(steps)
    return sum(x.count() for x in cubes)


@timed
def part_two(puzzle_input: str):
    steps = parse(puzzle_input)
    cubes = reboot(steps)
    return sum(x.count() for x in cubes)
