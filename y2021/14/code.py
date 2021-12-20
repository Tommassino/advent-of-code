# Advent of code Year 2021 Day 14 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from collections import Counter
from copy import copy
from typing import Dict, Tuple
from collections import defaultdict
from y2021.utils import timed


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


def parse(puzzle_input) -> Tuple[PolymerChain, Dict[str, str]]:
    chain, rules = puzzle_input.split("\n\n")
    polymer_chain = PolymerChain.parse(chain)
    rule_map = dict(rule.split(" -> ") for rule in rules.split("\n"))
    return polymer_chain, rule_map


@timed
def part_one(puzzle_input):
    polymer_chain, rule_map = parse(puzzle_input)
    for i in range(10):
        polymer_chain.apply(rule_map)
    counts = polymer_chain.element_counter
    min_element = min(counts.items(), key=lambda x: x[1])
    max_element = max(counts.items(), key=lambda x: x[1])
    return max_element[1] - min_element[1]


@timed
def part_two(puzzle_input):
    polymer_chain, rule_map = parse(puzzle_input)
    for i in range(40):
        polymer_chain.apply(rule_map)
    counts = polymer_chain.element_counter
    min_element = min(counts.items(), key=lambda x: x[1])
    max_element = max(counts.items(), key=lambda x: x[1])
    return max_element[1] - min_element[1]


class PolymerChain:
    pair_counter: Dict[str, int]
    element_counter: Dict[str, int]

    def __init__(self, pairs, elements) -> None:
        self.pair_counter = defaultdict(int, pairs)
        self.element_counter = defaultdict(int, elements)

    @staticmethod
    def parse(chain: str):
        pair_counter = Counter(chain[i : i + 2] for i in range(len(chain) - 1))
        return PolymerChain(pair_counter, Counter(chain))

    def apply(self, rules: Dict[str, str]):
        new_pair_counter = copy(self.pair_counter)
        for pair, insertion in rules.items():
            if pair in self.pair_counter:
                count = self.pair_counter[pair]
                # print(f"{pair} -= {count}")
                new_pair_counter[pair] -= count
                for new_pair in [f"{pair[0]}{insertion}", f"{insertion}{pair[1]}"]:
                    # print(f"{new_pair} += {count}")
                    new_pair_counter[new_pair] += count
                self.element_counter[insertion] += count
        self.pair_counter = new_pair_counter
