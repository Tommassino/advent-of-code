from functools import wraps
from itertools import islice
from time import time


def window(seq, n=2):
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
