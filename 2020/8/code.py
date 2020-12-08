# Advent of code Year 2020 Day 8 solution
# Author = witzatom
# Date = December 2020

from __future__ import annotations

from enum import Enum
from itertools import count
from typing import List
from typing import NamedTuple


def run(puzzle_input):
    program = [
        Instruction.from_string(line)
        for line in puzzle_input.split("\n")
    ]

    print(f"Part One : {part1(program)}")
    print(f"Part Two : {part2(program)}")


class OpCode(Enum):
    NoOP = 'nop'
    Accumulate = 'acc'
    Jump = 'jmp'


class Instruction(NamedTuple):
    opcode: OpCode
    argument: int

    @staticmethod
    def from_string(string: str) -> Instruction:
        opcode, argument = tuple(string.split(" "))
        return Instruction(OpCode(opcode), int(argument))

    def execute(self, memory: int, program_counter: int):
        if self.opcode == OpCode.Accumulate:
            return memory + self.argument, program_counter + 1
        elif self.opcode == OpCode.Jump:
            return memory, program_counter + self.argument
        elif self.opcode == OpCode.NoOP:
            return memory, program_counter + 1
        else:
            raise NotImplementedError(self.opcode)


def iter_program(program: List[Instruction], memory=0, program_counter=0):
    for step in count(0):
        memory, program_counter = program[program_counter].execute(memory, program_counter)
        yield step, memory, program_counter
        if program_counter == len(program):
            return


def find_loop(program: List[Instruction]):
    seen_pcs = {0}
    accumulator = 0
    for step, memory, program_counter in iter_program(program):
        if program_counter in seen_pcs:
            return True, accumulator
        accumulator = memory
        seen_pcs.add(program_counter)
    return False, accumulator


def part1(program: List[Instruction]):
    _, accumulator = find_loop(program)
    return accumulator


def part2(program: List[Instruction]):
    replacements = [
        idx
        for idx, op in enumerate(program)
        if op.opcode == OpCode.NoOP or op.opcode == OpCode.Jump
    ]
    for idx in replacements:
        modified_program = program.copy()
        if modified_program[idx].opcode == OpCode.NoOP:
            new_instruction = Instruction(OpCode.Jump, modified_program[idx].argument)
        else:
            new_instruction = Instruction(OpCode.NoOP, modified_program[idx].argument)
        del modified_program[idx]
        modified_program.insert(idx, new_instruction)
        loops, accumulator = find_loop(modified_program)
        if not loops:
            return accumulator
