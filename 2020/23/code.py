# Advent of code Year 2020 Day 23 solution
# Author = witzatom
# Date = December 2020
from utils import timed
from itertools import islice


def run(puzzle_input):
    cups = [int(x) for x in puzzle_input]
    print(f"Part One : {part1(cups)}")
    print(f"Part Two : {part2(cups)}")


class CupGame:
    def __init__(self, cups, debug=False):
        self.cup_next = {
            cup: cups[(idx + 1) % len(cups)]
            for idx, cup in enumerate(cups)
        }
        self.current_cup = cups[0]
        self.max_cup = max(cups)
        self.debug = debug
        self.picked_up = [None, None, None]

    def iter_from(self, cup):
        next_cup = self.cup_next[cup]
        while next_cup != cup:
            yield next_cup
            next_cup = self.cup_next[next_cup]

    def log(self, data):
        if self.debug:
            print(data)

    def tick(self):
        # self.log(f"cups: {list(self.iter_from(self.current_cup))}")
        self.picked_up[0] = self.cup_next[self.current_cup]
        self.picked_up[1] = self.cup_next[self.picked_up[0]]
        self.picked_up[2] = self.cup_next[self.picked_up[1]]
        # self.log(f"pick up: {picked_up}")
        destination_cup = self.max_cup if self.current_cup == 1 else self.current_cup - 1
        while destination_cup in self.picked_up:
            destination_cup = self.max_cup if destination_cup == 1 else destination_cup - 1
        # self.log(f"destination: {destination_cup}")
        # pick up cups
        self.cup_next[self.current_cup] = self.cup_next[self.picked_up[-1]]
        # place down picked up cups
        self.cup_next[self.picked_up[-1]] = self.cup_next[destination_cup]
        self.cup_next[destination_cup] = self.picked_up[0]

        # move to the right
        self.current_cup = self.cup_next[self.current_cup]


def part1(cups):
    cup_game = CupGame(cups, debug=False)
    for i in range(100):
        # print(f"-- move {i+1} --")
        cup_game.tick()
    return "".join(
        str(i)
        for i in cup_game.iter_from(1)
    )


@timed
def part2(cups):
    max_cup = max(cups)
    extra_cups = [
        i
        for i in range(max_cup + 1, 1000001)
    ]
    cups = cups + extra_cups
    cup_game = CupGame(cups, debug=False)
    for i in range(10000000):
        cup_game.tick()
    first, second = tuple(islice(cup_game.iter_from(1), 2))
    return first * second
