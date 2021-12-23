# Advent of code Year 2021 Day 23 solution
# Author = witzatom
# Date = December 2021
from __future__ import annotations
from typing import Generator, NamedTuple, Tuple
from collections import defaultdict
from heapq import heappush, heappop
from y2021.utils import timed


class State(NamedTuple):
    positions: Tuple
    """
    The positions are encoded as a tuple of size width
    idx -> tuple[pods]
    """
    depth: int

    costs = {"A": 1, "B": 10, "C": 100, "D": 1000}

    @property
    def width(self):
        return len(self.positions)

    def is_room(self, x: int) -> int:
        return x % 2 == 0 and x != 0 and x != self.width - 1

    def is_occupied(self, x: int) -> bool:
        if self.is_room(x):
            return False
        return len(self.positions[x])

    @staticmethod
    def parse(input: str) -> State:
        positions = defaultdict(list)
        for line in input.splitlines()[1:-1]:
            for (x, char) in enumerate(line):
                if char in {"A", "B", "C", "D"}:
                    positions[x - 1].append(char)
        width = len(input.splitlines()[0]) - 2
        depth = max(map(len, positions.values()))
        return State(tuple(
            tuple(positions[i]) if i in positions else tuple() 
            for i in range(width)
        ), depth)

    @staticmethod
    def target_room(pod: str) -> int:
        return (ord(pod) - ord("A") + 1) * 2

    def move_cost(self, start: int, end: int) -> int:
        """
        >>> State(((), (), (), (), (), (), ('B', 'B'), (), (), (), ()), 2).move_cost(6, 3)
        40

        >>> State((('D'), (), (), (), (), (), (), (), (), (), ()), 2).move_cost(0, 8)
        10000

        >>> State(((), (), 
        ... ('B', 'D', 'D', 'A'), (), 
        ... ('C', 'C', 'B', 'D'), (), 
        ... ('B', 'B', 'A', 'C'), (), 
        ... ('D', 'A', 'C', 'A'), (), ()), 4).move_cost(8, 10)
        3000
        """

        # x moves
        moves = abs(end - start)
        # y moves
        if self.is_room(start):
            moves += self.depth - len(self.positions[start]) + 1
        if self.is_room(end):
            moves += self.depth - len(self.positions[end])
        pod = self.positions[start][0]
        return moves * self.costs[pod]

    def possible_moves(self, position: int) -> Generator[int, None, None]:
        """
        Cannot move in hall
        >>> set(State(((), (), 
        ... ('B', 'D', 'D', 'A'), (), 
        ... ('C', 'C', 'B', 'D'), ('A'), 
        ... ('B', 'B', 'A', 'C'), (), 
        ... ('D', 'A', 'C', 'A'), (), ()), 4).possible_moves(5))
        set()
        
        Can move from room to hall
        >>> set(State(((), (), 
        ... ('B', 'D', 'D', 'A'), (), 
        ... ('C', 'C', 'B', 'D'), (), 
        ... ('B', 'B', 'A', 'C'), (), 
        ... ('D', 'A', 'C', 'A'), (), ()), 4).possible_moves(8))
        {0, 1, 3, 5, 7, 9, 10}

        >>> set(State(((), (), 
        ... ('B', 'A'), (), 
        ... ('C', 'D'), (), 
        ... ('B', 'C'), (), 
        ... ('D', 'A'), (), ()), 2).possible_moves(4))
        {0, 1, 3, 5, 7, 9, 10}
        
        Move right into target if possible
        >>> set(State(((), (), 
        ... ('B', 'A'), ('B'), 
        ... ('C', 'D'), (), 
        ... ('C'), (), 
        ... ('D', 'A'), (), ()), 2).possible_moves(4))
        {6}
        
        Cannot move over other pods
        >>> set(State(((), (), 
        ... ('B', 'D', 'D', 'A'), (), 
        ... ('C', 'C', 'B', 'D'), ('A'), 
        ... ('B', 'B', 'A', 'C'), (), 
        ... ('D', 'A', 'C', 'A'), (), ()), 4).possible_moves(8))
        {9, 10, 7}
        
        Does not move if its done
        >>> set(State(((), (), 
        ... ('A', 'A', 'A', 'A'), (), 
        ... ('B', 'B', 'B', 'B'), (), 
        ... ('C', 'C', 'C', 'C'), (), 
        ... ('D', 'D', 'D', 'D'), (), ()), 4).possible_moves(8))
        set()
        """

        pod = self.positions[position][0]
        pods_room = self.target_room(pod)
        room_possible = len(set(self.positions[pods_room]).difference(pod)) == 0

        # if its already in the right place never move
        if position == pods_room and room_possible:
            return
        # if it can move to its room
        if room_possible:
            diff = -1 if pods_room < position else 1
            x = position
            while x != pods_room:
                x += diff
                if self.is_room(x):
                    continue
                if self.is_occupied(x):
                    break
            if x == pods_room:
                yield x
                return

        # if it is in the hall
        if position % 2 != 0 or position == 0 or position == self.width - 1:
            return
        # left hall
        for x in range(position - 1, -1, -1):
            if x == pods_room and room_possible:
                yield x
            if self.is_room(x):
                continue
            if self.is_occupied(x):
                break
            yield x
        # right hall
        for x in range(position + 1, self.width):
            if x == pods_room and room_possible:
                yield x
            if self.is_room(x):
                continue
            if self.is_occupied(x):
                break
            yield x

    def finished(self) -> bool:
        """
        >>> State(((), (), 
        ... ('A', 'A', 'D', 'A'), (), 
        ... ('B', 'B', 'B', 'D'), (), 
        ... ('C', 'C', 'A', 'C'), (), 
        ... ('D', 'D', 'C', 'A'), (), ()), 4).finished()
        False

        >>> State(((), (), 
        ... ('A', 'A', 'A', 'A'), (), 
        ... ('B', 'B', 'B', 'B'), (), 
        ... ('C', 'C', 'C', 'C'), (), 
        ... ('D', 'D', 'D', 'D'), (), ()), 4).finished()
        True
        """

        for position, pods in enumerate(self.positions):
            for pod in pods:
                if position != self.target_room(pod):
                    return False
        return True

    def move_pod(self, from_idx: int, to_idx: int) -> Tuple[int, State]:
        """
        >>> State(((), (), ('B', 'A'), (), ('C', 'D'), (), ('B', 'C'), (), ('D', 'A'), (), ()), 2).move_pod(6, 3)
        (40, State(positions=((), (), ('B', 'A'), ('B',), ('C', 'D'), (), ('C',), (), ('D', 'A'), (), ()), depth=2))

        >>> State(((), (), 
        ... ('B', 'D', 'D', 'A'), (), 
        ... ('C', 'C', 'B', 'D'), (), 
        ... ('B', 'B', 'A', 'C'), (), 
        ... ('D', 'A', 'C', 'A'), (), ()), 4).move_pod(8, 10)
        (3000, State(positions=((), (), ('B', 'D', 'D', 'A'), (), ('C', 'C', 'B', 'D'), (), ('B', 'B', 'A', 'C'), (), ('A', 'C', 'A'), (), ('D',)), depth=4))
        """
        cost = self.move_cost(from_idx, to_idx)
        positions = list(self.positions)
        pod = positions[from_idx][0]
        positions[from_idx] = positions[from_idx][1:]
        positions[to_idx] = list(positions[to_idx])
        positions[to_idx].insert(0, pod)
        positions[to_idx] = tuple(positions[to_idx])
        return cost, State(tuple(positions), self.depth)

    def all_moves(self) -> Generator[Tuple[int, State], None, None]:
        for start_id, pods in enumerate(self.positions):
            if len(pods) == 0:
                continue
            for end_id in self.possible_moves(start_id):
                cost, new_state = self.move_pod(start_id, end_id)
                yield cost, new_state


def order_pods(start: State) -> int:
    heap = [(0, start)]
    visited = set()
    milestone = 1000

    while heap:
        distance, current = heappop(heap)
        if current in visited:
            continue
        if current.finished():
            return distance, current
        if distance >= milestone:
            print(distance, current)
            milestone += 1000
        visited.add(current)
        for cost, next in current.all_moves():
            heappush(heap, (distance + cost, next))



def run(puzzle_input):
    print(f"Part One : {part_one(puzzle_input)}")
    print(f"Part Two : {part_two(puzzle_input)}")


@timed
def part_one(puzzle_input: str):
    start = State.parse(puzzle_input)
    finished = order_pods(start)
    return finished[0]


@timed
def part_two(puzzle_input: str):
    puzzle_input = puzzle_input.splitlines()
    puzzle_input.insert(3, "  #D#B#A#C#")
    puzzle_input.insert(3, "  #D#C#B#A#")
    start = State.parse("\n".join(puzzle_input))
    finished = order_pods(start)
    return finished[0]
