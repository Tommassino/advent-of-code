# Advent of code Year 2021 Day 21 solution
# Author = witzatom
# Date = December 2021

from collections import Counter
from functools import lru_cache
from itertools import product
from typing import Tuple
from y2021.utils import timed


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


def play(position_a: int, position_b: int) -> Tuple[Tuple[int, int], int]:
    """
    >>> play(4, 8)
    ([1000, 745], 993)
    """
    positions = [position_a - 1, position_b - 1]
    scores = [0, 0]
    die_state = 0
    player_turn = 0
    while max(scores) < 1000:
        roll = 3 * die_state + 6
        die_state += 3
        positions[player_turn] = (positions[player_turn] + roll) % 10
        scores[player_turn] += positions[player_turn] + 1
        player_turn = not player_turn
    return scores, die_state


@timed
def part_one(puzzle_input: str) -> int:
    positions = list(
        int(line.split("position: ")[1]) for line in puzzle_input.splitlines()
    )
    scores, die_state = play(*positions)
    return min(scores) * die_state


dimensional_dice = Counter(sum(rolls) for rolls in product([1, 2, 3], repeat=3))


@lru_cache(maxsize=100 * 400)
def count_wins(
    position_a: int, score_a: int, position_b: int, score_b: int, max_score: int = 21
) -> Tuple[int, int]:
    """
    >>> count_wins(
    ...     3, 0,
    ...     7, 0,
    ...     max_score=21
    ... )
    (444356092776315, 341960390180808)
    """
    wins_a, wins_b = 0, 0
    for roll, split_counts in dimensional_dice.items():
        next_pos_a = (position_a + roll) % 10
        next_score_a = score_a + next_pos_a + 1

        if next_score_a >= max_score:
            wins_a += split_counts
        else:
            recursive_b_wins, recursive_a_wins = count_wins(
                position_b, score_b, next_pos_a, next_score_a, max_score=max_score
            )
            wins_a += recursive_a_wins * split_counts
            wins_b += recursive_b_wins * split_counts
    return wins_a, wins_b


@timed
def part_two(puzzle_input: str) -> int:
    positions = tuple(
        int(line.split("position: ")[1]) - 1 for line in puzzle_input.splitlines()
    )
    return max(count_wins(positions[0], 0, positions[1], 0))
