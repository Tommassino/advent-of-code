# Advent of code Year 2020 Day 25 solution
# Author = witzatom
# Date = December 2020

MODULO = 20201227


def run(puzzle_input):
    card_public_key, door_public_key = tuple(int(x) for x in puzzle_input.split("\n"))
    print(f"Part One : {part1(card_public_key, door_public_key)}")
    print(f"Part Two : YOU WIN")


def find_loop_size(public_key):
    exponent = 0
    n = 1
    while n != public_key:
        exponent += 1
        n = 7 * n % MODULO
    return exponent


def part1(card_public_key, door_public_key):
    exponent = find_loop_size(door_public_key)
    return pow(card_public_key, exponent, MODULO)
