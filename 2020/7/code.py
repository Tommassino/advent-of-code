# Advent of code Year 2020 Day 7 solution
# Author = witzatom
# Date = December 2020
import re


def run(puzzle_input):
    dependency_graph = parse_input(puzzle_input)

    print(f"Part One : {part1(dependency_graph)}")
    print(f"Part Two : {part2(dependency_graph)}")


def parse_input(puzzle_input):
    line_pattern = re.compile(r"(\d+\s)?(\w+ \w+) bags?")
    graph = {}
    for line in puzzle_input.split("\n"):
        colors = line_pattern.findall(line)
        _, bag_color = colors[0]
        contained = {
            (int(count.strip()), color)
            for count, color in colors[1:]
            if color != "no other"
        }
        graph[bag_color] = contained
    return graph


def part1(dependency_graph):
    bag_colors = set()
    bags = ["shiny gold"]
    while len(bags) > 0:
        bag_color = bags.pop(0)
        candidates = [
            bag
            for bag, contents in dependency_graph.items()
            if len(list(filter(lambda x: x[1] == bag_color, contents))) > 0
        ]
        bags.extend(candidates)
        bag_colors.update(candidates)
    return len(bag_colors)


def part2(dependency_graph):
    total_count = 0
    bags = [(1, "shiny gold")]
    while len(bags) > 0:
        bag_count, bag_color = bags.pop(0)
        contents = dependency_graph[bag_color]
        total_count += sum(count * bag_count for count, _ in contents)
        bags.extend((count * bag_count, color) for count, color in contents)
    return total_count
