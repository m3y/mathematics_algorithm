from typing import List


def main(params: List[int]):
    a = b = c = d = 0
    for i in params:
        if i == 100:
            a += 1
        elif i == 200:
            b += 1
        elif i == 300:
            c += 1
        elif i == 400:
            d += 1

    print(a * d + b * c)


if __name__ == "__main__":
    main([100, 200, 300, 400, 100, 200, 300, 400, 100, 300, 400, 100, 400])
