# Advent of code Year 2020 Day 13 solution
# Author = witzatom
# Date = December 2020
from utils import egcd


def run(puzzle_input):
    puzzle_input = puzzle_input.split("\n")
    timestamp = int(puzzle_input[0])
    bus_lines = [
        int(bus)
        for bus in puzzle_input[1].split(",")
        if bus != "x"
    ]
    bus_ids = [
        (int(bus), idx)
        for idx, bus in enumerate(puzzle_input[1].split(","))
        if bus != "x"
    ]
    print(f"Part One : {part1(timestamp, bus_lines)}")
    print(f"Part Two : {part2(bus_ids)}")


def part1(timestamp, bus_lines):
    def waiting_time(x):
        return x * round(timestamp / x + 0.5) - timestamp

    best_line = min(bus_lines, key=waiting_time)
    return waiting_time(best_line) * best_line


def part2(bus_ids):
    period, phase = (1, 0)
    for next_period, next_phase in bus_ids:
        period, phase = combine_periods(period, phase, next_period, next_phase)
    return -phase % period


def combine_periods(period_a, phase_a, period_b, phase_b):
    g, s, t = egcd(period_a, period_b)
    multiplier, remainder = divmod(phase_a - phase_b, g)
    if remainder:
        raise ValueError("Cannot align phases of input")
    combined_period = period_a // g * period_b
    combined_phase = (phase_a - s * multiplier * period_a) % combined_period
    return combined_period, combined_phase
