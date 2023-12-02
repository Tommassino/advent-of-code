# Advent of code Year 2020 Day 17 solution
# Author = witzatom
# Date = December 2020
from collections import Counter, defaultdict
from itertools import product


class Grid:
    def __init__(self, active_coordinates, dimension):
        self.active_coordinates = active_coordinates
        self.dimension = dimension

    def neighbors(self, coordinate):
        for move in product([-1, 0, 1], repeat=self.dimension):
            if set(move) == {0}:
                continue
            yield tuple(sum(x) for x in zip(coordinate, move))

    def tick(self):
        def should_be_active(coordinate, activation):
            if coordinate in self.active_coordinates:
                return activation == 2 or activation == 3
            else:
                return activation == 3

        coordinate_activations = Counter(
            neighbor
            for coordinate in self.active_coordinates
            for neighbor in self.neighbors(coordinate)
        )

        self.active_coordinates = {
            coordinate
            for coordinate, activation in coordinate_activations.items()
            if should_be_active(coordinate, activation)
        }

    def __repr__(self):
        x_levels = set(coordinate[0] for coordinate in self.active_coordinates)
        min_x = min(x_levels)
        max_x = max(x_levels)
        y_levels = set(coordinate[1] for coordinate in self.active_coordinates)
        min_y = min(y_levels)
        max_y = max(y_levels)
        rest_coordinates = sorted(list(set(
            coordinate[2:] for coordinate in self.active_coordinates
        )), key=lambda x: list(reversed(x)))

        repr = []
        repr.append(f"x=[{min_x}, {max_x}], y=[{min_y}, {max_y}]")
        for space in rest_coordinates:
            repr.append(f"z={space}")
            repr.extend(
                "".join(
                    "#" if (x, y, *space) in self.active_coordinates else "."
                    for x in range(min_x, max_x + 1)
                )
                for y in range(min_y, max_y + 1)
            )
            repr.append(f"\n")

        return "\n".join(repr)


def run(puzzle_input):
    print(f"Part One : {part1(puzzle_input)}")
    print(f"Part Two : {part2(puzzle_input)}")


def part1(puzzle_input):
    coordinates = {
        (x, y, 0)
        for y, line in enumerate(puzzle_input.split())
        for x, c in enumerate(line)
        if c == '#'
    }
    grid = Grid(coordinates, 3)
    for i in range(6):
        grid.tick()
    return len(grid.active_coordinates)


def part2(puzzle_input):
    coordinates = {
        (x, y, 0, 0)
        for y, line in enumerate(puzzle_input.split())
        for x, c in enumerate(line)
        if c == '#'
    }
    grid = Grid(coordinates, 4)
    for i in range(6):
        grid.tick()
    return len(grid.active_coordinates)
