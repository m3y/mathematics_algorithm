N = int(input())

result = []
N2 = int(N ** 0.5) # 先にルートを取っておくことで時間短縮
for i in range(1, N2 + 1):
    if N % i != 0:
        continue
    result.append(i)
    print(i)
    if i != N / i:
        result.append(int(N/i))
        print(int(N/i))
