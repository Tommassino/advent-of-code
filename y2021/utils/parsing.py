from typing import List, Callable
from dataclasses import dataclass
import inspect
import re

def lines(input: str) -> List[str]:
    return input.split("\n")

def numbers(input: List[str]) -> List[int]:
    return list(map(int, input))

def re_group_parser(regex: str, datacls: dataclass) -> Callable[[str], dataclass]:
    pattern = re.compile(regex)
    spec = inspect.getfullargspec(datacls.__init__)

    def inner(string) -> dataclass:
        match = pattern.match(string)
        if match:
            arguments = {
                name: spec.annotations[name](value)
                for name, value in zip(spec.args[1:], match.groups())
            }
            return datacls(**arguments)
        else:
            return None

    return inner
