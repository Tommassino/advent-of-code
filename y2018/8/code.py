# Advent of code Year 2018 Day 8 solution
# Author = witzatom
# Date = December 2018

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


def read_metadata(data):
    children_count = data.pop(0)
    metadata_count = data.pop(0)
    for _ in range(children_count):
        yield from read_metadata(data)
    for _ in range(metadata_count):
        yield data.pop(0)


data = [
    int(x)
    for x in input.split(" ")
]

print("Part One : " + str(sum(read_metadata(data))))

data = [
    int(x)
    for x in input.split(" ")
]


def read_value(data):
    children_count = data.pop(0)
    metadata_count = data.pop(0)
    if children_count == 0:
        return sum(data.pop(0) for _ in range(metadata_count))
    else:
        child_values = {
            idx + 1: read_value(data)
            for idx in range(children_count)
        }
        return sum(
            child_values.get(data.pop(0), 0)
            for _ in range(metadata_count)
        )


print("Part Two : " + str(read_value(data)))
