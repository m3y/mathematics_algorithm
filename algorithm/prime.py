def is_prime(n: int) -> bool:
    if n <= 1:
        return False

    if n == 2:
        return False

    flg = True
    for i in range(2, n):
        if i * i > n:
            break
        elif n % i == 0:
            flg = False
    return flg


def main(n: int):
    result = [i for i in range(1, n + 1) if is_prime(i)]
    print(result)
    print(len(result))


if __name__ == "__main__":
    N = 10000
    main(N)
