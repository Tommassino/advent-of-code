# Advent of code Year 2018 Day 9 solution
# Author = witzatom
# Date = December 2018

from __future__ import annotations
import re

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


class Marble:
    def __init__(self, idx: int, previous: Marble, next: Marble):
        self.idx, self.next, self.previous = idx, next, previous

    def __repr__(self):
        return f"{self.previous.idx}, {self.idx}, {self.next.idx}"


def play(marbles: int, players: int, multiple: int = 23, rewind=6):
    scores = [0] * players
    current = Marble(0, None, None)
    current.next = current.previous = current
    marbles_used = 0
    player_idx = 0
    while marbles_used < marbles:
        marbles_used += 1
        if marbles_used % multiple == 0:
            for _ in range(rewind):
                current = current.previous
            scores[player_idx] += marbles_used + current.previous.idx
            current.previous = current.previous.previous
            current.previous.next = current
        else:
            current = Marble(marbles_used, current.next, current.next.next)
            current.previous.next = current.next.previous = current
        player_idx = (player_idx + 1) % players
    return max(scores)


input_match = re.compile(r"(\d+) players; last marble is worth (\d+) points").match(input)
players = int(input_match.group(1))
marbles = int(input_match.group(2))

print("Part One : " + str(play(marbles, players)))

print("Part Two : " + str(play(marbles * 100, players)))
