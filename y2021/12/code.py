# Advent of code Year 2021 Day 12 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from collections import defaultdict
from typing import Iterable, Tuple, List


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


def part_one(puzzle_input: str) -> int:
    graph = Graph.parse(puzzle_input)
    start = "start"
    end = "end"
    paths = []
    queue = [[start]]
    while len(queue) > 0:
        path = queue.pop(0)
        last = path[-1]
        for neighbor in graph.neighbors(last):
            if neighbor.islower() and neighbor in path:
                continue
            next_path = path + [neighbor]
            if neighbor == end:
                paths.append(next_path)
            else:
                queue.append(next_path)
    return len(paths)


def can_visit(path: List[str], revisits: int, next: str) -> bool:
    if next == "start":
        return False
    if not next.islower():
        return True
    revisit_count = path.count(next)
    return revisit_count + revisits <= 1


def part_two(puzzle_input: str) -> int:
    graph = Graph.parse(puzzle_input)
    end = "end"
    paths = []
    queue = [(["start"], 0)]
    while len(queue) > 0:
        path, revisits = queue.pop(0)
        last = path[-1]
        for neighbor in graph.neighbors(last):
            if not can_visit(path, revisits, neighbor):
                continue
            revisit_count = path.count(neighbor) if neighbor.islower() else 0
            next_revisits = revisits + revisit_count
            next_path = path + [neighbor]
            if neighbor == end:
                paths.append(next_path)
            else:
                queue.append((next_path, next_revisits))
    # print("\n".join(
    #     ",".join(path)
    #     for path in sorted(paths)
    # ))
    return len(paths)


class Graph:
    def __init__(self, connections: Iterable[Tuple[str, str]]) -> None:
        self._neighbors = defaultdict(set)
        for start, end in connections:
            self._neighbors[start].add(end)
            self._neighbors[end].add(start)
        self._neighbors = dict(self._neighbors)

    def __repr__(self) -> str:
        return "\n".join(
            f"{start} -> {', '.join(ends)}" for start, ends in self._neighbors.items()
        )

    def neighbors(self, node: str) -> List[str]:
        if node in self._neighbors:
            return self._neighbors[node]
        return []

    @staticmethod
    def parse(input: str) -> Graph:
        return Graph(line.split("-") for line in input.split("\n"))
