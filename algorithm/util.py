from typing import List


def multi(f, head: int, tail: List[int]) -> int:
    if len(tail) < 0:
        raise ValueError("error")
    elif len(tail) == 1:
        return f(head, tail[0])
    else:
        return multi(f, f(head, tail[0]), tail[1:])
