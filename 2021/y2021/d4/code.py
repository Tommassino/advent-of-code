# Advent of code Year 2021 Day 4 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
import re
from y2021.utils.parsing import lines
from typing import List, Tuple


def run(puzzle_input):
    puzzle_input = lines(puzzle_input)
    moves = [int(x) for x in puzzle_input[0].split(",")]
    puzzle_input = puzzle_input[2:]
    boards = []
    while len(puzzle_input) > 0:
        board, puzzle_input = BingoBoard.parse(puzzle_input)
        boards.append(board)
        puzzle_input = puzzle_input[1:]

    winning_board, winning_move = find_winning_board(moves, boards)
    winning_score = winning_board.score()

    losing_board, losing_move = find_losing_board(moves, boards)
    losing_score = losing_board.score()

    print(f"Part One : {winning_score * winning_move}")
    print(f"Part Two : {losing_score * losing_move}")


class BingoBoard:
    def __init__(self, data):
        self.rows: List[List[int]] = [set(row) for row in data]
        self.columns: List[List[int]] = [
            set(row[idx] for row in data) for idx in range(len(data))
        ]

    def play(self, move: int) -> bool:
        for row in self.rows:
            row.discard(move)
        for column in self.columns:
            column.discard(move)

        for row in self.rows:
            if len(row) == 0:
                return True
        for column in self.columns:
            if len(column) == 0:
                return True
        return False

    def score(self) -> int:
        return sum(x for row in self.rows for x in row)

    def __repr__(self) -> str:
        return "\n".join(["\t".join(str(i) for i in row) for row in self.rows]) + "\n"

    @classmethod
    def parse(cls, lines: List[str]) -> BingoBoard:
        dimension = len(re.split("\s+", lines[0].strip()))
        data = [
            [int(x) for x in re.split("\s+", lines[i].strip())]
            for i in range(dimension)
        ]
        return BingoBoard(data), lines[dimension:]


def find_winning_board(
    moves: List[int], boards: List[BingoBoard]
) -> Tuple[BingoBoard, int]:
    for move in moves:
        for board in boards:
            winning = board.play(move)
            if winning:
                return board, move


def find_losing_board(
    moves: List[int], boards: List[BingoBoard]
) -> Tuple[BingoBoard, int]:
    winning_boards = set()
    for move in moves:
        for idx, board in enumerate(boards):
            if idx in winning_boards:
                continue
            winning = board.play(move)
            if winning:
                winning_boards.add(idx)
                if len(winning_boards) == len(boards):
                    return board, move
