from .factorial import fact


def combination(n: int, r: int) -> int:
    return int(fact(n) / (fact(r) * fact(n - r)))


def main(n: int, r: int):
    print(combination(n, r))


if __name__ == "__main__":
    main(8, 5)
