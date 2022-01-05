from typing import Dict, List

from algorithm.combination import combination as c


def count_number(a: List[int]) -> Dict[int, int]:
    d = {}
    for i in a:
        if i not in d:
            d[i] = 1
        else:
            d[i] += 1
    return d


def main(params: List[int]):
    d = count_number(params)
    r = 0
    for i in range(1, 50000):
        j = 100000 - i
        if i in d and j in d:
            r += d[i] * d[j]

    if 50000 in d:
        r += c(d[50000], 2)
    print(r)


if __name__ == '__main__':
    main([1, 2, 3, 4, 4, 49999, 50000, 50000, 50001, 99997, 99997, 99998, 99999])
