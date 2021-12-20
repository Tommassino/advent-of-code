# Advent of code Year 2020 Day 5 solution
# Author = witzatom
# Date = December 2020
from y2021.utils import window


def run(puzzle_input: str):
    def read_seat(line: str):
        row_spec = line[:7]
        row = int(row_spec.replace("B", "1").replace("F", "0"), 2)
        column_spec = line[7:]
        column = int(column_spec.replace("R", "1").replace("L", "0"), 2)
        seat_id = row * 8 + column
        return row, column, seat_id

    seat_specs = [
        read_seat(line)
        for line in puzzle_input.split("\n")
    ]

    _, _, max_id = max(seat_specs, key=lambda x: x[2])

    def find_my_id(seat_specs):
        seat_ids = [
            seat[2]
            for seat in seat_specs
        ]

        for group in window(sorted(seat_ids)):
            if group[1] != group[0] + 1:
                return group[0] + 1

    print(f"Part One : {max_id}")
    print(f"Part Two : {find_my_id(seat_specs)}")
