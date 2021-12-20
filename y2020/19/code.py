# Advent of code Year 2020 Day 19 solution
# Author = witzatom
# Date = December 2020
import regex as re


def run(puzzle_input):
    rule_input, messages = tuple(puzzle_input.split("\n\n"))
    rules = parse_rules(rule_input)
    messages = messages.split()
    print(f"Part One : {part1(messages, rules)}")
    print(f"Part Two : {part2(messages, rules)}")


def parse_rules(rule_input):
    def parse_rule(rule):
        idx, rest = tuple(rule.split(": "))
        idx = int(idx)
        if "\"" in rest:
            return idx, rest.replace("\"", "")
        else:
            patterns = [
                [
                    int(number)
                    for number in pattern.split()
                ]
                for pattern in rest.split(" | ")
            ]
            return idx, patterns

    return dict(
        parse_rule(line)
        for line in rule_input.split("\n")
    )


def part1(messages, rules):
    def propagate_rule(idx):
        patterns = rules[idx]
        if isinstance(patterns, str):
            return patterns
        else:
            re_patterns = []
            for pattern in patterns:
                re_pattern = [
                    propagate_rule(part_id)
                    for part_id in pattern
                ]
                re_patterns.append("".join(re_pattern))
            re_patterns = "|".join(re_patterns)
            return f"({re_patterns})"

    rule = re.compile(propagate_rule(0))
    return sum(
        1 if rule.fullmatch(message) else 0
        for message in messages
    )


def part2(messages, rules):
    def propagate_rule(idx):
        if idx == 8:
            # rule 8 ~ 42+
            return f"({propagate_rule(42)})+"
        if idx == 11:
            # rule 11 ~ 42{n}31{n}, or 42<g>31 where g matches the whole pattern
            first = propagate_rule(42)
            second = propagate_rule(31)
            return f"(?P<rule11>{first}(?P>rule11)?{second})"

        patterns = rules[idx]
        if isinstance(patterns, str):
            return patterns
        else:
            re_patterns = []
            for pattern in patterns:
                re_pattern = [
                    propagate_rule(part_id)
                    for part_id in pattern
                ]
                re_patterns.append("".join(re_pattern))
            re_patterns = "|".join(re_patterns)
            pattern = f"(:?{re_patterns})"
            return pattern

    rule = re.compile(propagate_rule(0))
    return sum(
        1 if rule.fullmatch(message) else 0
        for message in messages
    )
