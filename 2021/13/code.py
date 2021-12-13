# Advent of code Year 2021 Day 13 solution
# Author = witzatom
# Date = December 2021
from typing import Set, Tuple


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two :\n{part_two(puzzle_input)}")


def part_one(puzzle_input):
    coordinates, commands = puzzle_input.split("\n\n")
    grid = Grid.parse(coordinates)
    commands = list(command.split("=") for command in commands.split("\n"))
    fold, coordinate = commands[0]
    grid.fold(fold[-1], int(coordinate))
    return len(grid.coordinates)


def part_two(puzzle_input):
    coordinates, commands = puzzle_input.split("\n\n")
    grid = Grid.parse(coordinates)
    commands = list(command.split("=") for command in commands.split("\n"))
    for fold, coordinate in commands:
        grid.fold(fold[-1], int(coordinate))
    return grid


class Grid:
    dimension_map = {"x": 0, "y": 1}

    def __init__(self, coordinates: Set[Tuple[int, int]]) -> None:
        self.coordinates = coordinates

    def __repr__(self) -> str:
        height = max(y for _, y in self.coordinates) + 1
        width = max(x for x, _ in self.coordinates) + 1
        return "\n".join(
            "".join("█" if (x, y) in self.coordinates else " " for x in range(width))
            for y in range(height)
        )

    def fold(self, dimension, fold_coordinate):
        dimension = self.dimension_map[dimension]
        new_coordinates = set()
        for coordinate in self.coordinates:
            if coordinate[dimension] > fold_coordinate:
                new_coordinate = list(coordinate)
                new_coordinate[dimension] = (
                    fold_coordinate - new_coordinate[dimension]
                ) % fold_coordinate
                new_coordinates.add(tuple(new_coordinate))
            else:
                new_coordinates.add(coordinate)
        self.coordinates = new_coordinates

    @staticmethod
    def parse(input):
        return Grid({tuple(map(int, line.split(","))) for line in input.split("\n")})
