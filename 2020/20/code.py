# Advent of code Year 2020 Day 20 solution
# Author = witzatom
# Date = December 2020
from __future__ import annotations
import numpy as np
from scipy.signal import convolve2d
from enum import Enum
from typing import Dict, Set, Iterable, Tuple


def run(puzzle_input):
    tile_data = parse_input(puzzle_input)
    placed_tiles = tile_image(tile_data)
    print(f"Part One : {part1(placed_tiles)}")
    print(f"Part Two : {part2(placed_tiles)}")


class Direction(Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    @classmethod
    def neighbors(cls, x, y):
        return [
            ((x, y - 1), Direction.NORTH),
            ((x, y + 1), Direction.SOUTH),
            ((x - 1, y), Direction.WEST),
            ((x + 1, y), Direction.EAST),
        ]


class Tile:
    def __init__(self, tile_id: int, data: np.ndarray):
        self.id = tile_id
        self.data = data

    def rotate(self, k=1):
        return type(self)(
            self.id,
            np.rot90(self.data, k=k)
        )

    def flip(self, k=1, axis=0):
        k = k % 2
        if k == 1:
            return type(self)(
                self.id,
                np.flip(self.data, axis=axis)
            )
        else:
            return self

    def placements(self) -> Iterable[Tile]:
        for orientation in range(4):
            tile = self.rotate(orientation)
            for flip in range(2):
                yield tile.flip(flip, axis=0)

    def __repr__(self):
        return "\n".join(
            "".join(
                "#" if v else "."
                for v in self.data[y, :]
            )
            for y in range(self.data.shape[0])
        )

    def matching(self, other: Tile, other_placement: Direction) -> bool:
        self_border = self.border(other_placement)
        if other_placement == Direction.NORTH:
            other_border = other.border(Direction.SOUTH)
        elif other_placement == Direction.SOUTH:
            other_border = other.border(Direction.NORTH)
        elif other_placement == Direction.WEST:
            other_border = other.border(Direction.EAST)
        elif other_placement == Direction.EAST:
            other_border = other.border(Direction.WEST)
        else:
            raise ValueError(other_placement)
        return np.array_equal(self_border, other_border)

    def border(self, direction: Direction) -> np.ndarray:
        if direction == Direction.NORTH:
            return self.data[0, :]
        elif direction == Direction.SOUTH:
            return self.data[-1, :]
        elif direction == Direction.EAST:
            return self.data[:, -1]
        elif direction == Direction.WEST:
            return self.data[:, 0]

    @classmethod
    def from_string(cls, tile: str):
        lines = tile.strip().split("\n")
        header = int(lines.pop(0).replace("Tile ", "").replace(":", ""))
        data = np.array([
            [
                0 if char == '.' else 1
                for char in line
            ]
            for line in lines
        ])
        return cls(header, data)


def parse_input(puzzle_input: str):
    return list(
        Tile.from_string(tile)
        for tile in puzzle_input.split("\n\n")
    )


def tile_image(tiles):
    placed_tiles: Dict[Tuple[int, int], Tile] = dict()
    unplaced_tiles: Set[Tile] = set(tiles)
    fringe_positions: Set[Tuple[int, int]] = {(0, 0)}

    def can_place(tile: Tile, position: Tuple[int, int]) -> bool:
        return all(
            tile.matching(placed_tiles[neighbor_position], neighbor_direction)
            for neighbor_position, neighbor_direction in Direction.neighbors(*position)
            if neighbor_position in placed_tiles
        )

    while unplaced_tiles:
        position, tile, original_tile = next(
            (fringe_position, placement, candidate_tile)
            for candidate_tile in unplaced_tiles
            for placement in candidate_tile.placements()
            for fringe_position in fringe_positions
            if can_place(placement, fringe_position)
        )

        unplaced_tiles.remove(original_tile)
        placed_tiles[position] = tile
        fringe_positions.update(
            x
            for x, _ in Direction.neighbors(*position)
            if x not in placed_tiles
        )
        fringe_positions.remove(position)

    (min_x, min_y) = min(placed_tiles.keys())
    (max_x, max_y) = max(placed_tiles.keys())
    return [
        [
            placed_tiles[(x, y)]
            for x in range(min_x, max_x + 1)
        ]
        for y in range(min_y, max_y + 1)
    ]


def part1(placed_tiles):
    return (
            placed_tiles[0][0].id *
            placed_tiles[0][-1].id *
            placed_tiles[-1][0].id *
            placed_tiles[-1][-1].id
    )


def part2(placed_tiles):
    stitched = np.vstack([
        np.hstack([x.data[1:-1, 1:-1] for x in row])  #
        for row in placed_tiles
    ])
    stitched_tile = Tile(0, stitched)

    monster_str = """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """
    monster_mask = np.flip(np.array([
        np.array([
            1 if char == '#' else 0
            for char in line
        ])
        for line in monster_str.split("\n")
    ]))
    monster_size = np.sum(monster_mask)

    for orientation in stitched_tile.placements():
        monsters = convolve2d(orientation.data, monster_mask)
        detected_monsters = list(zip(*np.where(monsters == monster_size)))
        if len(detected_monsters) > 0:
            return np.sum(orientation.data) - len(detected_monsters) * monster_size
