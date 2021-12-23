# Advent of code Year 2021 Day 3 solution
# Author = witzatom
# Date = December 2021
from collections import Counter
from itertools import count


def run(puzzle_input):
    puzzle_input = puzzle_input.split("\n")
    bit_counters = [
        Counter(number[bit_position] for number in puzzle_input)
        for bit_position in range(len(puzzle_input[0]))
    ]
    gamma = int(
        "".join(
            max(bit_counter.items(), key=lambda x: x[1])[0]
            for bit_counter in bit_counters
        ),
        2,
    )
    epsilon = int(
        "".join(
            min(bit_counter.items(), key=lambda x: x[1])[0]
            for bit_counter in bit_counters
        ),
        2,
    )
    power_consumption = gamma * epsilon

    def oxygen_selector(counter):
        if counter["0"] > counter["1"]:
            return "0"
        return "1"

    def scrubber_selector(counter):
        if counter["1"] < counter["0"]:
            return "1"
        return "0"

    oxygen_rating = filter_values(puzzle_input, oxygen_selector)
    scrubber_rating = filter_values(puzzle_input, scrubber_selector)
    life_support_rating = oxygen_rating * scrubber_rating

    print(f"Part One : {power_consumption}")
    print(f"Part Two : {life_support_rating}")


def filter_values(bit_numbers, selector):
    bit_index = 0
    numbers_left = range(len(bit_numbers))
    while len(numbers_left) > 1:
        bit_counter = Counter(bit_numbers[idx][bit_index] for idx in numbers_left)
        filter_bit = selector(bit_counter)
        numbers_left = list(
            idx for idx in numbers_left if bit_numbers[idx][bit_index] == filter_bit
        )
        bit_index += 1
    result_number = numbers_left[0]
    return int(bit_numbers[result_number], 2)
