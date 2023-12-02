# Advent of code Year 2021 Day 16 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from typing import Optional, Tuple, List, Any
from io import StringIO
from functools import reduce


def run(puzzle_input):
    bits = "".join(f"{int(c, 16):04b}" for c in puzzle_input)
    packets = decode(bits)

    print(f"Part One : {part_one(packets)}")
    print(f"Part Two : {part_two(packets)}")


def part_one(packets: List[Packet]) -> int:
    version_sum = sum(packet.version() for packet in packets)
    return version_sum


def part_two(packets: List[Packet]) -> int:
    packet = packets[0]
    print(packet)
    return packet.value()


def decode(bits: str):
    packets = []
    reader = StringIO(bits)
    while reader:
        try:
            packet = Packet.parse(reader)
            packets.append(packet)
        except Exception as e:
            print(e)
            break
    return packets


class Packet:
    _version: int
    type_id: int
    data: Any

    def __init__(self, version, type_id, data) -> None:
        self._version = version
        self.type_id = type_id
        self.data = data

    def __repr__(self) -> str:
        if self.type_id == 0:  # sum
            return f"({' + '.join(str(x) for x in self.data)})"
        elif self.type_id == 1:  # prod
            return f"({' * '.join(str(x) for x in self.data)})"
        elif self.type_id == 2:  # min
            return f"min({', '.join(str(x) for x in self.data)})"
        elif self.type_id == 3:  # max
            return f"max({', '.join(str(x) for x in self.data)})"
        elif self.type_id == 4:  # literal
            return f"{self.data}"
        elif self.type_id == 5:  # greater
            return f"({self.data[0]} > {self.data[1]})"
        elif self.type_id == 6:  # less
            return f"({self.data[0]} < {self.data[1]})"
        elif self.type_id == 7:  # equal
            return f"({self.data[0]} == {self.data[1]})"

    def version(self):
        if self.type_id == 4:
            return self._version
        return self._version + sum(packet.version() for packet in self.data)

    def value(self):
        if self.type_id == 0:  # sum
            return sum(packet.value() for packet in self.data)
        elif self.type_id == 1:  # prod
            return reduce(lambda x, y: x * y, (packet.value() for packet in self.data))
        elif self.type_id == 2:  # min
            return min(packet.value() for packet in self.data)
        elif self.type_id == 3:  # max
            return max(packet.value() for packet in self.data)
        elif self.type_id == 4:  # literal
            return self.data
        elif self.type_id == 5:  # greater
            return int(self.data[0].value() > self.data[1].value())
        elif self.type_id == 6:  # less
            return int(self.data[0].value() < self.data[1].value())
        elif self.type_id == 7:  # equal
            return int(self.data[0].value() == self.data[1].value())
        raise ValueError(self.type_id)

    @staticmethod
    def parse(reader: StringIO) -> Optional[Packet]:
        version = int(reader.read(3), 2)
        type_id = int(reader.read(3), 2)
        data = None

        if type_id == 4:  # literal
            stack = []
            while len(stack) == 0 or stack[-1][0] != "0":
                stack.append(reader.read(5))
            data = int("".join(bit[1:] for bit in stack), 2)
        else:  # operator
            length_type = reader.read(1)
            subpackets = []
            if length_type == "0":
                subpacket_bits = int(reader.read(15), 2)
                bit_limit = reader.tell() + subpacket_bits
                while reader.tell() < bit_limit:
                    subpackets.append(Packet.parse(reader))
            else:
                subpacket_count = int(reader.read(11), 2)
                for _ in range(subpacket_count):
                    subpackets.append(Packet.parse(reader))
            data = subpackets

        return Packet(version, type_id, data)
