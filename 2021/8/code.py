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
    encoding = {
        i: segments_for(unique_signals, i)
        for i in range(10)
    }
    
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

digit_lengths = {
    digit: len(digits[digit])
    for digit in range(10)
}
subset_digits = dict(filter(lambda x: len(x[1]) > 0, {
    digit: [
        other_digit
        for other_digit in [1, 4, 7, 8] # only consider resolved digits
        if digit != other_digit and digits[other_digit] <= digits[digit]
    ]
    for digit in range(10)
}.items()))

not_subset_digits = {
    0: [4]
}
not_digits = {
    6: [0, 9],
    2: [3, 5]
}

def segments_for(unique_signals, digit):
    def predicate_for(x):
        if len(x) != digit_lengths[digit]:
            return False
        if digit in subset_digits:
            for subset in subset_digits[digit]:
                subset_segment = segments_for(unique_signals, subset)
                if not subset_segment <= x:
                    return False
        if digit in not_subset_digits:
            for not_subset in not_subset_digits[digit]:
                not_subset_segment = segments_for(unique_signals, not_subset)
                if not_subset_segment <= x:
                    return False
        if digit in not_digits:
            for not_digit in not_digits[digit]:
                not_segment = segments_for(unique_signals, not_digit)
                if not_segment == x:
                    return False
        return True

    if digit == 5:
        return segments_for(unique_signals, 6).intersection(segments_for(unique_signals, 9))
    filtered = list(filter(predicate_for, unique_signals))
    if len(filtered) > 1:
        raise ValueError()
    return filtered[0]
