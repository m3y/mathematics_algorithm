from typing import List

from .util import multi


def gcd(l: int, r: int) -> int:
    if l == r:
        return l

    if l < r:
        l, r = r, l

    return r if l % r == 0 else gcd(r, l % r)


def main(l: List[int]):
    print(gcd(l[0], l[1]))
    print(multi(gcd, l[0], l[1:]))


if __name__ == "__main__":
    main([40, 40, 60, 80, 100, 20])
