from typing import List

from .gcd import gcd
from .util import multi


def lcm(l: int, r: int) -> int:
    return int(l * r / gcd(l, r))


def main(l: List[int]):
    print(lcm(l[0], l[1]))
    print(multi(lcm, l[0], l[1:]))


if __name__ == "__main__":
    main([40, 40, 60, 80, 100, 20])
