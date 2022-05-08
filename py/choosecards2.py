#!/usr/bin/env python


def main(N, A):
    ans = 0
    for i in range(0, N):
        for j in range(i+1, N):
            if A[i] + A[j] > 997:
                continue
            for k in range(j+1, N):
                if A[i] + A[j] + A[k] > 998:
                    continue
                for l in range(k+1, N):
                    if A[i] + A[j] + A[k] + A[l] > 999:
                        continue
                    for m in range(l+1, N):
                        if A[i] + A[j] + A[k] + A[l] + A[m] == 1000:
                            ans += 1
    print(ans)


if __name__ == '__main__':
    N = int(input())
    A = [int(i) for i in input().split()]
    main(N, A)
