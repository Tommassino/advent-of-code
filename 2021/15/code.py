# Advent of code Year 2021 Day 15 solution
# Author = witzatom
# Date = December 2021
from collections import defaultdict
import numpy as np
from queue import PriorityQueue
from utils import timed
from heapq import heappush, heappop


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


_neighbor_moves = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def neighbors(coordinate, grid):
    for x, y in _neighbor_moves:
        nx, ny = x + coordinate[0], y + coordinate[1]
        if 0 <= nx < grid.shape[0] and 0 <= ny < grid.shape[1]:
            yield nx, ny


def shortest_path(grid):
    heap = [(0, (0, 0))]
    end = (grid.shape[0] - 1, grid.shape[1] - 1)
    visited = set()
    while heap:
        distance, current = heappop(heap)
        if current in visited:
            continue
        visited.add(current)
        if current == end:
            return distance
        for neighbor in neighbors(current, grid):
            neighbor_cost = distance + grid[neighbor[0], neighbor[1]]
            heappush(heap, (neighbor_cost, neighbor))

    # this is a hack that worked for part 1 though :D
    # i was hoping there would be never any walking left or up
    # but that is not the case for part 2
    # for idx in range(1, grid.shape[0]):
    #     for x in range(0, idx):
    #         cost_matrix[x, idx] = min(cost_matrix[x-1, idx], cost_matrix[x, idx-1]) + grid[x, idx]
    #     for y in range(0, idx + 1):
    #         cost_matrix[idx, y] = min(cost_matrix[idx, y-1], cost_matrix[idx-1, y]) + grid[idx, y]
    # return int(cost_matrix[-1, -1])


@timed
def part_one(puzzle_input):
    grid = np.array([list(map(int, line)) for line in puzzle_input.splitlines()])
    return shortest_path(grid)


@timed
def part_two(puzzle_input):
    grid = np.array([list(map(int, line)) for line in puzzle_input.splitlines()])
    g = np.array(grid) - 1
    k = 5
    grid = np.concatenate([(g + i) % 9 for i in range(k)], axis=1)
    grid = np.concatenate([(grid + i) % 9 for i in range(k)], axis=0)
    grid += 1
    return shortest_path(grid)
