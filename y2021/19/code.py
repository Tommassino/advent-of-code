# Advent of code Year 2021 Day 19 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from collections import defaultdict
from typing import Dict, List, Set, Tuple
import itertools
from y2021.utils import timed
from typing import NamedTuple


class Vector3(NamedTuple):
    x: ...
    y: ...
    z: ...

    def __add__(self, other: Vector3) -> Vector3:
        return Vector3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: Vector3) -> Vector3:
        return Vector3(self.x - other.x, self.y - other.y, self.z - other.z)

    def roll(self) -> Vector3:
        return Vector3(self.x, self.z, -self.y)

    def turn(self) -> Vector3:
        return Vector3(-self.y, self.x, self.z)

    def rotations(self) -> List[Vector3]:
        """Generates all 3d rotations of a vector
        https://stackoverflow.com/a/16467849/5422958

        >>> len(Vector3(1,-1,2).rotations())
        24
        """
        a = []
        vector = self
        for cycle in range(2):
            for step in range(3):
                vector = vector.roll()
                a.append(vector)
                for i in range(3):
                    vector = vector.turn()
                    a.append(vector)
            vector = vector.roll().turn().roll()
        return a


def run(puzzle_input):
    placements, beacons = place_scanners(puzzle_input)
    print(f"Part One : {len(beacons)}")
    print(f"Part Two : {part_two(placements)}")


def part_two(placements: Dict[int, Vector3]):
    return max(
        sum(abs(x[i] - y[i]) for i in range(len(x)))
        for x, y in itertools.permutations(placements.values(), 2)
    )


@timed
def place_scanners(puzzle_input: str) -> Tuple[Dict[int, Vector3], Set[Vector3]]:
    # blech this parsing
    scanners = [
        [
            *zip(
                *list(
                    Vector3(*tuple(int(x) for x in line.split(","))).rotations()
                    for line in scanner.splitlines()[1:]
                )
            )
        ]
        for scanner in puzzle_input.split("\n\n")
    ]
    placed_rotation = {0: 0}
    placed_relation = {0: Vector3(0, 0, 0)}
    beacons = set(scanners[0][0])
    # memoization on find_scanner_overlap
    skip_placements = defaultdict(set)

    while len(placed_rotation) < len(scanners):
        print(len(placed_rotation))
        for scanner_idx, scanner in enumerate(scanners):
            if scanner_idx in placed_rotation:
                continue
            for set_idx, set_rotation in [*placed_rotation.items()]:
                if set_idx in skip_placements[scanner_idx]:
                    continue

                overlap = find_scanner_overlap(scanners[set_idx], set_rotation, scanner)
                if overlap is None:
                    skip_placements[scanner_idx].add(set_idx)
                    continue
                relation, rotation = overlap

                placed_rotation[scanner_idx] = rotation
                placed_relation[scanner_idx] = relation + placed_relation[set_idx]
                beacons.update(
                    vector - placed_relation[scanner_idx]
                    for vector in scanner[rotation]
                )
                break
            else:
                continue
            break

    return placed_relation, beacons


def find_coordinate_overlap(
    coordinates_a: List[Vector3], coordinates_b: List[Vector3], n=12
) -> Vector3:
    """Finds whether there is a n point overlap between sets of 3d coordinates"""
    coordinates_a = set(coordinates_a)
    for beacon_a, beacon_b in itertools.product(coordinates_a, coordinates_b):
        # try to place beacon_b on beacon_a
        relation = beacon_b - beacon_a
        overlap = coordinates_a.intersection(
            vector - relation for vector in coordinates_b
        )
        if len(overlap) == n:
            return relation


def find_scanner_overlap(
    scanner_a: List[Vector3], scanner_a_rotation: int, scanner_b: List[List[Vector3]]
) -> Tuple[Vector3, int]:
    """Rotates scanner b to see whether it can be overlapped with scaner a with a fixed rotation"""
    for rotation_id, rotated_coordinates in enumerate(scanner_b):
        relation = find_coordinate_overlap(
            scanner_a[scanner_a_rotation], rotated_coordinates
        )
        if relation is None:
            continue
        return relation, rotation_id
