# Advent of code Year 2018 Day 11 solution
# Author = witzatom
# Date = December 2018
import numpy as np

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


def power_level(x, y, grid_serial_number):
    rack_id = x + 10
    power = (rack_id * y + grid_serial_number) * rack_id
    power = (power % 1000) // 100 - 5
    return power


def make_grid(grid_serial_number, width=300, height=300):
    return np.array([
        [
            power_level(x, y, grid_serial_number)
            for x in range(width)
        ]
        for y in range(height)
    ])


def square_sums(grid, dimension=3):
    cumsum = np.cumsum(grid, axis=0)
    cumsum = np.vstack([cumsum[dimension, :], cumsum[dimension:] - cumsum[:-dimension]])
    cumsum = np.cumsum(cumsum, axis=1)
    cumsum = np.hstack([cumsum[:, dimension][:, np.newaxis], cumsum[:, dimension:] - cumsum[:, :-dimension]])
    return cumsum


grid_serial_number = int(input)
grid = make_grid(grid_serial_number)
squares = square_sums(grid)
y, x = np.unravel_index(np.argmax(squares, axis=None), squares.shape)

print(f"Part One : {x}, {y}")


def generate_squares(grid, max_size=300):
    for i in range(1, max_size):
        squares = square_sums(grid, dimension=i)
        y, x = np.unravel_index(np.argmax(squares, axis=None), squares.shape)
        value = squares[y, x]
        yield value, x, y, i


largest_square = max(generate_squares(grid), key=lambda x: x[0])

print(f"Part Two : {largest_square}")
