# Advent of code Year 2020 Day 2 solution
# Author = witzatom
# Date = December 2020

from dataclasses import dataclass

from y2021.utils import re_group_parser


@dataclass
class Rule:
    min_count: int
    max_count: int
    char: str
    password: str


def run(input):
    parse = re_group_parser(r"^(\d+)-(\d+) (.): (.*)$", Rule)

    passwords = [
        parse(line)
        for line in input.split("\n")
    ]

    def is_valid(rule: Rule):
        return rule.min_count <= rule.password.count(rule.char) <= rule.max_count

    valid_passwords = list(filter(is_valid, passwords))
    print(f"Part One : {len(valid_passwords)}")

    def is_valid_real(rule: Rule):
        related = (rule.password[rule.min_count - 1], rule.password[rule.max_count - 1])
        return rule.char in related and (len(set(related)) != 1)

    really_valid_passwords = list(filter(is_valid_real, passwords))
    print(f"Part Two : {len(really_valid_passwords)}")
