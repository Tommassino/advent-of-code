# Advent of code Year 2018 Day 6 solution
# Author = witzatom
# Date = December 2018
from typing import List, Any, Optional, Dict, Tuple
from collections import defaultdict

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


class Grid:
    width: int
    height: int
    data: Dict[Tuple[int, int], Any]

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.data = {}

    def __getitem__(self, item) -> Optional[Any]:
        return self.data[item] if item in self.data else None

    def __setitem__(self, key, value):
        self.data[key] = value

    def __repr__(self) -> str:
        return "\n".join([
            "".join([
                str(self.data.get((x, y), "."))
                for x in range(self.width)
            ])
            for y in range(self.height)
        ])


def from_string(data: str) -> Grid:
    coordinates = [
        (int(line.split(",")[0]), int(line.split(",")[1]))
        for line in data.split("\n")
    ]
    max_x = max(coordinates, key=lambda x: x[0])[0]
    max_y = max(coordinates, key=lambda x: x[1])[1]
    grid = Grid(max_x + 1, max_y + 1)
    for idx, coordinate in enumerate(coordinates):
        grid[coordinate] = idx
    return grid


def manhattan_distance(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


grid = from_string(input)
distance_grid = Grid(grid.width, grid.height)
for x in range(grid.width):
    for y in range(grid.height):
        distances = [
            manhattan_distance(point, (x, y))
            for point in grid.data.keys()
        ]
        min_dist = min(distances)
        min_indices = [i for i, x in enumerate(distances) if x == min_dist]
        if len(min_indices) == 1:
            distance_grid[x, y] = min_indices[0]

areas = defaultdict(int)
for idx in range(len(grid.data)):
    points = [
        point
        for point, key in distance_grid.data.items()
        if key == idx
    ]
    is_edge = any(
        point[0] == 0 or point[1] == 0 or point[0] == grid.width - 1 or point[1] == grid.height
        for point in points
    )
    if not is_edge:
        areas[idx] = len(points)

largest_area = max(areas.items(), key=lambda x: x[1])

print("Part One : " + str(largest_area[1]))

all_distance_grid = Grid(grid.width, grid.height)
for x in range(grid.width):
    for y in range(grid.height):
        distances = [
            manhattan_distance(point, (x, y))
            for point in grid.data.keys()
        ]
        all_distance_grid[x, y] = int(sum(distances) < 10000)
region_size = len([
    1
    for x in range(grid.width)
    for y in range(grid.height)
    if all_distance_grid[x, y] == 1
])

print("Part Two : " + str(region_size))
