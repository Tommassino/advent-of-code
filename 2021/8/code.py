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

def resolve_encoding(signals):
    # mapping from digit => signal (encoding)
    encoding = {}
    # mapping from signal idx => possible digits
    assignments = {
        i: set(range(10))
        for i in range(10)
    }
    def assign_encoding():
        to_delete = []
        for idx, digits in assignments.items():
            if len(digits) == 1:
                digit = next(iter(digits))
                encoding[digit] = signals[idx]
                to_delete.append(idx)
                # print(f"resolved {digit} ({idx}) = {encoding[digit]}")
        for idx in to_delete:
            del assignments[idx]
        for idx in assignments:
            assignments[idx].difference(to_delete)
    
    # resolve based on length
    for i in assignments:
        length = len(signals[i])
        assignments[i] = assignments[i].intersection(
            x
            for x in digits
            if length == len(digits[x])
        )
    assign_encoding()

    def is_subset(digit, other_digit):
        return digits[digit] <= digits[other_digit]
    
    for idx in assignments:
        # for each candidate
        # for each know digit
        # if the known digit is a subset of the candidate
        # the encodings have to be subsets too 
        assignments[idx] = {
            candidate_digit
            for candidate_digit in assignments[idx]
            if all(
                digit_encoding <= signals[idx]
                for known_digit, digit_encoding in encoding.items()
                if is_subset(known_digit, candidate_digit)
            )
        }
    assign_encoding()

    for idx in assignments:
        # for each candidate
        # for each know digit
        # if the known digit is not a subset of the candidate
        # the encodings have to not be subsets too 
        assignments[idx] = {
            candidate_digit
            for candidate_digit in assignments[idx]
            if all(
                not digit_encoding <= signals[idx]
                for known_digit, digit_encoding in encoding.items()
                if not is_subset(known_digit, candidate_digit)
            )
        }
    assign_encoding()

    # 2 and 5 will never be resolved using above rules...
    # resolve 5 as a intersection
    matching_encoding = encoding[6].intersection(encoding[9])
    matching_idx = next(
        idx
        for idx, idx_encoding in enumerate(signals)
        if matching_encoding == idx_encoding
    )
    assignments[matching_idx] = {5}
    for idx in assignments:
        if idx != matching_idx:
            assignments[idx].discard(5)
    assign_encoding()

    if len(assignments) > 0:
        raise ValueError()
    return encoding
