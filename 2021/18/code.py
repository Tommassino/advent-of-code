# Advent of code Year 2021 Day 18 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from functools import reduce
from typing import List, Tuple, Union, Optional
import math
from utils import timed
from itertools import permutations


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


SnailNumber = Union[int, List["SnailNumber"]]


def add_left(number: SnailNumber, n: int) -> SnailNumber:
    """
    Adds n to the leftmost literal in numbers subtree
    """
    if n is None:
        return number
    if isinstance(number, int):
        return number + n
    return [add_left(number[0], n), number[1]]


def add_right(number: SnailNumber, n: int) -> SnailNumber:
    """
    Adds n to the rightmost literal in numbers subtree
    """
    if n is None:
        return number
    if isinstance(number, int):
        return number + n
    return [number[0], add_right(number[1], n)]


def explode(
    number: SnailNumber, depth: int = 0
) -> Tuple[bool, Optional[int], SnailNumber, Optional[int]]:
    """
    Explodes number
    returns (exploded, what to add to left, number, what to add to right)
    """
    if isinstance(number, int):
        return False, None, number, None
    left_part, right_part = number
    if depth >= 4:
        return True, left_part, 0, right_part
    # explode left number
    exploded, left_add, left_part, right_add = explode(left_part, depth + 1)
    if exploded:
        return True, left_add, [left_part, add_left(right_part, right_add)], None
    # explode right number
    exploded, left_add, right_part, right_add = explode(right_part, depth + 1)
    if exploded:
        return True, None, [add_right(left_part, left_add), right_part], right_add
    return False, None, number, None


def split(number: SnailNumber, has_split: bool = False) -> SnailNumber:
    if isinstance(number, list):
        for idx in range(len(number)):
            replacement, has_split = split(number[idx], has_split)
            number[idx] = replacement
        return number, has_split
    else:
        if number >= 10 and not has_split:
            number = [(math.floor(number / 2.0)), (math.ceil(number / 2.0))]
            has_split = True
        return number, has_split


def add(a: SnailNumber, b: SnailNumber) -> SnailNumber:
    number = [a, b]
    reduced = True
    while reduced:
        reduced, _, number, _ = explode(number)
        if not reduced:
            number, reduced = split(number)
    return number


def magnitude(number: SnailNumber) -> int:
    if isinstance(number, list):
        return 3 * magnitude(number[0]) + 2 * magnitude(number[1])
    else:
        return number


@timed
def part_one(puzzle_input: str) -> int:
    numbers = list(map(eval, puzzle_input.splitlines()))
    return magnitude(reduce(add, numbers))


@timed
def part_two(puzzle_input: str) -> int:
    numbers = list(map(eval, puzzle_input.splitlines()))
    return max(magnitude(add(a, b)) for a, b in permutations(numbers, 2))
