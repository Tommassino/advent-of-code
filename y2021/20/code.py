# Advent of code Year 2021 Day 20 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from y2021.utils import timed


def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


class Image:
    def __init__(self, pixel_coordinates, pixels_lit) -> None:
        self.coordinates = pixel_coordinates
        self.polarity = pixels_lit

    def __repr__(self) -> str:
        min_x, min_y, max_x, max_y = self.bounds()
        pixels = ("#" if self.polarity else ".", "." if self.polarity else "#")
        return "\n".join(
            [
                "".join(
                    [
                        pixels[0] if (x, y) in self.coordinates else pixels[1]
                        for x in range(min_x - 1, max_x + 2)
                    ]
                )
                for y in range(min_y - 1, max_y + 2)
            ]
        )

    def bounds(self):
        min_x, _ = min(self.coordinates, key=lambda x: x[0])
        max_x, _ = max(self.coordinates, key=lambda x: x[0])
        _, min_y = min(self.coordinates, key=lambda x: x[1])
        _, max_y = max(self.coordinates, key=lambda x: x[1])
        return min_x, min_y, max_x, max_y

    def _enhance_index(self, px, py):
        out = 0
        for oy in [-1, 0, 1]:
            for ox in [-1, 0, 1]:
                bit = (
                    self.polarity
                    if (ox + px, oy + py) in self.coordinates
                    else not self.polarity
                )
                out = (out << 1) | bit
        return out

    def enhance(self, algorithm) -> Image:
        min_x, min_y, max_x, max_y = self.bounds()
        target_polarity = not self.polarity if algorithm[0] else self.polarity
        new_pixels = {
            (x, y)
            for x in range(min_x - 1, max_x + 2)
            for y in range(min_y - 1, max_y + 2)
            if algorithm[self._enhance_index(x, y)] == target_polarity
        }
        return Image(new_pixels, target_polarity)

    @staticmethod
    def from_str(input: str) -> Image:
        return Image(
            {
                (x, y)
                for y, line in enumerate(input.splitlines())
                for x, c in enumerate(line)
                if c == "#"
            },
            True,
        )


@timed
def part_one(puzzle_input: str):
    algorithm, image = puzzle_input.split("\n\n")
    algorithm = list(True if c == "#" else False for c in algorithm)
    image = Image.from_str(image)
    image = image.enhance(algorithm)
    image = image.enhance(algorithm)
    # print(image)
    return len(image.coordinates)


@timed
def part_two(puzzle_input: str):
    algorithm, image = puzzle_input.split("\n\n")
    algorithm = list(True if c == "#" else False for c in algorithm)
    image = Image.from_str(image)
    for i in range(50):
        image = image.enhance(algorithm)
    # print(image)
    return len(image.coordinates)
