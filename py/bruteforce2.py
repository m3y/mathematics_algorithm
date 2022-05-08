N, S = [int(i) for i in input().split()]
A = [int(i) for i in input().split()]


def judge(N, S, A):
    # 1 << N は、2のN乗 1 << 5 => 10000
    # range(0, 1<<N) は、0から(2のN乗-1)までのループ
    for i in range(0, 1 << N):
        ps = 0
        for j in range(0, N):
            # i & (1 << j) この結果が0じゃないとき、j + 1桁目のビットが立っている
            # 最初の桁は1とする
            if (i & (1 << j)) != 0:
                ps += A[j]
        if ps == S:
            return True
    return False


if judge(N, S, A):
    print("Yes")
else:
    print("No")
