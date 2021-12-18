# Advent of code Year 2020 Day 15 solution
# Author = witzatom
# Date = December 2020
from collections import defaultdict
from y2021.utils import timed


def run(puzzle_input):
    numbers = [
        int(i) for i in puzzle_input.split(",")
    ]
    print(f"Part One : {part1(numbers, 2020)}")
    print(f"Part Two : {part1(numbers, 30000000)}")


class Number:
    def __init__(self):
        self.spoken_steps = [None] * 2

    def speak(self, step):
        self.spoken_steps[0] = self.spoken_steps[1]
        self.spoken_steps[1] = step

    def ask(self):
        if self.spoken_steps[0] is None:
            return 0
        else:
            return self.spoken_steps[1] - self.spoken_steps[0]


@timed
def part1(numbers, limit):
    memory = defaultdict(Number)
    last_number = numbers[-1]
    for idx, number in enumerate(numbers):
        memory[number].speak(idx + 1)
    for step in range(len(numbers) + 1, limit + 1):
        previous = memory[last_number]
        to_speak = previous.ask()
        memory[to_speak].speak(step)
        last_number = to_speak
    return last_number
