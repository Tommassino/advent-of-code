# Advent of code Year 2021 Day 6 solution
# Author = witzatom
# Date = December 2021
from collections import Counter

def run(puzzle_input):
    timers = list(map(int, puzzle_input.split(",")))
    total_fish = sum(day_n(80, timers).values())
    forever_fish = sum(day_n(256, timers).values())

    print(f"Part One : {total_fish}")
    print(f"Part Two : {forever_fish}")

def day_n(n, starting_timers):
    spawn_timer = Counter(starting_timers)
    for day in range(n):
        spawn_timer[day + 7] += spawn_timer[day]
        spawn_timer[day + 9] += spawn_timer[day]
        spawn_timer[day] = 0
    return spawn_timer
