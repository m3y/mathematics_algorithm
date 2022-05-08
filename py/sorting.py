#!/usr/bin/env python

'''
Merge sort
'''


def mergesort(a):
    if len(a) == 1:
        return a

    m = len(a) // 2
    return merge(mergesort(a[:m]), mergesort(a[m:]))


def merge(l, r):
    c1 = 0
    c2 = 0
    c = []
    while (c1 < len(l) or c2 < len(r)):
        if c1 == len(l):
            c.append(r[c2])
            c2 += 1
        elif c2 == len(r):
            c.append(l[c1])
            c1 += 1
        else:
            if l[c1] <= r[c2]:
                c.append(l[c1])
                c1 += 1
            else:
                c.append(r[c2])
                c2 += 1
    return c


N = int(input())
A = [int(i) for i in input().split()]
ans = mergesort(A)
# ans = sorted(A)
print(*ans)
