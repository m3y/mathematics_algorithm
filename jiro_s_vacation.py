#!/usr/bin/env python

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

ans = 0.0

i = 0
while i < N:
    ans += (A[i] * 1 / 3) + (B[i] * 2 / 3)
    i += 1

print(ans)
