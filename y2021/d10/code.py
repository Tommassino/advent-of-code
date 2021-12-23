# Advent of code Year 2021 Day 10 solution
# Author = witzatom
# Date = December 2021


def run(puzzle_input):
    puzzle_input = puzzle_input.split("\n")

    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


def part_one(puzzle_input):
    return sum(validate(line) for line in puzzle_input)


def part_two(puzzle_input):
    autocomplete_scores = sorted(
        list(filter(lambda x: x > 0, (complete(line) for line in puzzle_input)))
    )
    return autocomplete_scores[len(autocomplete_scores) // 2]


opening = set("([{<")
pairs = {"(": ")", "[": "]", "{": "}", "<": ">"}
invalid_points = {")": 3, "]": 57, "}": 1197, ">": 25137}
incomplete_points = {"(": 1, "[": 2, "{": 3, "<": 4}


def validate(line):
    stack = []
    for c in line:
        if c in opening:
            stack.append(c)
        else:
            matching = stack.pop()
            if pairs[matching] != c:
                return invalid_points[c]
    return 0


def complete(line):
    stack = []
    for c in line:
        if c in opening:
            stack.append(c)
        else:
            matching = stack.pop()
            if pairs[matching] != c:
                return 0
    score = 0
    for c in reversed(stack):
        score *= 5
        score += incomplete_points[c]
    return score
