# Advent of code Year 2021 Day 8 solution
# Author = witzatom
# Date = December 2021

def run(puzzle_input):
    puzzle_input = parse_input(puzzle_input)
    
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")

def parse_input(puzzle_input):
    return [
        parse_line(line)
        for line in puzzle_input.split("\n")
    ]

def parse_line(line):
    unique_signals, output = line.split(" | ")
    unique_signals = list(map(set, unique_signals.split(" ")))
    output = list(map(set, output.split(" ")))
    return unique_signals, output

def part_one(puzzle_input):
    unique_lengths = {2, 3, 4, 7}
    unique_digits = [
        digit
        for _, output_signals in puzzle_input
        for digit in output_signals
        if len(digit) in unique_lengths
    ]
    return len(unique_digits)

def part_two(puzzle_input):
    return sum(
        resolve_output(unique, output)
        for unique, output in puzzle_input
    )

def resolve_output(unique_signals, output):
    encoding = resolve_encoding(unique_signals)
    
    result = sum(
        next(filter(
            lambda x: x[1] == encoded_digit,
            encoding.items()
        ))[0] * (10 ** (len(output) - idx - 1))
        for idx, encoded_digit in enumerate(output)
    )
    return result

#   0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....

#   5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg

digits = {
    0: set('abcefg'),
    1: set('cf'),
    2: set('acdeg'),
    3: set('acdfg'),
    4: set('bcdf'),
    5: set('abdfg'),
    6: set('abdefg'),
    7: set('acf'),
    8: set('abcdefg'),
    9: set('abcdfg')
}

easy_digits = (1, 4, 7, 8)

def signal_characteristics(signals, simple_signals):
    return [
        tuple(
            len(signal.intersection(simple_signal))
            for simple_signal in simple_signals
        )
        for signal in signals
    ]

default_characteristics = dict(zip(
    signal_characteristics(
        digits.values(),
        [digits[digit] for digit in easy_digits]
    ),
    digits.keys()
))

def resolve_encoding(signals):
    easy_signals = [
        next(
            signal
            for signal in signals
            if len(signal) == len(digits[digit])
        )
        for digit in easy_digits
    ]
    characteristics = signal_characteristics(signals, easy_signals)
    return {
        default_characteristics[characteristic]: signal
        for characteristic, signal in zip(characteristics, signals)
    }
