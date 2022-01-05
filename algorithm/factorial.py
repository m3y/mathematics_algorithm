def fact(n: int) -> int:
    return n * fact(n - 1) if n > 1 else 1


def main(n: int):
    print(fact(n))


if __name__ == "__main__":
    N = 10
    main(N)
