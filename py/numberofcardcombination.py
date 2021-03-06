from typing import List

from algorithm.combination import combination as c


def main(l: List[int]):
    r = y = b = 0
    for i in l:
        if i == 1:
            r += 1
        elif i == 2:
            y += 1
        elif i == 3:
            b += 1
    # print(c(r, 2) + c(y, 2) + c(b, 2))

    # c(r, 2) を式展開すると、r * (r - 1) // 2 となる
    # この場合、factorial計算などをしなくて良くなる
    print(r * (r - 1) // 2 + y * (y - 1) // 2 + b * (b - 1) // 2)


if __name__ == "__main__":
    main([1, 3, 1, 2, 2, 3, 2, 1, 3, 2, 1])
