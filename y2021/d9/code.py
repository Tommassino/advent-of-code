# Advent of code Year 2021 Day 9 solution
# Author = witzatom
# Date = December 2021
from functools import reduce


class Grid:
    def __init__(self, data) -> None:
        self.height = len(data)
        self.width = len(data[0])
        self.data = data

    def neighbor_indices(self, x, y):
        for move_x, move_y in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            if move_x + x < 0 or move_x + x >= self.width:
                continue
            if move_y + y < 0 or move_y + y >= self.height:
                continue
            yield move_x + x, move_y + y

    def neighbors(self, x, y):
        for nx, ny in self.neighbor_indices(x, y):
            yield self.data[ny][nx]

    def get(self, x, y):
        return self.data[y][x]


def run(puzzle_input):
    puzzle_input = list(
        map(lambda x: list(map(int, list(x))), puzzle_input.split("\n"))
    )
    puzzle_input = Grid(puzzle_input)

    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


def part_one(puzzle_input: Grid):
    return sum(1 + puzzle_input.get(x, y) for x, y in local_minima_points(puzzle_input))


def part_two(puzzle_input: Grid):
    basins = find_basins(puzzle_input)
    top_basins = sorted(basins.items(), key=lambda x: len(x[1]), reverse=True)[:3]
    return reduce(lambda x, y: x * y, (len(basin) for _, basin in top_basins))


def local_minima_points(puzzle_input: Grid):
    for y in range(puzzle_input.height):
        for x in range(puzzle_input.width):
            if puzzle_input.get(x, y) < min(puzzle_input.neighbors(x, y)):
                yield x, y


def find_basins(puzzle_input: Grid):
    local_minima = local_minima_points(puzzle_input)

    basins = {idx: {point} for idx, point in enumerate(local_minima)}
    fringe = set((idx, next(iter(points))) for idx, points in basins.items())
    visited = set()

    while len(fringe) > 0:
        basin, (next_x, next_y) = next(iter(fringe))
        visited.add((next_x, next_y))
        fringe.discard((basin, (next_x, next_y)))
        basins[basin].add((next_x, next_y))

        for neighbor_x, neighbor_y in puzzle_input.neighbor_indices(next_x, next_y):
            if (neighbor_x, neighbor_y) in visited:
                continue
            if puzzle_input.get(neighbor_x, neighbor_y) == 9:
                continue

            fringe.add((basin, (neighbor_x, neighbor_y)))

    return basins
