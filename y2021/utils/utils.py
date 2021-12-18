from functools import wraps
from itertools import islice
from math import gcd
from time import time
from typing import Tuple, Generator, Iterable, TypeVar

T = TypeVar('T')

def window(seq: Iterable[T], n: int=2): # -> Generator[Tuple[T, ...]]:
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def timed(f):
    @wraps(f)
    def wrapper(*args, **kwds):
        start = int(round(time() * 1000))
        result = f(*args, **kwds)
        elapsed = int(round(time() * 1000)) - start
        print("%s took %dms to finish" % (f.__name__, elapsed))
        return result

    return wrapper


def lcm(a, b):
    """
    Lowest common multiple
    """
    return abs(a * b) // gcd(a, b)


def egcd(a: int, b: int) -> Tuple[int, int, int]:
    """
    return (g, x, y) such that a*x + b*y = g = gcd(a, b)
    """
    x0, x1, y0, y1 = 0, 1, 1, 0
    while a != 0:
        (q, a), b = divmod(b, a), a
        y0, y1 = y1, y0 - q * y1
        x0, x1 = x1, x0 - q * x1
    return b, x0, y0
