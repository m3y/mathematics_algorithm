#!/usr/bin/env python

'''
014 - Factorization
'''


def factorization(n):
    res = []
    flg = True
    limit = int(n ** 0.5)
    for i in range(2, limit+1):
        if n % i == 0:
            flg = False
            res.append(i)
            res += factorization(int(n/i))
            break
    if flg:
        res.append(n)
    return res


def main(n: int):
    res = factorization(n)
    print(" ".join([str(i) for i in res]))


if __name__ == '__main__':
    N = int(input())
    main(N)
