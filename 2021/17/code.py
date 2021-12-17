# Advent of code Year 2021 Day 17 solution
# Author = witzatom
# Date = December 2021
# from __future__ import annotations
import re
from typing import NamedTuple
import math
from dataclasses import dataclass, replace
from utils import timed


@dataclass
class Point2:
    x: int
    y: int

    def add(self, other):
        self.x += other.x
        self.y += other.y


class TargetArea(NamedTuple):
    x_min: int
    x_max: int
    y_min: int
    y_max: int

    def isin(self, other) -> bool:
        if other.x < self.x_min or self.x_max < other.x:
            return False
        if other.y < self.y_min or self.y_max < other.y:
            return False
        return True


@timed
def run(puzzle_input):
    target_area = parse(puzzle_input)
    visualize(target_area)
    velocities = find_velocities(target_area)
    print(f"Part One : {max(velocities.values())}")
    print(f"Part Two : {len(velocities.keys())}")


def parse(puzzle_input):
    pattern = r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)"
    target = tuple(map(int, re.match(pattern, puzzle_input).groups()))
    return TargetArea(*target)


def launch(velocity: Point2, target_area: TargetArea):
    position = Point2(0, 0)
    top_y = position.y
    while position.x <= target_area.x_max and position.y >= target_area.y_min:
        position.add(velocity)
        top_y = max(top_y, position.y)

        if target_area.isin(position):
            return top_y

        velocity.y -= 1  # gravity
        velocity.x = max(0, velocity.x - 1)  # drag
    return -1


def find_velocities(target_area: TargetArea):
    min_x_v = round(math.sqrt(target_area.x_min * 2))
    return dict(
        filter(
            lambda x: x[1] >= 0,
            (
                ((x, y), launch(Point2(x, y), target_area))
                for x in range(min_x_v, target_area.x_max + 1)
                for y in range(target_area.y_min, -target_area.y_min)
            ),
        )
    )

import numpy as np
import matplotlib.pyplot as plt
from matplotlib import animation
from tqdm.auto import tqdm

def trajectory(velocity: Point2, target_area: TargetArea):
    position = Point2(0, 0)
    yield replace(position)
    while position.x <= target_area.x_max and position.y >= target_area.y_min:
        position.add(velocity)
        yield replace(position)
        if target_area.isin(position):
            break
        velocity.y -= 1  # gravity
        velocity.x = max(0, velocity.x - 1)  # drag

def visualize(target_area: TargetArea):
    fig, ax = plt.subplots()
    fig.set_size_inches(10, 10)
    velocities = find_velocities(target_area)
    trajectories = list(
        tuple(zip(*list((t.x, t.y) for t in trajectory(Point2(*velocity), target_area))))
        for velocity in velocities.keys()
    )
    min_x = min(
        min(t)
        for t, _ in trajectories
    )
    max_x = max(
        max(t)
        for t, _ in trajectories
    )
    min_y = min(
        min(t)
        for _, t in trajectories
    )
    max_y = max(
        max(t)
        for _, t in trajectories
    )

    num_frames = max(
        len(tr[0]) + 4
        for tr in trajectories
    )
    with tqdm(total=num_frames) as pbar:
        def animate_step(frame):
            pbar.update(1)
            ax.clear()
            ax.set_xlim((min_x - 10, max_x + 10))
            ax.set_ylim((min_y - 10, max_y + 10))
            for x, y in trajectories:
                ax.plot(x[:frame], y[:frame], alpha=0.1)
            for x, y in trajectories:
                vector_x = x[frame-2:frame]
                vector_y = y[frame-2:frame]
                if len(vector_x) != 2:
                    continue
                vector = np.array((vector_x[1] - vector_x[0], vector_y[1] - vector_y[0]))
                size = np.linalg.norm(vector)
                if size <= 0:
                    continue
                vector = vector / size * 10
                ax.plot(
                    [vector_x[1] - vector[0], vector_x[1]],
                    [vector_y[1] - vector[1], vector_y[1]]
                )
                ax.scatter([vector_x[1]], [vector_y[1]], s=5, marker='X')

        anim = animation.FuncAnimation(fig, animate_step, frames = num_frames, interval=50)
        anim.save('trench.gif')
