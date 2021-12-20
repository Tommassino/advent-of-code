# Advent of code Year 2018 Day 4 solution
# Author = witzatom
# Date = December 2018

import re
from itertools import groupby
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import List, Tuple
import numpy as np

with open((__file__.rstrip("code.py") + "input.txt"), 'r') as input_file:
    input = input_file.read()


class Action(Enum):
    START = "START"
    FALLS_ASLEEP = "FALLS_ASLEEP"
    WAKES_UP = "WAKES_UP"


@dataclass
class Shift:
    guard_id: int
    actions: List[Tuple[datetime, Action]]

    def sleep_array(self):
        minutes = np.zeros(60)
        for i in range(len(self.actions) - 1):
            if self.actions[i][1] != Action.FALLS_ASLEEP:
                continue

            minutes[self.actions[i][0].minute:self.actions[i + 1][0].minute] += 1
        return minutes


def from_string(data: str) -> List[Shift]:
    guard_re = re.compile("\\[1518-(\d+)-(\d+) (\d+):(\d+)] Guard #(\d+) begins shift")
    sleep_re = re.compile("\\[1518-(\d+)-(\d+) (\d+):(\d+)] falls asleep")
    awake_re = re.compile("\\[1518-(\d+)-(\d+) (\d+):(\d+)] wakes up")

    def make_time(match):
        month = int(match.group(1))
        day = int(match.group(2))
        hour = int(match.group(3))
        minute = int(match.group(4))
        return datetime(2000, month, day, hour, minute)

    guard_id = None
    actions = list()
    shifts = list()

    for line in sorted(data.split("\n")):
        guard_match = guard_re.match(line)
        if guard_match:
            if guard_id is not None:
                shifts.append(Shift(guard_id, actions))
                actions = list()
            guard_id = int(guard_match.group(5))
            actions.append((make_time(guard_match), Action.START))
            continue
        sleep_match = sleep_re.match(line)
        if sleep_match:
            if actions[-1][1] == Action.FALLS_ASLEEP:
                continue
            actions.append((make_time(sleep_match), Action.FALLS_ASLEEP))
            continue
        awake_match = awake_re.match(line)
        if awake_match:
            if actions[-1][1] == Action.WAKES_UP:
                continue
            actions.append((make_time(awake_match), Action.WAKES_UP))
            continue
    if guard_id is not None:
        shifts.append(Shift(guard_id, actions))
    return shifts


shifts = sorted(from_string(input), key=lambda x: x.guard_id)
guard_info = []

for guard_id, guard_shifts in groupby(shifts, key=lambda x: x.guard_id):
    minutes = np.zeros(60)
    for shift in guard_shifts:
        minutes += shift.sleep_array()
    max_minute = np.argmax(minutes)
    guard_info.append((guard_id, np.sum(minutes), max_minute, minutes[max_minute]))

guard_id, _, minute, _ = max(guard_info, key=lambda x: x[1])

print("Part One : " + str(guard_id * minute))

guard_id, _, minute, _ = max(guard_info, key=lambda x: x[3])

print("Part Two : " + str(guard_id * minute))
