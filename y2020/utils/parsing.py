from dataclasses import dataclass
import inspect
import re


def re_group_parser(regex: str, datacls: dataclass):
    pattern = re.compile(regex)
    spec = inspect.getfullargspec(datacls.__init__)

    def inner(string):
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
