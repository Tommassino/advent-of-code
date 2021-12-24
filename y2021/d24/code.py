# Advent of code Year 2021 Day 24 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from collections import defaultdict
from os import stat
from typing import DefaultDict, Dict, List, NamedTuple, Tuple, Union
from itertools import product
from y2021.utils import timed
from functools import lru_cache

def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")

class Instruction(NamedTuple):
    operation: str
    target: str
    operand: Union[str, int]

    @staticmethod
    def parse(input: str) -> Instruction:
        operation, target, operand = input.split(" ")
        if operand.isdigit() or (operand[0] == '-' and operand[1:].isdigit()):
            operand = int(operand)
        return Instruction(operation, target, operand)

class Memory(NamedTuple):
    data: Tuple[int, int, int, int]

    @staticmethod
    def idx(key):
        if key == "w":
            return 0
        elif key == "x":
            return 1
        elif key == "y":
            return 2
        elif key == "z":
            return 3
        else:
            raise ValueError(key)

    def input(self, input, input_to):
        data = list(self.data)
        data[self.idx(input_to)] = input
        return Memory(tuple(data))

    def apply(self, instruction: Instruction):
        data = list(self.data)
        a = data[self.idx(instruction.target)]
        b = data[self.idx(instruction.operand)] if isinstance(instruction.operand, str) else instruction.operand
        if instruction.operation == "add":
            data[self.idx(instruction.target)] = a + b
        elif instruction.operation == "mul":
            data[self.idx(instruction.target)] = a * b
        elif instruction.operation == "div":
            data[self.idx(instruction.target)] = a // b
        elif instruction.operation == "mod":
            data[self.idx(instruction.target)] = a % b
        elif instruction.operation == "eql":
            data[self.idx(instruction.target)] = a == b
        else:
            raise ValueError(instruction)
        return Memory(tuple(data))

negate_example = """inp x
mul x -1"""

example = """inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"""

class Program:
    def __init__(self, parts: List[Tuple[str, Tuple[Instruction]]]) -> None:
        self.parts = parts

    def run(self, inputs: List[int]) -> Memory:
        """
        >>> Program.parse(negate_example).run([5])
        Memory(data=(0, -5, 0, 0))

        >>> Program.parse(example).run([9])
        Memory(data=(1, 0, 0, 1))
        """
        memory = Memory((0, 0, 0, 0))
        for input, (input_to, chunk) in zip(inputs, self.parts):
            memory = memory.input(input, input_to)
            memory = self._run_chunk(memory, chunk)
        return memory

    @lru_cache(None)
    def _run_chunk(self, memory: Memory, chunk: Tuple[Instruction]) -> Memory:
        for instruction in chunk:
            memory = memory.apply(instruction)
        return memory

    @staticmethod
    def parse(puzzle_input: str) -> Program:
        program = []
        for input in puzzle_input.split("inp "):
            lines = input.splitlines()
            if not len(lines):
                continue
            input_variable = lines[0]
            part = tuple(
                Instruction.parse(line)
                for line in lines[1:]
            )
            program.append((input_variable, part))
        reference = program[0][1]
        for _, chunk in program[1:]:
            for idx, line in enumerate(chunk):
                if reference[idx] != line:
                    print(f"Diff at {idx}: {reference[idx]} != {line}")
        return Program(program)

"""
inp w
mul x 0
add x z
mod x 26

div z a
add x b

eql x w
eql x 0

mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y c
mul y x
add z y

w = set
x = z % 26 + b
z = z // a
x = (x != w)
y = 25 * x + 1
z = z * y
y = (w + c) * x
z = z + y

Diff at 14: Instruction(operation='add', target='y', operand=16) != Instruction(operation='add', target='y', operand=7)
Diff at 3: Instruction(operation='div', target='z', operand=1) != Instruction(operation='div', target='z', operand=26)
Diff at 4: Instruction(operation='add', target='x', operand=14) != Instruction(operation='add', target='x', operand=-10)

"""


@lru_cache(None)
def run_chunk(w: int, x: int, y: int, z: int, a: int, b: int, c: int) -> Tuple[int, int, int, int]:
    """
    Test based on the general implementation

    >>> run_chunk(0, 0, 0, 0, 1, 14, 16)
    (0, True, 16, 16)
    
    >>> run_chunk(10, 0, 0, 0, 1, 14, 16)
    (10, True, 26, 26)
    
    >>> run_chunk(10, 0, 0, 100, 1, 14, 16)
    (10, True, 26, 2626)
    """
    x = z % 26 + b
    z = z // a
    x = (x != w)
    y = 25 * x + 1
    z = z * y
    y = (w + c) * x
    z = z + y
    return (w, x, y, z)

def run_program(program: Program, input: List[int]):
    w, x, y, z = 0, 0, 0, 0
    for wset, (_, chunk) in zip(input, program.parts):
        a, b, c = chunk[3].operand, chunk[4].operand, chunk[14].operand
        w = wset
        w, x, y, z = run_chunk(w, x, y, z, a, b, c)
    return w, x, y, z

from tqdm.auto import tqdm
def part_one(puzzle_input: str):
    program = Program.parse(puzzle_input)
    maximum_valid = 0
    for input in tqdm(product([1,2,3,4,5,6,7,8,9], repeat=14), total=9**14):
        result = run_program(program, input)
        if result[3] == 0:
            number = int("".join(map(str, input)))
            if number > maximum_valid:
                print(number)
                maximum_valid = number


def part_two(puzzle_input: str):
    pass
