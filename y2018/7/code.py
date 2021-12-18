# Advent of code Year 2018 Day 7 solution
# Author = witzatom
# Date = December 2018

from __future__ import annotations
from itertools import groupby, count
import re

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


def from_string(data):
    input_re = re.compile(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
    for line in data.split("\n"):
        input_match = input_re.match(line)
        prerequisite = input_match.group(1)
        next_step = input_match.group(2)
        yield prerequisite, next_step


def part1(dependencies):
    requirements = {
        node: set(v for v, _ in reqs)
        for node, reqs in groupby(sorted(dependencies, key=lambda x: x[1]), key=lambda x: x[1])
    }
    to_do = set(
        step
        for dep in dependencies
        for step in dep
    )
    done = set()
    order = []
    while len(to_do) > 0:
        possible_steps = [
            step
            for step in sorted(to_do)
            if step not in requirements
               or done.issuperset(requirements[step])
        ]
        for step in possible_steps:
            done.add(step)
            to_do.remove(step)
            order.append(step)

    return "".join(order)


def part2(dependencies, worker_count, default_cost):
    def cost(letter):
        if letter is None:
            return 0
        return ord(letter[0]) - 64 + default_cost

    requirements = {
        node: set(v for v, _ in reqs)
        for node, reqs in groupby(sorted(dependencies, key=lambda x: x[1]), key=lambda x: x[1])
    }
    to_do = set(
        step
        for dep in dependencies
        for step in dep
    )
    in_progress = set()
    done = set()
    worker_assignments = {
        i: (0, None)
        for i in range(worker_count)
    }

    for time in count(0):
        # print(f"Time is {time}")
        finished_jobs = [
            (worker_id, assigned_job)
            for worker_id, (finished_time, assigned_job) in worker_assignments.items()
            if finished_time <= time and assigned_job is not None
        ]
        for worker_id, assigned_job in finished_jobs:
            worker_assignments[worker_id] = (0, None)
            # print(f"Freeing up worker {worker_id}")
            to_do.remove(assigned_job)
            in_progress.remove(assigned_job)
            done.add(assigned_job)
            # print(f"Finishing job {assigned_job}")
        free_workers = [
            idx
            for idx, (_, assignment) in worker_assignments.items()
            if assignment is None
        ]
        # print(f"Free workers are {free_workers}")
        if len(free_workers) == 0:
            continue
        possible_steps = [
            step
            for step in sorted(to_do.difference(in_progress))
            if step not in requirements
               or done.issuperset(requirements[step])
        ]
        # print(f"Possible steps are {possible_steps}")
        for worker_id, step in zip(free_workers, possible_steps):
            # print(f"Assigning worker {worker_id} to {step}")
            in_progress.add(step)
            worker_assignments[worker_id] = (time + cost(step), step)
        if len(to_do) == 0:
            return time


dependencies = set(from_string(input))
order = part1(dependencies)
time_to_finish = part2(dependencies, 5, 60)

print("Part One : " + str(order))

print("Part Two : " + str(time_to_finish))
