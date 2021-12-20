# Advent of code Year 2020 Day 18 solution
# Author = witzatom
# Date = December 2020
from __future__ import annotations

import re
from enum import Enum
from typing import Union, Tuple, List


def run(puzzle_input):
    print(f"Part One : {part1(puzzle_input)}")
    print(f"Part Two : {part2(puzzle_input)}")


def part1(puzzle_input):
    expressions = [
        [
            x.strip()
            for x in re.split(r"([\s()])", expression)
            if x.strip() != ""
        ]
        for expression in puzzle_input.split("\n")
    ]
    return sum(evaluate_expression(x) for x in expressions)


def part2(puzzle_input):
    return sum(
        Expression.parse(expression).evaluate()
        for expression in puzzle_input.split("\n")
    )


def evaluate_expression(expression: List[str]) -> int:
    accumulator = 0
    operation = '+'
    while len(expression) > 0:
        value = expression.pop(0)
        if value in {'+', '*'}:
            operation = value
            continue
        if value == ')':
            return accumulator

        if value == '(':
            value = evaluate_expression(expression)
        else:
            value = int(value)

        # print(f"{accumulator} {operation} {value}")
        if operation == '+':
            accumulator += value
        elif operation == '*':
            accumulator *= value
        else:
            raise ValueError("no operation")
        operation = None
    return accumulator


class Expression:
    expression: List[Tuple[Operation, Union[int, Expression]]]

    def __init__(self, expression: List[Tuple[Operation, Union[int, Expression]]]):
        self.expression = expression

    def __repr__(self):
        return "(" + " ".join(str(x) for x in self.expression) + ")"

    def evaluate(self):
        def propagated(x):
            if isinstance(x, Expression):
                return x.evaluate()
            else:
                return x

        product = 1
        accumulator = 0
        for operation, value in self.expression:
            if operation == Operation.PLUS:
                accumulator += propagated(value)
            elif operation == Operation.TIMES:
                product *= accumulator
                accumulator = propagated(value)
        return accumulator * product

    @classmethod
    def parse(cls, expression):
        return cls._parse([
            x.strip()
            for x in re.split(r"([\s()])", expression)
            if x.strip() != ''
        ])

    @classmethod
    def _parse(cls, parts):
        expression = []
        operation = Operation.PLUS
        while len(parts) > 0:
            token = parts.pop(0)
            if token == '(':
                expression.append((operation, cls._parse(parts)))
            elif token == ')':
                return cls(expression)
            elif token.isdigit():
                expression.append((operation, int(token)))
            else:
                operation = Operation(token)
        return cls(expression)


class Operation(Enum):
    PLUS = '+'
    TIMES = '*'
