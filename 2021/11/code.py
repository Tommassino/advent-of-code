# Advent of code Year 2021 Day 11 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
import numpy as np
from itertools import count
import time
import os
from functools import lru_cache
from utils.utils import timed
from copy import copy
import matplotlib.pyplot as plt
from matplotlib import animation
from tqdm.auto import tqdm

def run(puzzle_input):
    print(f"Part One : {part_one(OctopusGrid.parse(puzzle_input))}")
    print(f"Part Two : {part_two(OctopusGrid.parse(puzzle_input))}")
    visualize(OctopusGrid.parse(puzzle_input))

def part_one(grid: OctopusGrid):
    total_flashes = 0
    for _ in range(100):
        total_flashes += len(grid.step())
    return total_flashes

@timed
def part_two(grid: OctopusGrid):
    for step in count():
        flashes = grid.step()
        if len(flashes) == grid.size:
            return step + 1

def visualize(grid: OctopusGrid):
    fig, ax = plt.subplots()
    num_frames = part_one(copy(grid))
    num_frames = 20
    marker = u"\U0001F419"
    fontname = "Segoe UI Emoji"
    with tqdm(total=num_frames) as pbar:
        def animate_step(i):
            pbar.update(1)
            grid.step()
            data = grid.data
            ax.clear()
            ax.set_xlim([0, grid.width])
            ax.set_ylim([0, grid.height])
            ax.set_axis_off()
            #ax.imshow(data)
            #ax.text(0,.5,'😀 😃 😄 😁 😆 😅 😂 🤣 ☺️ 😊 😇',fontsize=20)

            for x in range(grid.width):
                for y in range(grid.height):
                    plt.text(x, y, marker, fontname=fontname, size=data[x, y] + 1)
        anim = animation.FuncAnimation(fig, animate_step, frames = num_frames)
        anim.save('octopi.gif')

class OctopusGrid:
    def __init__(self, data) -> None:
        self.data = data

    def __repr__(self) -> str:
        return str(self.data)

    @property
    def width(self):
        return self.data.shape[0]

    @property
    def height(self):
        return self.data.shape[1]

    @property
    def size(self):
        return self.data.shape[0] * self.data.shape[1]

    def step(self):
        self.data += 1
        to_flash = set(zip(*np.where(self.data > 9)))
        flashed_coordinates = set()
        while len(to_flash) > 0:
            x, y = to_flash.pop()
            self.data[self.neighbors(x, y)] += 1
            flashed_coordinates.add((x, y))
            to_flash.update(zip(*np.where(self.data > 9)))
            to_flash.difference_update(flashed_coordinates)
        if len(flashed_coordinates) > 0:
            self.data[tuple(zip(*flashed_coordinates))] = 0
        return flashed_coordinates
        
    @lru_cache(100)
    def neighbors(self, x, y):
        return tuple(zip(
            *(
                (nx, ny)
                for nx, ny in np.array([
                    [-1, 0], [1, 0], [0, -1], [0, 1],
                    [-1, -1], [-1, 1], [1, -1], [1, 1]
                ]) + [x, y]
                if nx >=0 and ny >= 0 and nx < self.data.shape[0] and ny < self.data.shape[1]
            )
        ))

    @staticmethod
    def parse(input):
        return OctopusGrid(np.array(
            list(
                list(map(int, list(line)))
                for line in input.split("\n")
            )
        ))
