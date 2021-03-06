#!/usr/bin/env python

N = int(input())

ans = 0

# 和の公式
# c < 1
# 1 + c + c**2 + c**3 + .... = 1/(1 - c)
#
# コインが集まるまでの期待値について
# N = 3 の場合
# 一種類目を引く回数の期待値は
# - 1(必ず1回目で引けるから)
# 二種類目を引く回数の期待値は
# - 1(必ず一回は引く)
# - 1/3(2回目を引く確率は、1回目で一種類目を引くこと)
# - 1/9(3回目を引く確率は、1回目と2回目で一種類目を引くこと)
# - 和の公式より、1/(1 - 1/3) = 3/2
# 三種類目を引く回数の期待値は
# - 1(必ず一回は引く)
# - 2/3(2回目を引く確率は、1回目で一種類目か二種類目を引くこと)
# - 4/9(3回目を引く確率は、1回目と2回目で一種類目か二種類目を引くこと)
# - 和の公式より、1/(1 - 2/3) = 3
# つまりN=3のときのコインが集まるまでの回数の期待値は、
# 1 + 3/2 + 3 = 11/2 回
#
# あとは、これを一般化して式変形をしていくと以下の式が得られる
# N * (1 + 1/2 + 1/3 + ..... + 1/(N-2) + 1/(N-1) + 1/N)

for i in range(1, N + 1):
    ans += 1 / i

print(N*ans)
