# Advent of code Year 2021 Day 24 solution
# Author = witzatom
# Date = December 2021

def run(puzzle_input: str):
    instructions = puzzle_input.splitlines()
    stack = []

    max_number, min_number = 99999999999999, 11111111111111

    for i in range(14):
        a = int(instructions[18*i+5].split()[-1])
        b = int(instructions[18*i+15].split()[-1])

        if a > 0: stack+=[(i, b)]; continue
        j, b = stack.pop()

        max_number -= abs((a+b)*10**(13-[i,j][a>-b]))
        min_number += abs((a+b)*10**(13-[i,j][a<-b]))

    print(f"Part One : \n{max_number}")
    print(f"Part Two : \n{min_number}")
