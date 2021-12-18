# Advent of code Year 2020 Day 16 solution
# Author = witzatom
# Date = December 2020
from typing import NamedTuple, Tuple
from collections import defaultdict
import numpy as np


class Rule(NamedTuple):
    name: str
    first_range: Tuple[int, int]
    second_range: Tuple[int, int]

    def passes(self, number):
        return (self.first_range[0] <= number <= self.first_range[1]) or (
                self.second_range[0] <= number <= self.second_range[1])

    @staticmethod
    def from_string(string: str):
        name, rest = tuple(string.split(": "))
        first_range, second_range = tuple(rest.split(" or "))
        first_range = tuple(int(x) for x in first_range.split("-"))
        second_range = tuple(int(x) for x in second_range.split("-"))
        return Rule(name, first_range, second_range)


def run(puzzle_input):
    rules, my_ticket, tickets = parse_input(puzzle_input)
    print(f"Part One : {part1(rules, tickets)}")
    print(f"Part Two : {part2(rules, my_ticket, tickets)}")


def parse_input(puzzle_input: str):
    rules, my_ticket, nearby_tickets = puzzle_input.split("\n\n")
    rules = [
        Rule.from_string(line)
        for line in rules.split("\n")
    ]
    my_ticket = [
        int(x) for x in my_ticket.split("\n")[1].split(",")
    ]
    nearby_tickets = [
        [
            int(x)
            for x in ticket.split(",")
        ]
        for ticket in nearby_tickets.split("\n")[1:]
    ]
    return rules, my_ticket, nearby_tickets


def part1(rules, nearby_tickets):
    invalid_values = []
    for ticket in nearby_tickets:
        for field in ticket:
            if not any(rule.passes(field) for rule in rules):
                invalid_values.append(field)
    return sum(invalid_values)


def part2(rules, my_ticket, nearby_tickets):
    valid_tickets = []
    for ticket in nearby_tickets:
        is_valid = all(
            any(
                rule.passes(field)
                for rule in rules
            )
            for field in ticket
        )
        if is_valid:
            valid_tickets.append(ticket)

    rule_candidates = defaultdict(set)
    for field in range(len(my_ticket)):
        for rule in rules:
            is_valid = all(
                rule.passes(ticket[field])
                for ticket in valid_tickets
            )
            if is_valid:
                rule_candidates[rule].add(field)

    field_assignments = {}
    while len(rule_candidates) > 0:
        rule, field = next(filter(lambda x: len(x[1]) == 1, rule_candidates.items()))
        field = list(field)[0]
        field_assignments[rule.name] = field
        del rule_candidates[rule]
        for x in rule_candidates:
            rule_candidates[x].remove(field)

    departure_values = [
        my_ticket[field]
        for rule, field in field_assignments.items()
        if rule.startswith("departure")
    ]
    return np.product(departure_values, dtype=np.int64)
